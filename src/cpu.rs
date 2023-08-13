use crate::{Read, Write};
use crate::bus::Bus;

pub struct CPU<'a> {
    pub bus: Option<&'a Bus<'a>>,

    acc_reg: u8,
    x_reg: u8,
    y_reg: u8,
    stk_ptr: u8,
    pgrm_ctr: u16,
    status: u8,

    fetched: u8,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
}

enum StatusFlags {
    C = 1 << 0,
    Z = 1 << 1,
    I = 1 << 2,
    D = 1 << 3,
    B = 1 << 4,
    U = 1 << 5,
    V = 1 << 6,
    N = 1 << 7,
}

pub enum AddressingMode {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY,
}

pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,

    XXX
}

impl Read for CPU {
    fn read(&self, addr: u16, _read_only: bool) -> u8 {
        self.bus.expect("no bus connected").read(addr, _read_only)
    }
}

impl Write for CPU {
    fn write(&self, addr: u16, data: u8) -> () {
        self.bus.expect("no bus connected").write(addr, data)
    }
}

impl CPU {
    pub fn connect_bus(&mut self, n: &Bus) {
        self.bus = Option::from(n)
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            bus: None,
            acc_reg: 0,
            x_reg: 0,
            y_reg: 0,
            stk_ptr: 0,
            pgrm_ctr: 0,
            status: 0,
            fetched: 0,
            addr_abs: 0,
            addr_rel: 0,
            opcode: 0,
            cycles: 0,
        }
    }
}
























































