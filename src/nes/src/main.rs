mod bus;

use bus::Bus;
use nesemu_core::Write;
use nesemu_cpu::cpu::{
    StatusFlag::{C, D, I, N, V, Z},
    CPU,
};

/// https://youtu.be/8XmxKPJDGU0?t=1692

fn main() {
    let bus = Bus::default();
    let mut cpu: CPU<_> = CPU::default();

    cpu.bus = Some(Box::new(bus));
    cpu.write(1, 16);

    cpu.reset();

    let mut i = 0;
    while i < 128 {
        let _ = cpu.clock();
        // if cpu.cycles == 0 {
        println!(
            "[ {:?}, {:?}, {:b} ]",
            cpu.lookup(cpu.opcode).opcode,
            cpu.lookup(cpu.opcode).addressing_mode,
            cpu.status
        );
        println!(
            "[ {:X} :: {}, {}, {}, {}, {}]",
            cpu.pgrm_ctr, cpu.acc_reg, cpu.x_reg, cpu.y_reg, cpu.status, cpu.stk_ptr
        );
        println!(
            "[ N:{} V:{} D:{} I:{} Z:{} C:{}]",
            cpu.get_flag(N),
            cpu.get_flag(V),
            cpu.get_flag(D),
            cpu.get_flag(I),
            cpu.get_flag(Z), // error here ?
            cpu.get_flag(C),
        );
        i += 1;
        // }
    }
}
