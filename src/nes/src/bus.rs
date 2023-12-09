use std::sync::{Arc, Mutex};

use nesemu_core::{Read, ReadFn, Write, WriteFn};

pub struct Bus {
    read: Mutex<Option<Box<dyn ReadFn>>>,
    write: Mutex<Option<Box<dyn WriteFn>>>,
}

impl Write for Bus {
    fn write(&mut self, addr: u16, data: u8) {
        let mut write_closure = self
            .write
            .lock()
            .unwrap()
            .take()
            .expect("No write closure set");
        write_closure.write(addr, data);
    }
}

impl Read for Bus {
    fn read(&self, addr: u16, _read_only: bool) -> u8 {
        let read_closure = self
            .read
            .lock()
            .unwrap()
            .take()
            .expect("No read closure set");
        read_closure.read(addr, _read_only)
    }
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            read: Mutex::new(None),
            write: Mutex::new(None),
        }
    }

    pub fn set_read(&self, r: Box<dyn ReadFn>) {
        *self.read.lock().unwrap() = Some(r);
    }

    pub fn set_write(&self, w: Box<dyn WriteFn>) {
        *self.write.lock().unwrap() = Some(w);
    }
}
