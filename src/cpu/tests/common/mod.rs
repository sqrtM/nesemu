use crate::common::fake_bus::FakeBus;
use nesemu_cpu::cpu::CPU;
use std::sync::{Arc, RwLock};

mod fake_bus;

pub fn setup(val: u8) -> CPU<FakeBus> {
    let mut bus = FakeBus::default();
    bus.ram[0] = val;
    CPU::new(Arc::new(RwLock::new(bus)))
}
