#![cfg(not(target_arch = "wasm32"))]

use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use nesemu::bus::Bus;
use nesemu::memory::CpuMemory;
use nesemu::Nes;
use nesemu_cpu::cpu::CPU;

use crate::{create_channels, EmulatorMessage, GuiMessage};
use crate::app::NesemuGui;

pub fn run() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let ram = Arc::new(RwLock::new(CpuMemory::default()));
    let bus = Arc::new(RwLock::new(Bus::new(ram.clone())));
    let cpu = CPU::new(bus.clone());

    let mut nes = Nes {
        ram: ram.clone(),
        cpu,
        bus: bus.clone(),
    };
    nes.load_rom("nestest.nes").expect("TODO: panic message");
    nes.cpu.reset();

    let nes_ref = Arc::new(RwLock::new(nes));


    // Set up communication channels between emulator and GUI
    let (emulator_tx, emulator_rx, gui_tx, gui_rx) = create_channels();

    spawn_emulator_thread(nes_ref.clone(), emulator_tx, emulator_rx);

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "nesemu",
        native_options,
        Box::new(|cc| {
            let ctx = cc.egui_ctx.clone();
            thread::spawn(move || {
                loop {
                    if let Ok(update) = gui_rx.try_recv() {
                        match update {
                            EmulatorMessage::Update => {
                                ctx.request_repaint();
                            }
                            _ => {
                                println!("emu said stop!!!")
                            }
                        }
                    }
                    // just a small sleep for now.. we'll get something global
                    // later. Just help the thing slow down a little.
                    thread::sleep(Duration::from_millis(10))
                }
            });
            Box::new(NesemuGui::new(cc, gui_tx, nes_ref))
        }),
    )
        .expect("Failed to start GUI")
}

fn spawn_emulator_thread(
    emulator: Arc<RwLock<Nes>>,
    emulator_tx: Sender<EmulatorMessage>,
    _gui_tx: Receiver<GuiMessage>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        let mut lock = emulator.write().unwrap();
        lock.cpu.clock();
        emulator_tx
            .send(EmulatorMessage::Update)
            .unwrap_or_else(|_| log::info!("sending between threads failed!!!!!!"));
    })
}
