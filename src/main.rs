use crate::bus::Bus;
use crate::cpu::CPU;

/// https://youtu.be/8XmxKPJDGU0?t=1692

mod cpu;
mod bus;
mod instruction;
mod addressing_mode;
mod op_code;

pub trait Read {
    fn read(&self, addr: u16, _read_only: bool) -> u8;
}

pub trait Write {
    fn write(&mut self, addr: u16, data: u8) -> ();
}


fn main() {
    println!("Hello, world!");

    let bus = Bus::default();
    let mut cpu = CPU::default();

    cpu.bus = Some(Box::new(bus));

    let val = &cpu.fetch();

    println!("{:?}", &cpu.bus.unwrap().ram);
}
