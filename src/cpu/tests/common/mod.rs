use nesemu_cpu::cpu::CPU;
use crate::common::fake_bus::FakeBus;

mod fake_bus;

pub fn setup() -> CPU<FakeBus> {
    let mut bus = FakeBus::default();
    bus.ram[0] = 0b1101;
    let mut cpu: CPU<FakeBus>= CPU::default();
    cpu.bus = Some(Box::new(bus));
    return cpu;
}
