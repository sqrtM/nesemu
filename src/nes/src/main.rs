use crate::memory::CpuMemory;
use bus::Bus;
use nesemu_cpu::cpu::CPU;
use std::cell::RefCell;
use std::rc::Rc;

mod bus;
mod memory;

fn main() -> () {
    struct Nes {
        cpu: CPU<Bus<CpuMemory>>,
        bus: Bus<CpuMemory>,
        ram: Rc<RefCell<CpuMemory>>,
    }

    impl Nes {
        fn connect(&mut self) {
            //self.connect_ram();
            //self.connect_bus();
            self.cpu.reset();
        }

        //fn connect_ram(&mut self) {
           // self.bus.connect_ram(self.ram.clone());
        //}
        fn connect_bus(mut self) {
            self.cpu.connect_bus(Box::new(self.bus));
        }

        pub fn get_ram(&self) -> Rc<RefCell<CpuMemory>> {
            self.ram.clone()
        }
    }

    //let ram = Rc::new(RefCell::new(CpuMemory::new()));
    let mut bus: Bus<CpuMemory> = Bus::new();
    let mut cpu: CPU<Bus<CpuMemory>> = CPU::default();

    //let mut nes = Nes { cpu, bus, ram };

    //nes.connect();

    //nes.get_ram();
    //loop {
    //    cpu.clock();
    //    println!("{:?} ", cpu.lookup(cpu.opcode));
    //}
}
