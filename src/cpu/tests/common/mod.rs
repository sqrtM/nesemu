use crate::common::fake_bus::FakeBus;
use nesemu_cpu::cpu::CPU;

mod fake_bus;

pub fn setup(val: u8) -> CPU<FakeBus> {
    let mut bus = FakeBus::default();
    bus.ram[0] = val;
    let mut cpu = CPU::default();
    cpu.connect_bus(Box::new(bus));
    cpu
}
