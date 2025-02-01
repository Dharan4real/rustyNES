
use crate::bus::Bus;

pub struct Cpu {
    a_reg: u8,          // Accumulator Register
    x_reg: u8,          // X Register
    y_reg: u8,          // Y Register
    stk_ptr: u16,       // Stack Pointer points to a location on the Bus
    pc: u8,             // Program Counter
    status: u8          // Status Register
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a_reg: 0x00,
            x_reg: 0x00,
            y_reg: 0x00,
            stk_ptr:0x0000,
            pc: 0x00,
            status: 0x00,
        }
    }
}
enum Flags6502 {
    C = 1 << 0,
    Z = 1 << 1,
    I = 1 << 2,
    D = 1 << 3,
    B = 1 << 4,
    U = 1 << 5,
    O = 1 << 6,
    N = 1 << 7
}