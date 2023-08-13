use crate::cpu::CPU;
use crate::{Read, Write};

pub struct Bus<'a> {
    cpu: CPU<'a>,
    ram: [u8; 64 * 1024]
}

impl Write for Bus {
    fn write(&self, addr: u16, data: u8) {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr] = data;
        }
    }
}

impl Read for Bus {
    fn read(&self, addr: u16, _read_only: bool) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr]
        } else {
            0x00
        }
    }
}

impl Bus {
    pub fn new() -> Self {
        let mut bus = Self {
            cpu: CPU::default(),
            ram: [0; 64 * 1024],
        };
        bus.cpu.connect_bus(&bus);
        bus
    }
}