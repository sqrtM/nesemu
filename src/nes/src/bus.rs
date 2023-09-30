use std::fs;

use nesemu_core::{Read, Write};

pub struct Bus {
    pub ram: [u8; 0xFFFF],
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
        let mut ram = [0; 65535];
        ram[1] = 44;
        ram[2] = 38;
        ram[3] = 43;

        Bus { ram }
    }
}

impl Bus {
    pub fn load(&mut self) {
        let contents: Vec<u8> = fs::read("src/nes/tests/roms/nestest.nes")
            .expect("Should have been able to read the file");
        for (i, j) in contents.iter().enumerate() {
            self.ram[i] = j.clone();
        }
    }
}
