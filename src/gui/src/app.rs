use std::cell::RefCell;
use std::rc::Rc;

use egui::{CentralPanel, Grid, ScrollArea, Ui};

use nesemu::bus::Bus;
use nesemu::memory::CpuMemory;
use nesemu_cpu::cpu::CPU;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct NesemuGui {
    ram: Rc<RefCell<CpuMemory>>,
}

impl Default for NesemuGui {
    fn default() -> Self {
        let ram = Rc::new(RefCell::new(CpuMemory::new()));
        let mut bus: Bus<CpuMemory> = Bus::new();
        let mut cpu: CPU<Bus<CpuMemory>> = CPU::default();

        bus.connect_ram(ram.clone());
        cpu.connect_bus(Box::new(bus));

        Self { ram: ram.clone() }
    }
}

impl NesemuGui {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        Default::default()
    }
}

impl eframe::App for NesemuGui {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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
            create_ram_panel(ui, "Work RAM", self.ram.borrow().main_ram());
            create_ram_panel(ui, "PPU Registers", self.ram.borrow().ppu_registers());
            create_ram_panel(ui, "APU/IO Registers", self.ram.borrow().apu_io_registers());
            create_ram_panel(ui, "Cartridge Space", self.ram.borrow().cartridge_space());
        });
    }
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
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
