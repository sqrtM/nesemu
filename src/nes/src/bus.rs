use nesemu_core::{Write, Read};

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
        let mut ram = [0u8; 64 * 1024];
        ram[0] = 0xA5;
        ram[1] = 0x1;
        ram[2] = 0x65;
        ram[3] = 0x1;
        ram[4] = 0x85;
        ram[5] = 0x1;
        Bus { ram }
    }
}
