mod bus;

use std::thread;
use std::time::Duration;

use bus::Bus;
use nesemu_cpu::{Write, Read};
use nesemu_cpu::cpu::CPU;
use nesemu_cpu::op_code::Opcode;

/// https://youtu.be/8XmxKPJDGU0?t=1692

fn main() {
    let bus = Bus::default();
    let mut cpu: CPU<_> = CPU::default();

    cpu.bus = Some(Box::new(bus));
    cpu.write(1, 16);

    cpu.reset();

    let mut code: Opcode = Opcode::XXX;
    while code != Opcode::BRK {
        let _ = cpu.clock();
        thread::sleep(Duration::from_millis(20));
        println!("{:?}", cpu.lookup(cpu.opcode).opcode);
        println!("{:X?}", &cpu.bus.as_mut().expect("").ram[0..6]);
        println!("[{:X?}, {:X?}, {:X?}, {:X?}, {:X?}, {:X?}]", &cpu.read(0, false), &cpu.read(1, false), &cpu.read(2, false), &cpu.read(3, false), &cpu.read(4, false), &cpu.read(5, false));
        code = cpu.lookup(cpu.opcode).opcode;
    }
}
