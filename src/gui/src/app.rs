use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

use egui::{CentralPanel, Grid, ScrollArea, Ui};

use nesemu_cpu::cpu::{CpuDebugInfo, FlagData};

use crate::{EmulatorState, GuiMessage, Nes};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct NesemuGui {
    state: EmulatorState,
    sender: Sender<GuiMessage>,
    nes_ref: Arc<Mutex<Nes>>,
}

//impl Default for NesemuGui {
//    fn default() -> Self {
//        Self { ram: ram.clone() }
//    }
//}

impl NesemuGui {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, gui_tx: Sender<GuiMessage>, nes_ref: Arc<Mutex<Nes>>) -> Self {
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
            nes_ref,
        }
    }
}

impl eframe::App for NesemuGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            create_ram_panel(ui, "Work RAM", self.nes_ref.lock().unwrap().ram.lock().unwrap().main_ram());
            create_ram_panel(ui, "PPU Registers", self.nes_ref.lock().unwrap().ram.lock().unwrap().ppu_registers());
            create_ram_panel(ui, "APU/IO Registers", self.nes_ref.lock().unwrap().ram.lock().unwrap().apu_io_registers());
            create_ram_panel(ui, "Cartridge Space", self.nes_ref.lock().unwrap().ram.lock().unwrap().cartridge_space());
            create_cpu_flag_panel(ui, self.nes_ref.lock().unwrap().get_cpu_flags());
            create_cpu_debug_panel(ui, self.nes_ref.lock().unwrap().get_cpu_debug_info());
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
                    if i % 32 == 0 {
                        // Display the index of the first element of the line.
                        ui.monospace(format!("{:04X}:", i));
                    }

                    // Display the value of the element.
                    ui.monospace(format!("{:?}", item));

                    if i % 32 == 31 {
                        // Add a newline for every 8 elements.
                        ui.end_row();
                    }
                }
            });
        })
    });
    ui.add_space(16.);
}

fn create_cpu_flag_panel(ui: &mut Ui, flags: FlagData) {
    ui.heading("Flags");
    ui.separator();

    // Make a scrollable area for the array grid.
    ui.push_id("cpu-flags", |ui| {
        Grid::new("cpu flags").striped(true).show(ui, |ui| {
            ui.label("C:");
            ui.label(format!("{}", flags.C));
            ui.end_row();

            ui.label("Z:");
            ui.label(format!("{}", flags.Z));
            ui.end_row();

            ui.label("I:");
            ui.label(format!("{}", flags.I));
            ui.end_row();

            ui.label("D:");
            ui.label(format!("{}", flags.D));
            ui.end_row();

            ui.label("B:");
            ui.label(format!("{}", flags.B));
            ui.end_row();

            ui.label("U:");
            ui.label(format!("{}", flags.U));
            ui.end_row();

            ui.label("V:");
            ui.label(format!("{}", flags.V));
            ui.end_row();

            ui.label("N:");
            ui.label(format!("{}", flags.N));
        })
    });
    ui.add_space(16.);
}

fn create_cpu_debug_panel(ui: &mut Ui, info: CpuDebugInfo) {
    ui.heading("Cpu Info");
    ui.separator();

    // Make a scrollable area for the array grid.
    ui.push_id("cpu-info", |ui| {
        Grid::new("cpu flags").striped(true).show(ui, |ui| {
            ui.label("Accumulator:");
            ui.label(format!("{}", info.acc_reg));
            ui.end_row();

            ui.label("X Register:");
            ui.label(format!("{}", info.x_reg));
            ui.end_row();

            ui.label("Y Register:");
            ui.label(format!("{}", info.y_reg));
            ui.end_row();

            ui.label("Stack Pointer:");
            ui.label(format!("{}", info.stk_ptr));
            ui.end_row();

            ui.label("Program Counter:");
            ui.label(format!("{}", info.pgrm_ctr));
            ui.end_row();

            ui.label("Status:");
            ui.label(format!("{:08b}", info.status));
            ui.end_row();

            ui.label("Fetched:");
            ui.label(format!("{}", info.fetched));
            ui.end_row();

            ui.label("Absolute Address:");
            ui.label(format!("{}", info.addr_abs));
            ui.end_row();

            ui.label("Relative Address:");
            ui.label(format!("{}", info.addr_rel));
            ui.end_row();

            ui.label("Opcode:");
            ui.label(format!("{:x}: {:?}", info.opcode_index, info.opcode));
            ui.end_row();

            ui.label("Cycles:");
            ui.label(format!("{}", info.cycles));
            ui.end_row();
        })
    });
    ui.add_space(16.);
}
