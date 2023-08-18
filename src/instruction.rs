use crate::addressing_mode::AddressingMode;
use crate::cpu::CPU;
use crate::op_code::Opcode;

#[derive(Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub cycles: u8,
}

impl CPU {
    pub fn lookup(&self, n: u8) -> Instruction {
        match n {
            0x00 => Instruction {
                opcode: Opcode::BRK,
                addressing_mode: AddressingMode::IMM,
                cycles: 7,
            },
            0x01 => Instruction {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::IZX,
                cycles: 6,
            },
            0x05 => Instruction {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0x06 => Instruction {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::ZP0,
                cycles: 5,
            },
            0x08 => Instruction {
                opcode: Opcode::PHP,
                addressing_mode: AddressingMode::IMP,
                cycles: 3,
            },
            0x09 => Instruction {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0x0A => Instruction {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x0D => Instruction {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0x0E => Instruction {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::ABS,
                cycles: 6,
            },
            0x10 => Instruction {
                opcode: Opcode::BPL,
                addressing_mode: AddressingMode::REL,
                cycles: 2,
            },
            0x11 => Instruction {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::IZY,
                cycles: 5,
            },
            0x15 => Instruction {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0x16 => Instruction {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::ZPX,
                cycles: 6,
            },
            0x18 => Instruction {
                opcode: Opcode::CLC,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x19 => Instruction {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::ABY,
                cycles: 4,
            },
            0x1D => Instruction {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::ABX,
                cycles: 4,
            },
            0x1E => Instruction {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::ABX,
                cycles: 7,
            },
            0x20 => Instruction {
                opcode: Opcode::JSR,
                addressing_mode: AddressingMode::ABS,
                cycles: 6,
            },
            0x21 => Instruction {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::IZX,
                cycles: 6,
            },
            0x24 => Instruction {
                opcode: Opcode::BIT,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0x25 => Instruction {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0x26 => Instruction {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::ZP0,
                cycles: 5,
            },
            0x28 => Instruction {
                opcode: Opcode::PLP,
                addressing_mode: AddressingMode::IMP,
                cycles: 4,
            },
            0x29 => Instruction {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0x2A => Instruction {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x2C => Instruction {
                opcode: Opcode::BIT,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0x2D => Instruction {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0x2E => Instruction {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::ABS,
                cycles: 6,
            },
            0x30 => Instruction {
                opcode: Opcode::BMI,
                addressing_mode: AddressingMode::REL,
                cycles: 2,
            },
            0x31 => Instruction {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::IZY,
                cycles: 5,
            },
            0x35 => Instruction {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0x36 => Instruction {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::ZPX,
                cycles: 6,
            },
            0x38 => Instruction {
                opcode: Opcode::SEC,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x39 => Instruction {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::ABY,
                cycles: 4,
            },
            0x3D => Instruction {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::ABX,
                cycles: 4,
            },
            0x3E => Instruction {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::ABX,
                cycles: 7,
            },
            0x40 => Instruction {
                opcode: Opcode::RTI,
                addressing_mode: AddressingMode::IMP,
                cycles: 6,
            },
            0x41 => Instruction {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::IZX,
                cycles: 6,
            },
            0x45 => Instruction {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0x46 => Instruction {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::ZP0,
                cycles: 5,
            },
            0x48 => Instruction {
                opcode: Opcode::PHA,
                addressing_mode: AddressingMode::IMP,
                cycles: 3,
            },
            0x49 => Instruction {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0x4A => Instruction {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x4C => Instruction {
                opcode: Opcode::JMP,
                addressing_mode: AddressingMode::ABS,
                cycles: 3,
            },
            0x4D => Instruction {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0x4E => Instruction {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::ABS,
                cycles: 6,
            },
            0x50 => Instruction {
                opcode: Opcode::BVC,
                addressing_mode: AddressingMode::REL,
                cycles: 2,
            },
            0x51 => Instruction {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::IZY,
                cycles: 5,
            },
            0x55 => Instruction {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0x56 => Instruction {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::ZPX,
                cycles: 6,
            },
            0x58 => Instruction {
                opcode: Opcode::CLI,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x59 => Instruction {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::ABY,
                cycles: 4,
            },
            0x5D => Instruction {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::ABX,
                cycles: 4,
            },
            0x5E => Instruction {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::ABX,
                cycles: 7,
            },
            0x60 => Instruction {
                opcode: Opcode::RTS,
                addressing_mode: AddressingMode::IMP,
                cycles: 6,
            },
            0x61 => Instruction {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::IZX,
                cycles: 6,
            },
            0x65 => Instruction {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0x66 => Instruction {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::ZP0,
                cycles: 5,
            },
            0x68 => Instruction {
                opcode: Opcode::PLA,
                addressing_mode: AddressingMode::IMP,
                cycles: 4,
            },
            0x69 => Instruction {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0x6A => Instruction {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x6C => Instruction {
                opcode: Opcode::JMP,
                addressing_mode: AddressingMode::IND,
                cycles: 5,
            },
            0x6D => Instruction {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0x6E => Instruction {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::ABS,
                cycles: 6,
            },
            0x70 => Instruction {
                opcode: Opcode::BVS,
                addressing_mode: AddressingMode::REL,
                cycles: 2,
            },
            0x71 => Instruction {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::IZY,
                cycles: 5,
            },
            0x75 => Instruction {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0x76 => Instruction {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::ZPX,
                cycles: 6,
            },
            0x78 => Instruction {
                opcode: Opcode::SEI,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x79 => Instruction {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::ABY,
                cycles: 4,
            },
            0x7D => Instruction {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::ABX,
                cycles: 4,
            },
            0x7E => Instruction {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::ABX,
                cycles: 7,
            },
            0x81 => Instruction {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::IZX,
                cycles: 6,
            },
            0x84 => Instruction {
                opcode: Opcode::STY,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0x85 => Instruction {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0x86 => Instruction {
                opcode: Opcode::STX,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0x88 => Instruction {
                opcode: Opcode::DEY,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x8A => Instruction {
                opcode: Opcode::TXA,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x8C => Instruction {
                opcode: Opcode::STY,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0x8D => Instruction {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0x8E => Instruction {
                opcode: Opcode::STX,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0x90 => Instruction {
                opcode: Opcode::BCC,
                addressing_mode: AddressingMode::REL,
                cycles: 2,
            },
            0x91 => Instruction {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::IZY,
                cycles: 6,
            },
            0x94 => Instruction {
                opcode: Opcode::STY,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0x95 => Instruction {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0x96 => Instruction {
                opcode: Opcode::STX,
                addressing_mode: AddressingMode::ZPY,
                cycles: 4,
            },
            0x98 => Instruction {
                opcode: Opcode::TYA,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x99 => Instruction {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::ABY,
                cycles: 5,
            },
            0x9A => Instruction {
                opcode: Opcode::TSX,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0x9D => Instruction {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::ABX,
                cycles: 5,
            },
            0xA0 => Instruction {
                opcode: Opcode::LDY,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0xA1 => Instruction {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::IZX,
                cycles: 6,
            },
            0xA2 => Instruction {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0xA4 => Instruction {
                opcode: Opcode::LDY,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0xA5 => Instruction {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0xA6 => Instruction {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0xA8 => Instruction {
                opcode: Opcode::TAY,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xA9 => Instruction {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0xAA => Instruction {
                opcode: Opcode::TAX,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xAC => Instruction {
                opcode: Opcode::LDY,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0xAD => Instruction {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0xAE => Instruction {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0xB0 => Instruction {
                opcode: Opcode::BCS,
                addressing_mode: AddressingMode::REL,
                cycles: 2,
            },
            0xB1 => Instruction {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::IZY,
                cycles: 5,
            },
            0xB4 => Instruction {
                opcode: Opcode::LDY,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0xB5 => Instruction {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0xB6 => Instruction {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::ZPY,
                cycles: 4,
            },
            0xB8 => Instruction {
                opcode: Opcode::CLV,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xB9 => Instruction {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::ABY,
                cycles: 4,
            },
            0xBA => Instruction {
                opcode: Opcode::TSX,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xBC => Instruction {
                opcode: Opcode::LDY,
                addressing_mode: AddressingMode::ABX,
                cycles: 4,
            },
            0xBD => Instruction {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::ABX,
                cycles: 4,
            },
            0xBE => Instruction {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::ABY,
                cycles: 4,
            },
            0xC0 => Instruction {
                opcode: Opcode::CPY,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0xC1 => Instruction {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::IZX,
                cycles: 6,
            },
            0xC4 => Instruction {
                opcode: Opcode::CPY,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0xC5 => Instruction {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0xC6 => Instruction {
                opcode: Opcode::DEC,
                addressing_mode: AddressingMode::ZP0,
                cycles: 5,
            },
            0xC8 => Instruction {
                opcode: Opcode::INY,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xC9 => Instruction {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0xCA => Instruction {
                opcode: Opcode::DEX,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xCC => Instruction {
                opcode: Opcode::CPY,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0xCD => Instruction {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0xCE => Instruction {
                opcode: Opcode::DEC,
                addressing_mode: AddressingMode::ABS,
                cycles: 6,
            },
            0xD0 => Instruction {
                opcode: Opcode::BNE,
                addressing_mode: AddressingMode::REL,
                cycles: 2,
            },
            0xD1 => Instruction {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::IZY,
                cycles: 5,
            },
            0xD5 => Instruction {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0xD6 => Instruction {
                opcode: Opcode::DEC,
                addressing_mode: AddressingMode::ZPX,
                cycles: 6,
            },
            0xD8 => Instruction {
                opcode: Opcode::CLD,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xD9 => Instruction {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::ABY,
                cycles: 4,
            },
            0xDD => Instruction {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::ABX,
                cycles: 4,
            },
            0xDE => Instruction {
                opcode: Opcode::DEC,
                addressing_mode: AddressingMode::ABX,
                cycles: 7,
            },
            0xE0 => Instruction {
                opcode: Opcode::CPX,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0xE1 => Instruction {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::IZX,
                cycles: 6,
            },
            0xE4 => Instruction {
                opcode: Opcode::CPX,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0xE5 => Instruction {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::ZP0,
                cycles: 3,
            },
            0xE6 => Instruction {
                opcode: Opcode::INC,
                addressing_mode: AddressingMode::ZP0,
                cycles: 5,
            },
            0xE8 => Instruction {
                opcode: Opcode::INX,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xE9 => Instruction {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::IMM,
                cycles: 2,
            },
            0xEA => Instruction {
                opcode: Opcode::NOP,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xEC => Instruction {
                opcode: Opcode::CPX,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0xED => Instruction {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0xEE => Instruction {
                opcode: Opcode::INC,
                addressing_mode: AddressingMode::ABS,
                cycles: 6,
            },
            0xF0 => Instruction {
                opcode: Opcode::BEQ,
                addressing_mode: AddressingMode::REL,
                cycles: 2,
            },
            0xF1 => Instruction {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::IZY,
                cycles: 5,
            },
            0xF5 => Instruction {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::ZPX,
                cycles: 4,
            },
            0xF6 => Instruction {
                opcode: Opcode::INC,
                addressing_mode: AddressingMode::ZPX,
                cycles: 6,
            },
            0xF8 => Instruction {
                opcode: Opcode::SED,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
            0xF9 => Instruction {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::ABS,
                cycles: 4,
            },
            0xFD => Instruction {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::ABX,
                cycles: 4,
            },
            0xFE => Instruction {
                opcode: Opcode::INC,
                addressing_mode: AddressingMode::ABX,
                cycles: 7,
            },
            _ => Instruction {
                opcode: Opcode::XXX,
                addressing_mode: AddressingMode::IMP,
                cycles: 2,
            },
        }
    }
}
