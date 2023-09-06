use nesemu_core::{Read, Write};

use crate::addressing_mode::AddressingMode;
use crate::cpu::StatusFlag::U;

pub struct CPU<Bus: Read + Write> {
    pub bus: Option<Box<Bus>>,

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
    Z,
    I,
    D,
    B,
    U,
    V,
    N,
}

impl StatusFlag {
    pub fn bit(&self) -> u8 {
        match self {
            StatusFlag::C => 1 << 0,
            StatusFlag::Z => 1 << 1,
            StatusFlag::I => 1 << 2,
            StatusFlag::D => 1 << 3,
            StatusFlag::B => 1 << 4,
            StatusFlag::U => 1 << 5,
            StatusFlag::V => 1 << 6,
            StatusFlag::N => 1 << 7,
        }
    }
}

impl<Bus: Read + Write> Read for CPU<Bus> {
    fn read(&self, addr: u16, _read_only: bool) -> u8 {
        self.bus
            .as_ref()
            .expect("no bus connected")
            .read(addr, _read_only)
    }
}

impl<Bus: Read + Write> Write for CPU<Bus> {
    fn write(&mut self, addr: u16, data: u8) {
        self.bus
            .as_mut()
            .expect("no bus connected")
            .write(addr, data)
    }
}

impl<Bus: Read + Write> CPU<Bus> {
    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(self.pgrm_ctr, false);
            self.pgrm_ctr += 1;
            let instruction = self.lookup(self.opcode);
            self.cycles = instruction.cycles
                + self.address(instruction.addressing_mode)
                + self.operation(instruction.opcode);
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
}

impl<Bus: Read + Write> CPU<Bus> {
    pub fn get_flag(&self, flag: StatusFlag) -> u8 {
        if (self.status & flag.bit()) != 0 {
            1
        } else {
            0
        }
    }

    pub(crate) fn set_flag(&mut self, flag: StatusFlag, set: bool) {
        if set {
            self.status |= flag.bit()
        } else {
            self.status &= !flag.bit()
        }
    }

    pub fn reset(&mut self) {
        self.addr_abs = 0xFFFC;
        let low: u16 = self.read(self.addr_abs + 0, false) as u16;
        let hi: u16 = self.read(self.addr_abs + 1, false) as u16;

        self.pgrm_ctr = (hi << 8) | low;

        self.acc_reg = 0;
        self.x_reg = 0;
        self.y_reg = 0;
        self.stk_ptr = 0xFD;
        self.status = 0x00 | U.bit();

        self.addr_rel = 0;
        self.addr_abs = 0;
        self.fetched = 0;

        self.cycles = 8;
    }
}

impl<Bus: Read + Write> Default for CPU<Bus> {
    fn default() -> Self {
        CPU {
            bus: None,
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
