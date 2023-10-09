#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod app;
mod native;
mod web;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    native::run();
    #[cfg(target_arch = "wasm32")]
    web::run();
}

pub enum EmulatorMessage {
    Update,
    Terminate,
}

pub enum GuiMessage {
    UpdateUI,
    Terminate,
}

pub fn create_channels() -> (
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
