use std::sync::{Arc, RwLock};

use nesemu_cpu::cpu::{CpuDebugInfo, FlagData, CPU};

use crate::bus::Bus;
use crate::memory::CpuMemory;

pub mod bus;
pub mod memory;

pub struct Nes {
    pub cpu: CPU<Bus<CpuMemory>>,
    pub ram: Arc<RwLock<CpuMemory>>,
    pub bus: Arc<RwLock<Bus<CpuMemory>>>,
}

impl Nes {
    pub fn get_main_ram(&self) -> [u8; 2048] {
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
    pub fn get_cpu_flags(&self) -> FlagData {
        self.cpu.get_flag_data()
    }

    pub fn get_cpu_debug_info(&self) -> CpuDebugInfo {
        self.cpu.get_cpu_debug_info()
    }
}
