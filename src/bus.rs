use crate::cpu::CPU;
use crate::{Read, Write};

pub struct Bus {
    cpu: CPU,
    ram: [u8; 64 * 1024]
}

impl Write for Bus {
    fn write(&mut self, addr: u16, data: u8) {
            self.ram[addr as usize] = data;
    }
}

impl Read for Bus {
    fn read(&self, addr: u16, _read_only: bool) -> u8 {
            self.ram[addr as usize]
    }
}

impl Bus {
    pub fn new(self) -> Self {
        Self {
            cpu: CPU {
                bus: Some(Box::new(self)),
                ..CPU::default()
            },
            ram: [0; 64 * 1024],
        }
    }
}