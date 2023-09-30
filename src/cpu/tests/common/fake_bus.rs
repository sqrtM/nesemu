use nesemu_core::{Read, Write};

pub struct FakeBus {
    pub(crate) ram: [u8; 0xFFFF],
}

impl Write for FakeBus {
    fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}

impl Read for FakeBus {
    fn read(&self, addr: u16, _read_only: bool) -> u8 {
        self.ram[addr as usize]
    }
}

impl Default for FakeBus {
    fn default() -> Self {
        let ram = [0u8; 0xFFFF];
        FakeBus { ram }
    }
}
