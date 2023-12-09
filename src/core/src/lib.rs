use std::sync::{Arc, Mutex};

pub trait Read {
    fn read(&self, addr: u16, _read_only: bool) -> u8;
}

pub trait Write {
    fn write(&mut self, addr: u16, data: u8);
}

pub trait ReadFn {
    fn read(&self, addr: u16, read_only: bool) -> u8;
    fn clone_box(&self) -> Box<dyn ReadFn>;
}

impl<F: Fn(u16, bool) -> u8 + Clone + 'static> ReadFn for F {
    fn read(&self, addr: u16, read_only: bool) -> u8 {
        (*self)(addr, read_only)
    }

    fn clone_box(&self) -> Box<dyn ReadFn> {
        Box::new(self.clone())
    }
}

pub trait WriteFn {
    fn write(&mut self, addr: u16, data: u8);
    fn clone_box(&self) -> Box<dyn WriteFn>;
}
impl<F: FnMut(u16, u8) + Clone + 'static> WriteFn for F {
    fn write(&mut self, addr: u16, data: u8) {
        (*self)(addr, data)
    }

    fn clone_box(&self) -> Box<dyn WriteFn> {
        Box::new(self.clone())
    }
}
