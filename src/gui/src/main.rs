#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use eframe::App;
use serde::{Deserialize, Serialize};

use nesemu::bus::Bus;
use nesemu::memory::CpuMemory;
use nesemu_cpu::cpu::{CPU, CpuDebugInfo, FlagData};

use crate::app::NesemuGui;

mod app;

pub struct Nes {
    ram: Arc<Mutex<CpuMemory>>,
    cpu: CPU<Bus<CpuMemory>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EmulatorState {
    ram: Arc<Mutex<CpuMemory>>,
    flags: FlagData,
}

impl EmulatorState {
    fn get_main_ram(&self) -> [u8; 2048] {
        *self.ram.lock().unwrap().main_ram()
    }

    pub fn get_main_ram_mirror(&self) -> [u8; 6144] {
        *self.ram.lock().unwrap().main_ram_mirror()
    }

    pub fn get_ppu_registers(&self) -> [u8; 8] {
        *self.ram.lock().unwrap().ppu_registers()
    }

    pub fn get_ppu_mirrors(&self) -> [u8; 8184] {
        *self.ram.lock().unwrap().ppu_mirrors()
    }

    pub fn get_apu_io_registers(&self) -> [u8; 24] {
        *self.ram.lock().unwrap().apu_io_registers()
    }

    pub fn get_apu_io_expansion(&self) -> [u8; 8] {
        *self.ram.lock().unwrap().apu_io_expansion()
    }

    pub fn get_cartridge_space(&self) -> [u8; 49120] {
        *self.ram.lock().unwrap().cartridge_space()
    }
}

impl Nes {
    fn generate_state(&self) -> EmulatorState {
        EmulatorState {
            ram: self.ram.clone(),
            flags: self.get_cpu_flags(),
        }
    }

    fn get_cpu_flags(&self) -> FlagData {
        self.cpu.get_flag_data()
    }

    fn get_cpu_debug_info(&self) -> CpuDebugInfo {
        self.cpu.get_cpu_debug_info()
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let ram = Arc::new(Mutex::new(CpuMemory::default()));
    let mut bus: Bus<CpuMemory> = Bus::new();
    let mut cpu: CPU<Bus<CpuMemory>> = CPU::default();

    bus.connect_ram(Arc::clone(&ram));
    cpu.connect_bus(Box::new(bus));

    cpu.reset();

    let nes_ref = Arc::new(Mutex::new(Nes { ram, cpu }));

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
        Box::new(
            |cc| {
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
                    }
                });
                Box::new(NesemuGui::new(cc, gui_tx, nes_ref))
            }),
    )
    //
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(NesemuGui::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

pub enum EmulatorMessage {
    UpdateState(EmulatorState),
    Update,
    // Example: Send emulator state updates
    Terminate,                  // Example: Terminate the emulator thread
}

pub enum GuiMessage {
    UpdateUI,
    // Example: Send data to update the GUI
    Terminate, // Example: Terminate the GUI thread
}

fn create_channels() -> (
    Sender<EmulatorMessage>,
    Receiver<GuiMessage>,
    Sender<GuiMessage>,
    Receiver<EmulatorMessage>,
) {
    let (emulator_tx, gui_rx) = mpsc::channel();
    let (gui_tx, emulator_rx) = mpsc::channel();
    (emulator_tx, emulator_rx, gui_tx, gui_rx)
}

fn spawn_emulator_thread(
    emulator: Arc<Mutex<Nes>>,
    emulator_tx: Sender<EmulatorMessage>,
    _gui_tx: Receiver<GuiMessage>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(200));
            let mut lock = emulator.lock().unwrap();
            lock.cpu.clock();
            emulator_tx
                .send(EmulatorMessage::Update)
                .unwrap_or_else(|_| log::info!("sending between threads failed!!!!!!"));
        }
    })
}
/*
fn spawn_gui_thread(
    gui: Arc<Mutex<NesemuGui>>,
    gui_rx: Sender<GuiMessage>,
    emulator_rx: Receiver<EmulatorMessage>,
) -> thread::JoinHandle<()> {
    return thread::spawn(move || {
        std::panic::set_hook(Box::new(|panic_info| {
            eprintln!("Thread panicked: {:?}", panic_info);
        }));
        // GUI initialization code
        let native_options = eframe::NativeOptions {
            initial_window_size: Some([400.0, 300.0].into()),
            min_window_size: Some([300.0, 220.0].into()),
            ..Default::default()
        };
        eframe::run_native(
            "nesemu",
            native_options,
            Box::new(|cc| Box::new(NesemuGui::new(cc, gui_rx, emulator_rx, nes_ref))),
        ).expect("TODO: panic message");
        // GUI event loop
        loop {
            // Handle messages from the emulator thread
            match emulator_rx.try_recv() {
                Ok(emulator_message) => {
                    match emulator_message {
                        EmulatorMessage::UpdateState(emulator_state) => {
                            // Process the emulator state and update the GUI accordingly
                            let mut gui = gui.lock().unwrap();
                            gui.request_repaint(emulator_state); // Replace with your GUI update logic
                        }
                        EmulatorMessage::Terminate => {
                            // Terminate the GUI thread gracefully
                            break;
                        }
                    }
                }
                Err(_) => {
                    // No messages from the emulator, continue processing other GUI events
                }
            }
        }
    });
}
*/

