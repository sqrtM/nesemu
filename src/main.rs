mod cpu;
mod bus;
mod instruction;

pub trait Read {
    fn read(&self, addr: u16, _read_only: bool) -> u8;
}
pub trait Write {
    fn write(&mut self, addr: u16, data: u8) -> ();
}


fn main() {
    println!("Hello, world!");
}
