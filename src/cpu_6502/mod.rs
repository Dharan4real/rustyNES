
pub mod instruction;

extern crate fxhash;
use fxhash::FxHashMap;

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

    pub fn reset() {}
    
    pub fn interrupt_req() {}
    
    pub fn non_maskable_interrupt_req() {}
    
    pub fn clock() {}
    
    pub fn is_complete() -> bool { false }
    
    pub fn disassemble(start: u16, stop: u16) -> FxHashMap<u16, String> {
        let mut map: FxHashMap<u16, String> = FxHashMap::default();
        
        map
    }
}

enum Flags6502 {
    Carry = 1 << 0,
    Zero = 1 << 1,
    InterruptDisable = 1 << 2,
    DecimalMode = 1 << 3,
    BreakCommand = 1 << 4,      //no CPU effect
    Xxx = 1 << 5,               //no CPU effect
    Overflow = 1 << 6,
    Negative = 1 << 7
}

impl Flags6502 {
    fn get_flag(&self) -> u8 { 0x00 }

    fn set_flag(&mut self, v: bool) {}
}