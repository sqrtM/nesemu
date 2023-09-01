use std::ptr::addr_of;
use crate::{Read, Write};

pub struct Bus {
    pub(crate) ram: [u8; 64 * 1024],
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

impl Default for Bus {
    fn default() -> Self {
        Bus {
            ram: [0u8; 64 * 1024]
        }
    }
}

impl Bus {
    pub fn get_ram_addr(&self) -> Box<[u8; 65536]> {
        Box::new(self.ram)
    }
}