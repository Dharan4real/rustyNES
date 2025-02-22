pub mod instruction;

extern crate fxhash;
use fxhash::FxHashMap;
use instruction::*;
use crate::bus::*;

pub struct Cpu {
    a_reg: u8,    // Accumulator Register
    x_reg: u8,    // X Register
    y_reg: u8,    // Y Register
    stk_ptr: u16, // Stack Pointer points to a location on the Bus
    pc: u8,       // Program Counter
    status: u8,   // Status Register
    fetched: u8,
    cycles_remaining: u8,
    clock_count: usize,
    bus: *mut Bus,
    next_instruction: Option<Instruction>,
}

//public
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a_reg: 0x00,
            x_reg: 0x00,
            y_reg: 0x00,
            stk_ptr: 0x0000,
            pc: 0x00,
            status: 0x00,

            fetched: 0,
            cycles_remaining: 0,
            clock_count: 0,
            bus: std::ptr::null_mut(),
            next_instruction: None,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        unsafe {
            return (*self.bus).read(addr, true);
        }
    }
    
    pub fn write(&self, addr: u16, data: u8) {
        unsafe {
            (*self.bus).write(addr, data);
        }
    }

    pub fn reset(&mut self) {
        self.a_reg = 0x00;
    }

    pub fn irq() {}

    pub fn nmi() {}

    pub fn clock() {}

    pub fn is_complete() -> bool {
        false
    }

    pub fn disassemble(start: u16, stop: u16) -> FxHashMap<u16, String> {
        let mut map: FxHashMap<u16, String> = FxHashMap::default();

        map
    }
}

//private
impl Cpu {
    fn get_flag(&self, flag: Flags6502) -> u8 {
        if (self.status & flag as u8) > 0 {
            return 1;
        } 
        else {
            return 0;
        }
    }

    fn set_flag(&mut self, flag: Flags6502) {
        self.status |= flag as u8;
    }

    fn unset_flag(&mut self, flag: Flags6502) {
        self.status &= !(flag as u8);
    }
    
    fn set_flag_status(&mut self, flag: Flags6502, v: bool) {
        if v {
            self.set_flag(flag);
        }
        else {
            self.unset_flag(flag);
        }
    }
}

pub enum Flags6502 {
    Carry = 1 << 0,
    Zero = 1 << 1,
    InterruptDisable = 1 << 2,
    DecimalMode = 1 << 3,
    BreakCommand = 1 << 4, //no CPU effect
    Unused = 1 << 5,          //no CPU effect
    Overflow = 1 << 6,
    Negative = 1 << 7,
}

mod tests {
    use super::*;

    #[test]
    fn test_flags() {
        let mut cpu = Cpu::new();

        cpu.set_flag_status(Flags6502::Carry, true);
        assert_eq!(cpu.get_flag(Flags6502::Carry),1);

        cpu.set_flag_status(Flags6502::Zero, true);
        assert_eq!(cpu.get_flag(Flags6502::Zero), 1);

        cpu.set_flag_status(Flags6502::InterruptDisable, true);
        assert_eq!(cpu.get_flag(Flags6502::InterruptDisable), 1);
        
        cpu.set_flag_status(Flags6502::DecimalMode, true);
        assert_eq!(cpu.get_flag(Flags6502::DecimalMode), 1);
        
        cpu.set_flag_status(Flags6502::BreakCommand, true);
        assert_eq!(cpu.get_flag(Flags6502::BreakCommand), 1);
        
        cpu.set_flag_status(Flags6502::Unused, true);
        assert_eq!(cpu.get_flag(Flags6502::Unused), 1);
        
        cpu.set_flag_status(Flags6502::Overflow, true);
        assert_eq!(cpu.get_flag(Flags6502::Overflow), 1);
        
        // cpu.set_flag_status(Flags6502::Negative, true);
        assert_eq!(cpu.get_flag(Flags6502::Negative), 0);
    }
}