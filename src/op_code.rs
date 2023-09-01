use crate::{Read, Write};
use crate::addressing_mode::AddressingMode::IMP;
use crate::cpu::CPU;
use crate::cpu::StatusFlag::{B, C, D, I, N, U, V, Z};

#[derive(Clone)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,

    XXX,
}

impl CPU {
    fn operation(&mut self, opcode: Opcode) -> u8 {
        match opcode {
            Opcode::ADC => {
                self.fetch();
                let tmp = (self.acc_reg + self.fetched + self.get_flag(C)) as u16;
                self.set_flag(C, tmp > 255);
                self.set_flag(Z, (tmp & 0xFF) == 0);
                let set: bool = !(self.acc_reg ^ self.fetched) as u16 & (self.acc_reg as u16 ^ tmp) & 0x80 == 1;
                self.set_flag(V, set);
                self.set_flag(N, tmp & 0x80 == 1);
                self.acc_reg = (tmp & 0xFF) as u8;
                1
            }
            Opcode::AND => {
                self.fetch();
                self.acc_reg = self.acc_reg & self.fetched;
                self.set_flag(Z, self.acc_reg == 0);
                self.set_flag(N, self.acc_reg & 0x80 == 1);
                1
            }
            Opcode::ASL => {
                self.fetch();
                let tmp: u16 = (self.fetched << 1) as u16;
                self.set_flag(C, (tmp & 0xFF00) > 0);
                self.set_flag(Z, (tmp & 0x00FF) == 0);
                self.set_flag(N, tmp & 0x80 == 1);
                if self.lookup(self.opcode).addressing_mode == IMP {
                    self.acc_reg = (tmp & 0xFF) as u8;
                } else {
                    self.write(self.addr_abs, (tmp & 0xFF) as u8);
                }
                0
            }
            Opcode::BCC => {
                if self.get_flag(C) == 0 {
                    self.branch()
                }
                0
            }
            Opcode::BCS => {
                if self.get_flag(C) == 1 {
                    self.branch()
                }
                0
            }
            Opcode::BEQ => {
                if self.get_flag(Z) == 1 {
                    self.branch()
                }
                0
            }
            Opcode::BIT => {
                self.fetch();
                let temp = self.acc_reg & self.fetched;
                self.set_flag(Z, (temp & 0xFF) == 0);
                self.set_flag(N, self.fetched & (1 << 7) == 1);
                self.set_flag(V, self.fetched & (1 << 7) == 1);
                0
            }
            Opcode::BMI => {
                if self.get_flag(N) == 1 {
                    self.branch()
                }
                0
            }
            Opcode::BNE => {
                if self.get_flag(Z) == 0 {
                    self.branch()
                }
                0
            }
            Opcode::BPL => {
                if self.get_flag(N) == 0 {
                    self.branch()
                }
                0
            }
            Opcode::BRK => {
                self.pgrm_ctr += 1;

                self.set_flag(I, true);
                self.write(0x0100 + self.stk_ptr as u16, ((self.pgrm_ctr >> 8) & 0xFF) as u8);
                self.stk_ptr -= 1;
                self.write(0x0100 + self.stk_ptr as u16, (self.pgrm_ctr & 0xFF) as u8);
                self.stk_ptr -= 1;

                self.set_flag(B, true);
                self.write(0x0100 + self.stk_ptr as u16, self.status);
                self.stk_ptr -= 1;
                self.set_flag(B, false);

                self.pgrm_ctr |= (self.read(0x100 + self.stk_ptr as u16, false) as u16) << 8;
                0
            }
            Opcode::BVC => {
                if self.get_flag(V) == 0 {
                    self.branch()
                }
                0
            }
            Opcode::BVS => {
                if self.get_flag(V) == 1 {
                    self.branch()
                }
                0
            }
            Opcode::CLC => {
                self.set_flag(C, false);
                0
            }
            Opcode::CLD => {
                self.set_flag(D, false);
                0
            }
            Opcode::CLI => {
                self.set_flag(I, false);
                0
            }
            Opcode::CLV => {
                self.set_flag(V, false);
                0
            }
            Opcode::CMP => {
                self.fetch();
                let temp: u16 = (self.acc_reg - self.fetched) as u16;
                self.set_flag(C, self.acc_reg >= self.fetched);
                self.set_flag(Z, (temp & 0xFF) == 0);
                self.set_flag(N, temp & 0x80 == 1);
                1
            }
            Opcode::CPX => {
                self.fetch();
                let temp: u16 = (self.x_reg - self.fetched) as u16;
                self.set_flag(C, self.acc_reg >= self.fetched);
                self.set_flag(Z, (temp & 0xFF) == 0);
                self.set_flag(N, temp & 0x80 == 1);
                0
            }
            Opcode::CPY => {
                self.fetch();
                let temp: u16 = (self.y_reg - self.fetched) as u16;
                self.set_flag(C, self.acc_reg >= self.fetched);
                self.set_flag(Z, (temp & 0xFF) == 0);
                self.set_flag(N, temp & 0x80 == 1);
                0
            }
            Opcode::DEC => {
                self.fetch();
                let temp = self.fetched - 1;
                self.write(self.addr_abs, temp & 0xFF);
                self.set_flag(Z, (temp & 0xFF) == 0);
                self.set_flag(N, temp & 0x80 == 1);
                0
            }
            Opcode::DEX => {
                self.x_reg -= 1;
                self.set_flag(Z, self.x_reg == 0);
                self.set_flag(N, self.x_reg & 0x80 == 1);
                0
            }
            Opcode::DEY => {
                self.y_reg -= 1;
                self.set_flag(Z, self.y_reg == 0);
                self.set_flag(N, self.y_reg & 0x80 == 1);
                0
            }
            Opcode::EOR => {
                self.fetch();
                self.acc_reg = self.acc_reg ^ self.fetched;
                self.set_flag(Z, self.acc_reg == 0);
                self.set_flag(N, self.acc_reg & 0x80 == 1);
                1
            }
            Opcode::INC => {
                self.fetch();
                let temp = self.fetched + 1;
                self.write(self.addr_abs, temp & 0xFF);
                self.set_flag(Z, (temp & 0xFF) == 0);
                self.set_flag(N, temp & 0x80 == 1);
                0
            }
            Opcode::INX => {
                self.x_reg += 1;
                self.set_flag(Z, self.x_reg == 0);
                self.set_flag(N, self.x_reg & 0x80 == 1);
                0
            }
            Opcode::INY => {
                self.y_reg += 1;
                self.set_flag(Z, self.y_reg == 0);
                self.set_flag(N, self.y_reg & 0x80 == 1);
                0
            }
            Opcode::JMP => {
                self.pgrm_ctr = self.addr_abs;
                0
            }
            Opcode::JSR => {
                self.pgrm_ctr -= 1;
                self.write(0x100 + self.stk_ptr as u16, ((self.pgrm_ctr >> 8) & 0xFF) as u8);
                0
            }
            Opcode::LDA => {
                self.fetch();
                self.acc_reg = self.fetched;
                self.set_flag(Z, self.acc_reg == 0);
                self.set_flag(N, self.acc_reg & 0x80 == 1);
                1
            }
            Opcode::LDX => {
                self.fetch();
                self.x_reg = self.fetched;
                self.set_flag(Z, self.x_reg == 0);
                self.set_flag(N, self.x_reg & 0x80 == 1);
                1
            }
            Opcode::LDY => {
                self.fetch();
                self.y_reg = self.fetched;
                self.set_flag(Z, self.y_reg == 0);
                self.set_flag(N, self.y_reg & 0x80 == 1);
                1
            }
            Opcode::LSR => {
                self.fetch();
                self.set_flag(C, self.fetched & 1 == 1);

                let temp = self.fetched >> 1;
                self.set_flag(Z, (temp & 0xFF) == 0);
                self.set_flag(N, temp & 0x80 == 1);
                if self.lookup(self.opcode).addressing_mode == IMP {
                    self.acc_reg = temp & 0xFF;
                } else {
                    self.write(self.addr_abs, temp & 0xFF);
                }
                0
            }
            Opcode::NOP => {
                // This is not hardware identical, since in the 6502,
                // some nOpcodes take 1 cycle extra. TODO???
                0
            }
            Opcode::ORA => {
                self.fetch();
                self.acc_reg |= self.fetched;
                self.set_flag(Z, self.acc_reg == 0);
                self.set_flag(N, self.acc_reg & 0x80 == 1);
                1
            }
            Opcode::PHA => {
                self.write(0x100 + self.stk_ptr as u16, self.acc_reg);
                self.stk_ptr -= 1;
                0
            }
            Opcode::PHP => {
                self.write(0x0100 + self.stk_ptr as u16, self.status | B.bit() | U.bit());
                self.set_flag(B, false);
                self.set_flag(U, true);
                self.stk_ptr -= 1;
                0
            }
            Opcode::PLA => {
                self.stk_ptr += 1;
                self.acc_reg = self.read(0x100 + self.stk_ptr as u16, false);
                self.set_flag(Z, self.acc_reg == 0);
                self.set_flag(N, self.acc_reg & 0x80 == 1);
                0
            }
            Opcode::PLP => {
                self.stk_ptr += 1;
                self.status = self.read(0x100 + self.stk_ptr as u16, false);
                self.set_flag(U, true);
                0
            }
            Opcode::ROL => {
                self.fetch();
                let temp: u16 = (self.fetched << 1 | self.get_flag(C)) as u16;

                self.set_flag(C, temp & 0xFF00 == 1);
                self.set_flag(Z, (temp & 0xFF) == 0);
                self.set_flag(N, temp & 0x80 == 1);

                if self.lookup(self.opcode).addressing_mode == IMP {
                    self.acc_reg = (temp & 0xFF) as u8;
                } else {
                    self.write(self.addr_abs, (temp & 0xFF) as u8);
                }
                0
            }
            Opcode::ROR => {
                self.fetch();
                let temp: u16 = (self.get_flag(C) << 7 | self.fetched >> 1) as u16;

                self.set_flag(C, self.fetched & 1 == 1);
                self.set_flag(Z, (temp & 0xFF) == 0);
                self.set_flag(N, temp & 0x80 == 1);

                if self.lookup(self.opcode).addressing_mode == IMP {
                    self.acc_reg = (temp & 0xFF) as u8;
                } else {
                    self.write(self.addr_abs, (temp & 0xFF) as u8);
                }
                0
            }
            Opcode::RTI => {
                self.stk_ptr += 1;
                self.status = self.read(0x100 + self.stk_ptr as u16, false);
                self.status &= !B.bit();
                self.status &= !U.bit();

                self.stk_ptr += 1;
                self.pgrm_ctr = self.read(0x100 + self.stk_ptr as u16, false) as u16;
                self.stk_ptr += 1;
                self.pgrm_ctr |= (self.read(0x100 + self.stk_ptr as u16, false) as u16) << 8;

                0
            }
            Opcode::RTS => {
                self.stk_ptr += 1;
                self.pgrm_ctr = self.read(0x100 + self.stk_ptr as u16, false) as u16;

                self.stk_ptr += 1;
                self.pgrm_ctr |= (self.read(0x100 + self.stk_ptr as u16, false) as u16) << 8;

                self.pgrm_ctr += 1;
                0
            }
            Opcode::SBC => {
                self.fetch();
                let val: u16 = (self.fetched ^ 0xFF) as u16;
                let tmp: u16 = self.acc_reg as u16 + val + self.get_flag(C) as u16;
                self.set_flag(C, tmp > 255);
                self.set_flag(Z, (tmp & 0xFF) == 0);

                let set: bool = !(tmp ^ self.acc_reg as u16) & (tmp ^ val) & 0x80 == 1;
                self.set_flag(V, set);
                self.set_flag(N, tmp & 0x80 == 1);

                self.acc_reg = (tmp & 0xFF) as u8;
                1
            }
            Opcode::SEC => {
                self.set_flag(C, true);
                0
            }
            Opcode::SED => {
                self.set_flag(D, true);
                0
            }
            Opcode::SEI => {
                self.set_flag(I, true);
                0
            }
            Opcode::STA => {
                self.write(self.addr_abs, self.acc_reg);
                0
            }
            Opcode::STX => {
                self.write(self.addr_abs, self.x_reg);
                0
            }
            Opcode::STY => {
                self.write(self.addr_abs, self.y_reg);
                0
            }
            Opcode::TAX => {
                self.x_reg = self.acc_reg;
                self.set_flag(Z, self.x_reg == 0);
                self.set_flag(N, self.x_reg & 0x80 == 1);
                0
            }
            Opcode::TAY => {
                self.y_reg = self.acc_reg;
                self.set_flag(Z, self.y_reg == 0);
                self.set_flag(N, self.y_reg & 0x80 == 1);
                0
            }
            Opcode::TSX => {
                self.x_reg = self.stk_ptr;
                self.set_flag(Z, self.x_reg == 0);
                self.set_flag(N, self.x_reg & 0x80 == 1);
                0
            }
            Opcode::TXA => {
                self.acc_reg = self.x_reg;
                self.set_flag(Z, self.acc_reg == 0);
                self.set_flag(N, self.acc_reg & 0x80 == 1);
                0
            }
            Opcode::TXS => {
                self.stk_ptr = self.x_reg;
                0
            }
            Opcode::TYA => {
                self.acc_reg = self.y_reg;
                self.set_flag(Z, self.acc_reg == 0);
                self.set_flag(N, self.acc_reg & 0x80 == 1);
                0
            }
            Opcode::XXX => {
                // any unreachable codes are nOps
                0
            }
        }
    }

    fn branch(&mut self) -> () {
        self.cycles += 1;
        self.addr_abs = self.pgrm_ctr + self.addr_rel;

        if (self.addr_abs & 0xFF00) != (self.pgrm_ctr & 0xFF00) {
            self.cycles += 1;
        }
        self.pgrm_ctr = self.addr_abs;
    }
}