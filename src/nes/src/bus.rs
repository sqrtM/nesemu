use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

use nesemu_core::{Read, Write};
use crate::memory::CpuMemory;

pub struct Bus<Memory>
where
    Memory: Read + Write,
{
    pub ram: Arc<RwLock<Memory>>,
}

impl<Memory> Write for Bus<Memory>
where
    Memory: Read + Write,
{
    fn write(&mut self, addr: u16, data: u8) {
        self.ram
            .write()
            .unwrap()
            .write(addr, data)
    }
}

impl<Memory> Read for Bus<Memory>
where
    Memory: Read + Write,
{
    fn read(&self, addr: u16, _read_only: bool) -> u8 {
        self.ram
            .read()
            .unwrap()
            .read(addr, false)
    }
}

impl<Memory> Bus<Memory>
where
    Memory: Read + Write,
{
    pub fn new(ram: Arc<RwLock<Memory>>) -> Self {
        Bus { ram }
    }
}
