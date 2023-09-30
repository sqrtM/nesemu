use std::cell::RefCell;
use std::rc::Rc;

use nesemu_core::{Read, Write};

pub struct Bus<Memory>
where
    Memory: Read + Write,
{
    pub ram: Option<Rc<RefCell<Memory>>>,
}

impl<Memory> Write for Bus<Memory>
where
    Memory: Read + Write,
{
    fn write(&mut self, addr: u16, data: u8) {
        self.ram
            .as_ref()
            .expect("RAM not found!")
            .borrow_mut()
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
            .borrow()
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

    pub fn connect_ram(&mut self, ram: Rc<RefCell<Memory>>) {
        self.ram = Some(ram.clone());
    }
}
