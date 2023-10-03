use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use nesemu_core::{Read, Write};
use crate::memory::CpuMemory;

pub struct Bus<Memory>
where
    Memory: Read + Write,
{
    pub ram: Option<Arc<Mutex<Memory>>>,
}

impl<Memory> Write for Bus<Memory>
where
    Memory: Read + Write,
{
    fn write(&mut self, addr: u16, data: u8) {
        self.ram
            .as_ref()
            .expect("RAM not found!")
            .lock()
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
            .as_ref()
            .expect("RAM not found!")
            .lock()
            .unwrap()
            .read(addr, false)
    }
}

impl<Memory> Bus<Memory>
where
    Memory: Read + Write,
{
    pub fn new() -> Self {
        Bus { ram: None }
    }

    pub fn connect_ram(&mut self, ram: Arc<Mutex<Memory>>) {
        self.ram = Some(ram.clone());
    }
}
