#![cfg(not(target_arch = "wasm32"))]

use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Duration;

use nesemu::bus::Bus;
use nesemu::memory::CpuMemory;
use nesemu::Nes;
use nesemu_core::{Read, ReadFn, Write, WriteFn};
use nesemu_cpu::cpu::CPU;

use crate::app::NesemuGui;
use crate::{create_channels, EmulatorMessage, GuiMessage};

pub fn run() {
    env_logger::init();

    let mut ram = CpuMemory::default();

    let read_closure: Box<dyn ReadFn> =
        Box::new(move |addr, read_only| -> u8 { ram.read(addr, read_only) });

    let write_closure: Box<dyn WriteFn> = Box::new(move |addr, data| {
        ram.write(addr, data);
    });

    let mut bus = Bus::new();
    let cpu = CPU::new(
        Box::new(move |addr, read_only| bus.read(addr, read_only)),
        Box::new(move |addr, data| bus.write(addr, data)),
    );

    let mut nes = Nes { ram, cpu, bus };
    nes.load_rom("nestest.nes").expect("TODO: panic message");
    nes.cpu.reset();

    let nes_ref = Arc::new(RwLock::new(nes));

    // Set up communication channels between emulator and GUI
    let (emulator_tx, emulator_rx, gui_tx, gui_rx) = create_channels();

    spawn_emulator_thread(
        nes_ref.clone(),
        emulator_tx,
        emulator_rx,
        read_closure,
        write_closure,
    );

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
    emulator_tx: mpsc::Sender<EmulatorMessage>,
    _gui_tx: mpsc::Receiver<GuiMessage>,
    read_closure: Box<dyn ReadFn>,
    write_closure: Box<dyn WriteFn>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));

        let mut emulator_lock = emulator.write().unwrap();
        emulator_lock.bus.set_read(Arc::clone(read_closure));
        emulator_lock.bus.set_write(write_closure);

        emulator_lock.cpu.clock();

        emulator_tx
            .send(EmulatorMessage::Update)
            .unwrap_or_else(|_| log::info!("sending between threads failed!!!!!!"));
    })
}
