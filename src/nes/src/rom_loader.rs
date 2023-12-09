use std::fs;

use nesemu_core::{Read, Write};

use crate::Nes;

pub struct Rom {
    header: Vec<u8>,
    trainer: Option<Vec<u8>>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Rom {
    fn build(head: &[u8]) {
        if !(head[0] == 0x4E && head[1] == 0x45 && head[2] == 0x53 && head[3] == 0x1A) {
            // header bad
        }
    }
}

impl Nes {
    pub fn load_rom(&mut self, path: &str) -> Result<(), ()> {
        let mut bytes: Vec<u8> = fs::read(path).expect("prob openin file");
        bytes.drain(0..0x10);
        for (k, v) in bytes.iter().enumerate() {
            if (k) < 0x4000 {
                self.bus.write((k + 0x8000) as u16, *v);
                self.bus.write((k + 0xC000) as u16, *v);
            }
        }
        Ok(())
    }
}
