use eframe::glow::MAX_HEIGHT;
use egui::{CentralPanel, Grid, ScrollArea};
use serde::{Deserializer, Serializer};
use nesemu::bus::Bus;
use nesemu_cpu::cpu::CPU;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct NesemuGui {
    #[serde(serialize_with = "serialize_array", deserialize_with = "deserialize_array")]
    ram: Box<[u8; 65535]>,
}

fn serialize_array<S>(data: &Box<[u8; 65535]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_bytes(&data[..])
}

fn deserialize_array<'de, D>(deserializer: D) -> Result<Box<[u8; 65535]>, D::Error>
    where
        D: Deserializer<'de>,
{
    let data: Vec<u8> = serde_bytes::deserialize(deserializer)?;

    // Ensure the deserialized data has the correct length
    if data.len() != 65535 {
        return Err(serde::de::Error::custom("Expected an array of length 65535"));
    }

    let mut array = [0u8; 65535];
    array.copy_from_slice(&data);
    Ok(Box::new(array))
}

impl Default for NesemuGui {
    fn default() -> Self {
        let bus = Bus::default();
        let mut cpu: CPU<_> = CPU::default();

        cpu.bus = Some(Box::new(bus));
        cpu.reset();

        let ram = Box::new(cpu.bus.as_ref().expect("what????").ram.clone());
        Self {
            ram
        }
    }
}

impl NesemuGui {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

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
            ui.heading("6502 Emulator RAM Display");
            ui.separator();

            // Make a scrollable area for the RAM grid.
            ScrollArea::vertical().max_height(MAX_HEIGHT as f32).show(ui, |ui| {


                // Display the RAM array as a hex grid.
                Grid::new("ram_grid").striped(true).show(ui, |ui| {
                    for (i, &byte) in self.ram.iter().enumerate() {
                        if i % 8 == 0 {
                            // Display the index of the first address of the line.
                            ui.monospace(format!("{:04X}:", i));
                        }

                        // Display the hexadecimal byte value.
                        ui.monospace(format!("{:02X}", byte));

                        if i % 8 == 7 {
                            // Add a newline for every 8 bytes.
                            ui.end_row();
                        }
                    }
                });
            });
        });
    }
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
