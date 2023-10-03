use std::sync::mpsc::{Receiver, Sender};

use egui::{CentralPanel, Grid, ScrollArea, Ui};

use crate::{EmulatorMessage, EmulatorState, GuiMessage};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct NesemuGui {
    state: EmulatorState,
    sender: Sender<GuiMessage>,
    receiver: Receiver<EmulatorMessage>
}

//impl Default for NesemuGui {
//    fn default() -> Self {
//        Self { ram: ram.clone() }
//    }
//}

impl NesemuGui {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, gui_tx: Sender<GuiMessage>, gui_rx: Receiver<EmulatorMessage>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        NesemuGui {
            state: Default::default(),
            sender: gui_tx,
            receiver: gui_rx,
        }
    }
}

impl eframe::App for NesemuGui {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        if let Ok(update) = self.receiver.recv() {
            match update {
                EmulatorMessage::UpdateState(new_state) => {
                    self.state = new_state
                }
                EmulatorMessage::Terminate => {
                    println!("emu said stop!!!")
                }
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            create_ram_panel(ui, "Work RAM", self.state.ram.lock().unwrap().main_ram());
            create_ram_panel(ui, "PPU Registers", self.state.ram.lock().unwrap().ppu_registers());
            create_ram_panel(ui, "APU/IO Registers", self.state.ram.lock().unwrap().apu_io_registers());
            create_ram_panel(ui, "Cartridge Space", self.state.ram.lock().unwrap().cartridge_space());
        });
    }
    // Called by the frame work to save state before shutdown.
    //fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //    eframe::set_value(storage, eframe::APP_KEY, self);
    //}
}

fn create_ram_panel<T: std::fmt::Debug>(ui: &mut Ui, title: &str, array: &[T]) {
    ui.heading(title);
    ui.separator();

    // Make a scrollable area for the array grid.
    ui.push_id(title, |ui| {
        ScrollArea::vertical().max_height(80.).show(ui, |ui| {
            // Display the array as a grid.
            Grid::new(title).striped(true).show(ui, |ui| {
                for (i, item) in array.iter().enumerate() {
                    if i % 8 == 0 {
                        // Display the index of the first element of the line.
                        ui.monospace(format!("{:04X}:", i));
                    }

                    // Display the value of the element.
                    ui.monospace(format!("{:?}", item));

                    if i % 8 == 7 {
                        // Add a newline for every 8 elements.
                        ui.end_row();
                    }
                }
            });
        })
    });
    ui.add_space(16.);
}
