use std::fmt::format;
use std::fs::OpenOptions;
use std::io::Write as fsWrite;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use nesemu_core::{Read, Write};

use crate::addressing_mode::AddressingMode;
use crate::cpu::StatusFlag::{B, C, D, I, N, U, V, Z};
use crate::instruction::Instruction;

pub struct CPU<Bus: Read + Write> {
    bus: Arc<RwLock<Bus>>,

    pub acc_reg: u8,
    pub x_reg: u8,
    pub y_reg: u8,
    pub stk_ptr: u8,
    pub pgrm_ctr: u16,
    pub status: u8,

    pub fetched: u8,
    pub addr_abs: u16,
    pub addr_rel: u16,
    pub opcode: u8,
    pub cycles: u8,
}

#[derive(Debug)]
pub enum StatusFlag {
    C,
    // Carry
    Z,
    // Zero
    I,
    // Interrupt Disable
    D,
    // Decimal
    B,
    // "B" Flag
    U,
    // Unused (always 1)
    V,
    // Overflow
    N, // Negative
}

#[allow(non_snake_case)]
#[derive(Default, Deserialize, Serialize)]
pub struct FlagData {
    pub C: u8,
    // Carry
    pub Z: u8,
    // Zero
    pub I: u8,
    // Interrupt Disable
    pub D: u8,
    // Decimal
    pub B: u8,
    // "B" Flag
    pub U: u8,
    // Unused (always 1)
    pub V: u8,
    // Overflow
    pub N: u8, // Negative
}

impl StatusFlag {
    pub fn bit(&self) -> u8 {
        match self {
            C => 1 << 0,
            Z => 1 << 1,
            I => 1 << 2,
            D => 1 << 3,
            B => 1 << 4,
            U => 1 << 5,
            V => 1 << 6,
            N => 1 << 7,
        }
    }
}

impl<Bus: Read + Write> Read for CPU<Bus> {
    fn read(&self, addr: u16, _read_only: bool) -> u8 {
        self.bus
            .read()
            .unwrap()
            .read(addr, _read_only)
    }
}

impl<Bus: Read + Write> Write for CPU<Bus> {
    fn write(&mut self, addr: u16, data: u8) {
        self.bus
            .write()
            .unwrap()
            .write(addr, data)
    }
}

#[derive(Debug)]
pub struct CpuDebugInfo {
    pub acc_reg: u8,
    pub x_reg: u8,
    pub y_reg: u8,
    pub stk_ptr: u8,
    pub pgrm_ctr: u16,
    pub status: u8,

    pub fetched: u8,
    pub addr_abs: u16,
    pub addr_rel: u16,
    pub opcode_index: u8,
    pub opcode: Instruction,
    pub cycles: u8,
}

impl<Bus: Read + Write> CPU<Bus> {
    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(self.pgrm_ctr, false);
            let instruction = self.lookup(self.opcode);
            //log
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("log.txt")
                .unwrap();
            let m = format!("{:X?}", self.get_cpu_debug_info());
            if let Err(e) = writeln!(file, "{}", m) {
                eprintln!("Couldn't write to file: {}", e);
            }
            self.cycles = instruction.cycles
                + self.address(instruction.addressing_mode)
                + self.operation(instruction.opcode);
            self.pgrm_ctr += 1;
        } else {
            self.cycles -= 1;
        }
        self.set_flag(U, true);
    }

    pub(crate) fn fetch(&mut self) -> u8 {
        if !(self.lookup(self.opcode).addressing_mode == AddressingMode::IMP) {
            self.fetched = self.read(self.addr_abs, false);
        }
        self.fetched
    }

    pub fn get_flag_data(&self) -> FlagData {
        FlagData {
            C: self.get_flag(C),
            Z: self.get_flag(Z),
            I: self.get_flag(I),
            D: self.get_flag(D),
            B: self.get_flag(B),
            U: self.get_flag(U),
            V: self.get_flag(V),
            N: self.get_flag(N),
        }
    }

    pub fn get_cpu_debug_info(&self) -> CpuDebugInfo {
        CpuDebugInfo {
            acc_reg: self.acc_reg,
            x_reg: self.x_reg,
            y_reg: self.y_reg,
            stk_ptr: self.stk_ptr,
            pgrm_ctr: self.pgrm_ctr,
            status: self.status,
            fetched: self.fetched,
            addr_abs: self.addr_abs,
            addr_rel: self.addr_rel,
            opcode_index: self.opcode,
            opcode: self.lookup(self.opcode),
            cycles: self.cycles,
        }
    }
}

impl<Bus: Read + Write> CPU<Bus> {
    pub fn get_flag(&self, flag: StatusFlag) -> u8 {
        if (self.status & flag.bit()) != 0 {
            1
        } else {
            0
        }
    }

    pub fn set_flag(&mut self, flag: StatusFlag, set: bool) {
        if set {
            self.status |= flag.bit()
        } else {
            self.status &= !flag.bit()
        }
    }

    pub fn reset(&mut self) {
        self.pgrm_ctr = 0xC5F5;
        //?? self.pgrm_ctr = 0xC5F5;

        self.acc_reg = 0;
        self.x_reg = 0;
        self.y_reg = 0;
        self.stk_ptr = 0xFD;
        self.status = U.bit();

        self.addr_rel = 0;
        self.addr_abs = 0;
        self.fetched = 0;

        self.cycles = 0;
    }
}

impl<Bus: Read + Write> CPU<Bus> {
    pub fn new(bus: Arc<RwLock<Bus>>) -> Self {
        CPU {
            bus,
            acc_reg: 0,
            x_reg: 0,
            y_reg: 0,
            stk_ptr: 0,
            pgrm_ctr: 0,
            status: 0,
            fetched: 0,
            addr_abs: 0,
            addr_rel: 0,
            opcode: 0,
            cycles: 0,
        }
    }
}
