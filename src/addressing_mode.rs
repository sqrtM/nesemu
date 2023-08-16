use crate::cpu::CPU;
use crate::Read;

#[derive(PartialEq)]
pub enum AddressingMode {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY,
}

impl CPU {
    fn address(&mut self, addressing_mode: AddressingMode) -> u8 {
        match addressing_mode {
            AddressingMode::IMP => {
                self.fetched = self.acc_reg;
                0
            }
            AddressingMode::IMM => {
                self.pgrm_ctr += 1;
                self.addr_abs = self.pgrm_ctr;
                0
            }
            AddressingMode::ZP0 => {
                self.addr_abs = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;
                self.addr_abs &= 0x00FF;
                0
            }
            AddressingMode::ZPX => {
                self.addr_abs = (self.read(self.pgrm_ctr, false) + self.x_reg) as u16;
                self.pgrm_ctr += 1;
                self.addr_abs &= 0x00FF;
                0
            }
            AddressingMode::ZPY => {
                self.addr_abs = (self.read(self.pgrm_ctr, false) + self.y_reg) as u16;
                self.pgrm_ctr += 1;
                self.addr_abs &= 0x00FF;
                0
            }
            AddressingMode::REL => {
                self.addr_abs = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;
                // if top bit == 1
                if self.addr_rel & 0x80 != 0 {
                    self.addr_rel |= 0xFF00
                }
                0
            }
            AddressingMode::ABS => {
                let low: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;

                let hi: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;

                self.addr_abs = (hi << 8) | low;
                0
            }
            AddressingMode::ABX => {
                let low: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;

                let hi: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;

                self.addr_abs = (hi << 8) | low;
                self.addr_abs += self.x_reg as u16;

                if (self.addr_abs & 0xFF00) != hi << 0 {
                    1
                } else {
                    0
                }
            }
            AddressingMode::ABY => {
                let low: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;

                let hi: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;

                self.addr_abs = (hi << 8) | low;
                self.addr_abs += self.y_reg as u16;

                if (self.addr_abs & 0xFF00) != hi << 0 {
                    1
                } else {
                    0
                }
            }
            AddressingMode::IND => {
                let ptr_low: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;
                let ptr_hi: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;
                let ptr = (ptr_hi << 8) | ptr_low;

                //emulating the page boundary bug ...
                if ptr_low == 0xFF {
                    self.addr_abs = ((self.read(ptr & 0xFF00, false).wrapping_shl(8)) | self.read(ptr, false)) as u16
                } else {
                    self.addr_abs = ((self.read(ptr, false).wrapping_shl(8)) | self.read(ptr, false)) as u16
                }
                0
            }
            AddressingMode::IZX => {
                let t: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;

                let low: u16 = self.read((t + self.x_reg as u16) & 0x00FF, false) as u16;
                let hi: u16 = self.read((t + self.x_reg as u16 + 1) & 0x00FF, false) as u16;

                self.addr_abs = (hi << 8) | low;

                0
            }
            AddressingMode::IZY => {
                let t: u16 = self.read(self.pgrm_ctr, false) as u16;
                self.pgrm_ctr += 1;

                let low: u16 = self.read(t & 0x00FF, false) as u16;
                let hi: u16 = self.read((t + 1) & 0x00FF, false) as u16;

                self.addr_abs = (hi << 8) | low;
                self.addr_abs += self.y_reg as u16;

                if (self.addr_abs & 0xFF00) != (hi << 8) {
                    1
                } else {
                    0
                }
            }
        }
    }
}