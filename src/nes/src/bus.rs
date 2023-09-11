use std::fs;

use nesemu_core::{Read, Write};

pub struct Bus {
    pub(crate) ram: [u8; 0xFFFF],
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
        let ram = [0u8; 0xFFFF];
        Bus { ram }
    }
}

impl Bus {
    pub(crate) fn load(&mut self) {
        let contents: Vec<u8> = fs::read("src/nes/tests/roms/nestest.nes")
            .expect("Should have been able to read the file");
        for (i, j) in contents.iter().enumerate() {
            self.ram[i] = j.clone();
        }
    }
}
