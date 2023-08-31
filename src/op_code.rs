use crate::addressing_mode::AddressingMode;
use crate::cpu::{CPU, StatusFlag};
use crate::cpu::StatusFlag::{B, C, D, I, N, V, Z};
use crate::{Read, Write};

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
                let tmp = (self.acc_reg + self.fetched + self.get_flag(StatusFlag::C)) as u16;
                self.set_flag(StatusFlag::C, tmp > 255);
                self.set_flag(StatusFlag::Z, (tmp & 0x00FF) == 0);
                let set: bool = !(self.acc_reg ^ self.fetched) as u16 & (self.acc_reg as u16 ^ tmp) & 0x0080 == 1;
                self.set_flag(StatusFlag::V, set);
                self.set_flag(StatusFlag::N, tmp & 0x80 == 1);
                self.acc_reg = (tmp & 0xFF) as u8;
                1
            }
            Opcode::AND => {
                self.fetch();
                self.acc_reg = self.acc_reg & self.fetched;
                self.set_flag(StatusFlag::Z, self.acc_reg == 0x00);
                self.set_flag(StatusFlag::N, self.acc_reg & 0x80 == 1);
                1
            }
            Opcode::ASL => {
                self.fetch();
                let tmp: u16 = (self.fetched << 1) as u16;
                self.set_flag(C, (tmp & 0xFF00) > 0);
                self.set_flag(Z, (tmp & 0x00FF) == 0x00);
                self.set_flag(N, tmp & 0x80 == 1);
                if self.lookup(self.opcode).addressing_mode == AddressingMode::IMP {
                    self.acc_reg = (tmp & 0x00FF) as u8;
                } else {
                    self.write(self.addr_abs, (tmp & 0x00FF) as u8);
                }
                0
            }
            Opcode::BCC => {
                if self.get_flag(C) == 0 {
                    self.cycles += 1;
                    self.addr_abs = self.pgrm_ctr + self.addr_rel;

                    if (self.addr_abs & 0xFF00) != (self.pgrm_ctr & 0xFF00) {
                        self.cycles += 1;
                    }
                    self.pgrm_ctr = self.addr_abs;
                }
                0
            }
            Opcode::BCS => {
                if self.get_flag(C) == 1 {
                    self.cycles += 1;
                    self.addr_abs = self.pgrm_ctr + self.addr_rel;

                    if (self.addr_abs & 0xFF00) != (self.pgrm_ctr & 0xFF00) {
                        self.cycles += 1;
                    }
                    self.pgrm_ctr = self.addr_abs;
                }
                0
            }
            Opcode::BEQ => {
                if self.get_flag(Z) == 1 {
                    self.cycles += 1;
                    self.addr_abs = self.pgrm_ctr + self.addr_rel;

                    if (self.addr_abs & 0xFF00) != (self.pgrm_ctr & 0xFF00) {
                        self.cycles += 1;
                    }
                    self.pgrm_ctr = self.addr_abs;
                }
                0
            }
            Opcode::BIT => {
                self.fetch();
                let temp = self.acc_reg & self.fetched;
                self.set_flag(Z, (temp & 0x00FF) == 0x00);
                self.set_flag(N, self.fetched & (1 << 7) == 1);
                self.set_flag(V, self.fetched & (1 << 7) == 1);
                0
            }
            Opcode::BMI => {
                if self.get_flag(N) == 1 {
                    self.cycles += 1;
                    self.addr_abs = self.pgrm_ctr + self.addr_rel;

                    if (self.addr_abs & 0xFF00) != (self.pgrm_ctr & 0xFF00) {
                        self.cycles += 1;
                    }
                    self.pgrm_ctr = self.addr_abs;
                }
                0
            }
            Opcode::BNE => {
                if self.get_flag(Z) == 0 {
                    self.cycles += 1;
                    self.addr_abs = self.pgrm_ctr + self.addr_rel;

                    if (self.addr_abs & 0xFF00) != (self.pgrm_ctr & 0xFF00) {
                        self.cycles += 1;
                    }
                    self.pgrm_ctr = self.addr_abs;
                }
                0
            }
            Opcode::BPL => {
                if self.get_flag(N) == 0 {
                    self.cycles += 1;
                    self.addr_abs = self.pgrm_ctr + self.addr_rel;

                    if (self.addr_abs & 0xFF00) != (self.pgrm_ctr & 0xFF00) {
                        self.cycles += 1;
                    }
                    self.pgrm_ctr = self.addr_abs;
                }
                0
            }
            Opcode::BRK => {
                self.pgrm_ctr += 1;

                self.set_flag(I, true);
                self.write(0x0100 + self.stk_ptr as u16, ((self.pgrm_ctr >> 8) & 0x00FF) as u8);
                self.stk_ptr -= 1;
                self.write(0x0100 + self.stk_ptr as u16, (self.pgrm_ctr & 0x00FF) as u8);
                self.stk_ptr -= 1;

                self.set_flag(B, true);
                self.write(0x0100 + self.stk_ptr as u16, self.status);
                self.stk_ptr -= 1;
                self.set_flag(B, false);

                self.pgrm_ctr = (self.read(0xFFFE, false) | (self.read(0xFFFF, false) << 8)) as u16;
                0
            }
            Opcode::BVC => {
                if self.get_flag(V) == 0 {
                    self.cycles += 1;
                    self.addr_abs = self.pgrm_ctr + self.addr_rel;

                    if (self.addr_abs & 0xFF00) != (self.pgrm_ctr & 0xFF00) {
                        self.cycles += 1;
                    }
                    self.pgrm_ctr = self.addr_abs;
                }
                0
            }
            Opcode::BVS => {
                if self.get_flag(V) == 1 {
                    self.cycles += 1;
                    self.addr_abs = self.pgrm_ctr + self.addr_rel;

                    if (self.addr_abs & 0xFF00) != (self.pgrm_ctr & 0xFF00) {
                        self.cycles += 1;
                    }
                    self.pgrm_ctr = self.addr_abs;
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
                self.set_flag(Z, (temp & 0x00FF) == 0x0000);
                self.set_flag(N, temp & 0x0080 == 1);
                1
            }
            Opcode::CPX => {
                self.fetch();
                let temp: u16 = (self.x_reg - self.fetched) as u16;
                self.set_flag(C, self.acc_reg >= self.fetched);
                self.set_flag(Z, (temp & 0x00FF) == 0x0000);
                self.set_flag(N, temp & 0x0080 == 1);
                0
            }
            Opcode::CPY => {
                self.fetch();
                let temp: u16 = (self.y_reg - self.fetched) as u16;
                self.set_flag(C, self.acc_reg >= self.fetched);
                self.set_flag(Z, (temp & 0x00FF) == 0);
                self.set_flag(N, temp & 0x0080 == 1);
                0
            }
            Opcode::DEC => {
                self.fetch();
                let temp = self.fetched - 1;
                self.write(self.addr_abs, temp & 0x00FF);
                self.set_flag(Z, (temp & 0x00FF) == 0);
                self.set_flag(N, temp & 0x0080 == 1);
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
            //Opcode::INC => {}
            //Opcode::INX => {}
            //Opcode::INY => {}
            //Opcode::JMP => {}
            //Opcode::JSR => {}
            //Opcode::LDA => {}
            //Opcode::LDX => {}
            //Opcode::LDY => {}
            //Opcode::LSR => {}
            //Opcode::NOP => {}
            //Opcode::ORA => {}
            //Opcode::PHA => {}
            //Opcode::PHP => {}
            //Opcode::PLA => {}
            //Opcode::PLP => {}
            //Opcode::ROL => {}
            //Opcode::ROR => {}
            //Opcode::RTI => {}
            //Opcode::RTS => {}
            Opcode::SBC => {
                self.fetch();
                let val: u16 = (self.fetched ^ 0x00FF) as u16;
                let tmp: u16 = self.acc_reg as u16 + val + self.get_flag(C) as u16;
                self.set_flag(C, tmp > 255);
                self.set_flag(Z, (tmp & 0x00FF) == 0);
                let set: bool = !(tmp ^ self.acc_reg as u16) & (tmp ^ val) & 0x0080 == 1;
                self.set_flag(V, set);
                self.set_flag(N, tmp & 0x80 == 1);
                self.acc_reg = (tmp & 0xFF) as u8;
                1
            }
            //Opcode::SEC => {}
            //Opcode::SED => {}
            //Opcode::SEI => {}
            //Opcode::STA => {}
            //Opcode::STX => {}
            //Opcode::STY => {}
            //Opcode::TAX => {}
            //Opcode::TAY => {}
            //Opcode::TSX => {}
            //Opcode::TXA => {}
            //Opcode::TXS => {}
            //Opcode::TYA => {}
            //Opcode::XXX => {}
            _ => 0
        }
    }
}