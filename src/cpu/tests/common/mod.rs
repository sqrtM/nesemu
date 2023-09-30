use nesemu_cpu::cpu::CPU;
use crate::common::fake_bus::FakeBus;

mod fake_bus;

pub fn setup(val: u8) -> CPU<FakeBus> {
    let mut bus = FakeBus::default();
    bus.ram[0] = val;
    let mut cpu: CPU<FakeBus>= CPU::default();
    cpu.bus = Some(Box::new(bus));
    return cpu;
}
