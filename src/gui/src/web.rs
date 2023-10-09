//#![cfg(target_arch = "wasm32")]

use std::sync::{Arc, RwLock};

use nesemu::bus::Bus;
use nesemu::memory::CpuMemory;
use nesemu::Nes;
use nesemu_cpu::cpu::CPU;

use crate::app::NesemuGui;
use crate::create_channels;

pub fn run() {
    use wasm_bindgen::prelude::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::Worker;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
                    let ram = Arc::new(RwLock::new(CpuMemory::default()));
                    let bus = Arc::new(RwLock::new(Bus::new(ram.clone())));
                    let mut cpu = CPU::new(bus.clone());
                    cpu.reset();

                    let nes = Nes {
                        ram: ram.clone(),
                        cpu,
                        bus: bus.clone(),
                    };
                    let nes_ref = Arc::new(RwLock::new(nes));
                    let nes_ref_2 = nes_ref.clone();

                    // Set up communication channels between emulator and GUI
                    let (emulator_tx, emulator_rx, gui_tx, gui_rx) = create_channels();

                    let ctx = cc.egui_ctx.clone();
                    let worker = Worker::new("./worker.js").unwrap();
                    let onmessage_callback =
                        Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
                            // if we ever want to handle the data from js
                            //let data = event.data();
                            nes_ref_2.clone().write().unwrap().cpu.clock();
                            ctx.request_repaint();
                        }) as Box<dyn FnMut(_)>);
                    worker.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                    onmessage_callback.forget();

                    Box::new(NesemuGui::new(cc, gui_tx, nes_ref.clone()))
                }),
            )
            .await
            .expect("failed to start web client");
    });
}
