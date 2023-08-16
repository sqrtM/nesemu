use crate::cpu::{CPU, StatusFlag};

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
            Opcode::AND => {}
            Opcode::ASL => {}
            Opcode::BCC => {}
            Opcode::BCS => {}
            Opcode::BEQ => {}
            Opcode::BIT => {}
            Opcode::BMI => {}
            Opcode::BNE => {}
            Opcode::BPL => {}
            Opcode::BRK => {}
            Opcode::BVC => {}
            Opcode::BVS => {}
            Opcode::CLC => {}
            Opcode::CLD => {}
            Opcode::CLI => {}
            Opcode::CLV => {}
            Opcode::CMP => {}
            Opcode::CPX => {}
            Opcode::CPY => {}
            Opcode::DEC => {}
            Opcode::DEX => {}
            Opcode::DEY => {}
            Opcode::EOR => {}
            Opcode::INC => {}
            Opcode::INX => {}
            Opcode::INY => {}
            Opcode::JMP => {}
            Opcode::JSR => {}
            Opcode::LDA => {}
            Opcode::LDX => {}
            Opcode::LDY => {}
            Opcode::LSR => {}
            Opcode::NOP => {}
            Opcode::ORA => {}
            Opcode::PHA => {}
            Opcode::PHP => {}
            Opcode::PLA => {}
            Opcode::PLP => {}
            Opcode::ROL => {}
            Opcode::ROR => {}
            Opcode::RTI => {}
            Opcode::RTS => {}
            Opcode::SBC => {
                self.fetch();
                let val: u16 = (self.fetched ^ 0x00FF) as u16;
                let tmp: u16 = self.acc_reg as u16 + val + self.get_flag(StatusFlag::C) as u16;
                self.set_flag(StatusFlag::C, tmp > 255);
                self.set_flag(StatusFlag::Z, (tmp & 0x00FF) == 0);
                let set: bool = !(tmp ^ self.acc_reg as u16) & (tmp ^ val) & 0x0080 == 1;
                self.set_flag(StatusFlag::V, set);
                self.set_flag(StatusFlag::N, tmp & 0x80 == 1);
                self.acc_reg = (tmp & 0xFF) as u8;
                1
            }
            Opcode::SEC => {}
            Opcode::SED => {}
            Opcode::SEI => {}
            Opcode::STA => {}
            Opcode::STX => {}
            Opcode::STY => {}
            Opcode::TAX => {}
            Opcode::TAY => {}
            Opcode::TSX => {}
            Opcode::TXA => {}
            Opcode::TXS => {}
            Opcode::TYA => {}
            Opcode::XXX => {}
        }
    }
}