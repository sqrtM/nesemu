use crate::{Read, Write};
use crate::addressing_mode::AddressingMode;
use crate::bus::Bus;

pub struct CPU {
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
    fn bit(&self) -> u8 {
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

impl Read for CPU {
    fn read(&self, addr: u16, _read_only: bool) -> u8 {
        self.bus.as_ref().expect("no bus connected").read(addr, _read_only)
    }
}

impl Write for CPU {
    fn write(&mut self, addr: u16, data: u8) {
        self.bus.as_mut().expect("no bus connected").write(addr, data)
    }
}

impl CPU {
    fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(self.pgrm_ctr, false);
            self.pgrm_ctr += 1;
            self.cycles = self.lookup(self.opcode).cycles
        }
    }

    pub(crate) fn fetch(&mut self) -> u8 {
        if !(self.lookup(self.opcode).addressing_mode == AddressingMode::IMP) {
            self.fetched = self.read(self.addr_abs, false);
        }
        self.fetched
    }
}

impl CPU {
    pub(crate) fn get_flag(&self, flag: StatusFlag) -> u8 {
        if (self.status & flag.bit()) > 0 {
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
}

impl Default for CPU {
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

