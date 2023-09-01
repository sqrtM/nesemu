use std::{error::Error, thread};
use std::time::Duration;

use crate::bus::Bus;
use crate::cpu::CPU;
use crate::op_code::Opcode;
use crate::op_code::Opcode::{BRK, XXX};

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

fn main() {
    let bus = Bus::default();
    let mut cpu = CPU::default();

    cpu.bus = Some(Box::new(bus));
    cpu.write(1, 16);

    cpu.reset();

    let mut code: Opcode = XXX;
    while code != BRK {
        let _ = cpu.clock();
        thread::sleep(Duration::from_millis(20));
        println!("{:?}", cpu.lookup(cpu.opcode).opcode);
        println!("{:X?}", &cpu.bus.as_mut().expect("").ram[0..6]);
        println!("[{:X?}, {:X?}, {:X?}, {:X?}, {:X?}, {:X?}]", &cpu.read(0, false), &cpu.read(1, false), &cpu.read(2, false), &cpu.read(3, false), &cpu.read(4, false), &cpu.read(5, false));
        code = cpu.lookup(cpu.opcode).opcode;
    }

    //gui::render(&r).expect("TODO: panic message");
}
