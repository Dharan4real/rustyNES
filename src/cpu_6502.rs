
pub mod opcodes;

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
    C = 1 << 0,
    Z = 1 << 1,
    I = 1 << 2,
    D = 1 << 3,
    B = 1 << 4,
    U = 1 << 5,
    O = 1 << 6,
    N = 1 << 7
}

impl Flags6502 {
    fn get_flag(&self) -> u8 { 0x00 }

    fn set_flag(&mut self, v: bool) {}
}

struct Instruction {
    name: String,
    operate: fn(&mut Cpu) -> u8,
    addr_mode: fn(&mut Cpu) -> u8,
    cycles: u8 
}

impl Instruction {
    // fn default(&self) -> Self {
    //     Instruction {
    //         name: String::new(),
    //         operate: 0,
    //         addr_mode: 0,
    //         0
    //     }
    // }
}

//Addressing modes
fn imp() -> u8 { 0 }
fn imm() -> u8 { 0 }
fn zp0() -> u8 { 0 }
fn zpx() -> u8 { 0 }
fn zpy() -> u8 { 0 }
fn rel() -> u8 { 0 }
fn abs() -> u8 { 0 }
fn abx() -> u8 { 0 }
fn aby() -> u8 { 0 }
fn ind() -> u8 { 0 }
fn inx() -> u8 { 0 }
fn iny() -> u8 { 0 }
