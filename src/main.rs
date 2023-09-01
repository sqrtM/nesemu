use std::{
    error::Error,
};

use crate::bus::Bus;
use crate::cpu::CPU;

/// https://youtu.be/8XmxKPJDGU0?t=1692

mod cpu;
mod bus;
mod instruction;
mod addressing_mode;
mod op_code;
mod gui;

pub trait Read {
    fn read(&self, addr: u16, _read_only: bool) -> u8;
}

pub trait Write {
    fn write(&mut self, addr: u16, data: u8) -> ();
}


fn main() -> Result<(), Box<dyn Error>> {
    let bus = Bus::default();
    let ram_addr = bus.get_ram_addr();
    let mut cpu = CPU::default();

    cpu.bus = Some(Box::new(bus));
    cpu.write(0, 1);
    cpu.opcode = 2;
    let val = cpu.fetch();

    gui::render(ram_addr)
}
