use crate::bus::Bus;
use crate::memory::CpuMemory;
use nesemu_cpu::cpu::CPU;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub mod bus;
pub mod memory;

pub fn startup() -> () {}
