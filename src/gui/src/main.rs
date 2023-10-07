#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::{Arc, mpsc, RwLock};
use std::sync::mpsc::{Receiver, Sender};

use serde::{Deserialize, Serialize};

use nesemu::bus::Bus;
use nesemu::memory::CpuMemory;
use nesemu_cpu::cpu::{CPU, CpuDebugInfo, FlagData};

mod app;
mod web;
mod native;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    native::run().unwrap();
    #[cfg(target_arch = "wasm32")]
    web::run();
}

pub struct Nes {
    cpu: CPU<Bus<CpuMemory>>,
    ram: Arc<RwLock<CpuMemory>>,
    bus: Arc<RwLock<Bus<CpuMemory>>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EmulatorState {
    ram: Arc<RwLock<CpuMemory>>,
    flags: FlagData,
}

impl Nes {
    fn get_main_ram(&self) -> [u8; 2048] {
        *self.ram.read().unwrap().main_ram()
    }

    pub fn get_main_ram_mirror(&self) -> [u8; 6144] {
        *self.ram.read().unwrap().main_ram_mirror()
    }

    pub fn get_ppu_registers(&self) -> [u8; 8] {
        *self.ram.read().unwrap().ppu_registers()
    }

    pub fn get_ppu_mirrors(&self) -> [u8; 8184] {
        *self.ram.read().unwrap().ppu_mirrors()
    }

    pub fn get_apu_io_registers(&self) -> [u8; 24] {
        *self.ram.read().unwrap().apu_io_registers()
    }

    pub fn get_apu_io_expansion(&self) -> [u8; 8] {
        *self.ram.read().unwrap().apu_io_expansion()
    }

    pub fn get_cartridge_space(&self) -> [u8; 49120] {
        *self.ram.read().unwrap().cartridge_space()
    }
}

impl Nes {
    fn get_cpu_flags(&self) -> FlagData {
        self.cpu.get_flag_data()
    }

    fn get_cpu_debug_info(&self) -> CpuDebugInfo {
        self.cpu.get_cpu_debug_info()
    }
}

pub enum EmulatorMessage {
    Update,
    Terminate,
}

pub enum GuiMessage {
    UpdateUI,
    Terminate,
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


/*

// For if we ever need to spawn this specifically.
// Dead code for now.
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

