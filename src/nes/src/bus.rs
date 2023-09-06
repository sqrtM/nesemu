use nesemu_core::{Read, Write};

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
        ram[0 + 0x8000] = 0xA2;
        ram[1 + 0x8000] = 0x0A;
        ram[2 + 0x8000] = 0x8E;
        ram[3 + 0x8000] = 0x00;
        ram[4 + 0x8000] = 0x00;
        ram[5 + 0x8000] = 0xA2;
        ram[6 + 0x8000] = 0x03;
        ram[7 + 0x8000] = 0x8E;

        ram[8 + 0x8000] = 0x01;
        ram[9 + 0x8000] = 0x00;
        ram[10 + 0x8000] = 0xAC;
        ram[11 + 0x8000] = 0x00;
        ram[12 + 0x8000] = 0x00;
        ram[13 + 0x8000] = 0xA9;
        ram[14 + 0x8000] = 0x00;
        ram[15 + 0x8000] = 0x18;

        ram[16 + 0x8000] = 0x6D;
        ram[17 + 0x8000] = 0x01;
        ram[18 + 0x8000] = 0x00;
        ram[19 + 0x8000] = 0x88;
        ram[20 + 0x8000] = 0xD0;
        ram[21 + 0x8000] = 0xFA;
        ram[22 + 0x8000] = 0x8D;
        ram[23 + 0x8000] = 0x02;

        ram[24 + 0x8000] = 0x00;
        ram[25 + 0x8000] = 0xEA;
        ram[26 + 0x8000] = 0xEA;
        ram[27 + 0x8000] = 0xEA;

        ram[0xFFFC] = 0x00;
        ram[0xFFFD] = 0x80;
        Bus { ram }
    }
}
