use bus::Bus;
use nesemu_cpu::cpu::CPU;

mod bus;

fn main() {
    let mut bus = Bus::default();
    bus.load();
    let mut cpu: CPU<_> = CPU::default();

    cpu.bus = Some(Box::new(bus));
    cpu.reset();

    loop {
        cpu.clock();
        println!("{:?} ", cpu.lookup(cpu.opcode));
    }
}
