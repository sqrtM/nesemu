pub mod op_code;
pub mod instruction;
pub mod cpu;
pub mod addressing_mode;

pub trait Read {
    fn read(&self, addr: u16, _read_only: bool) -> u8;
}

pub trait Write {
    fn write(&mut self, addr: u16, data: u8) -> ();
}
