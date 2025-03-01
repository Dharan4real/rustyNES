pub mod instruction;

extern crate fxhash;

use fxhash::FxHashMap;
use instruction::*;
use crate::bus::*;

const NME_BASE: u16 = 0xFFFA;
const RSR_BASE: u16 = 0xFFFC;
const IRQ_BASE: u16 = 0xFFFE;

pub struct Cpu {
    a_reg: u8,    // Accumulator Register
    x_reg: u8,    // X Register
    y_reg: u8,    // Y Register
    stk_ptr: u8, // Stack Pointer points to a location on the Bus
    pc: u16,       // Program Counter
    status: u8,   // Status Register
    bus: *mut Bus,
    fetched: u8,
    cycles_remaining: u8,
    clock_count: usize,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    next_instruction: Option<Instruction>,
}

//public
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a_reg: 0x00,
            x_reg: 0x00,
            y_reg: 0x00,
            stk_ptr: 0x00,
            pc: 0x0000,
            status: 0x00,
            bus: std::ptr::null_mut(),
            fetched: 0,
            cycles_remaining: 0,
            clock_count: 0,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            next_instruction: None,
        }
    }

    pub fn connect_to_bus(&mut self, bus_ptr: *mut Bus) {
        self.bus = bus_ptr;
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
        self.addr_abs = RSR_BASE;
        let lo: u16 = self.addr_abs + 0;
        let hi: u16 = self.addr_abs + 1;
        self.pc = hi << 8 | lo;

        self.a_reg = 0x00;
        self.x_reg = 0x00;
        self.y_reg = 0x00;
        self.stk_ptr = 0xFD;
        self.status = 0x00 | Flags6502::Unused as u8;

        self.addr_abs = 0x0000;
        self.addr_rel = 0x0000;

        self.fetched = 0x00;

        self.cycles_remaining = 8;
    }

    pub fn irq(&mut self) {
        if self.get_flag(Flags6502::InterruptDisable) == 0 {
            self.write(0x0100 + self.stk_ptr as u16, (self.pc >> 8) as u8 & 0x00FF);
            self.stk_ptr -= 1;
            self.write(0x0100 + self.stk_ptr as u16, self.pc as u8 & 0x00FF);
            self.stk_ptr -= 1;

            self.set_flag(Flags6502::BreakCommand, false);
            self.set_flag(Flags6502::InterruptDisable, true);
            self.set_flag(Flags6502::Unused, true);
            self.write(0x0000 + self.stk_ptr as u16, self.status);
            self.stk_ptr -= 1;

            self.addr_abs = IRQ_BASE;
            let lo: u16 = self.addr_abs + 0;
            let hi: u16 = self.addr_abs + 1;
            self.pc = hi << 8 | lo;
    
            self.cycles_remaining = 7;
        }
    }

    pub fn nmi(&mut self) {
        self.write(0x0100 + self.stk_ptr as u16, (self.pc >> 8) as u8 & 0x00FF);
            self.stk_ptr -= 1;
            self.write(0x0100 + self.stk_ptr as u16, self.pc as u8 & 0x00FF);
            self.stk_ptr -= 1;

            self.set_flag(Flags6502::BreakCommand, false);
            self.set_flag(Flags6502::InterruptDisable, true);
            self.set_flag(Flags6502::Unused, true);
            self.write(0x0000 + self.stk_ptr as u16, self.status);
            self.stk_ptr -= 1;

            self.addr_abs = NME_BASE;
            let lo: u16 = self.addr_abs + 0;
            let hi: u16 = self.addr_abs + 1;
            self.pc = hi << 8 | lo;
    
            self.cycles_remaining = 8;
    }

    pub fn clock(&mut self) {
        if self.cycles_remaining == 0 {
            self.opcode = self.read(self.pc);

            self.set_flag(Flags6502::Unused, true);

            self.pc += 1;

            self.cycles_remaining = CPU_INSTRUCTIONS[self.opcode as usize].cycles;

            let additional_cycle_addr_mode: u8 = (CPU_INSTRUCTIONS[self.opcode as usize].addr_mode).addr_mode_operation(self);

            let additional_cycle_opcode: u8 = (CPU_INSTRUCTIONS[self.opcode as usize].opcode).opcode_operation(self);

            self.cycles_remaining += additional_cycle_addr_mode & additional_cycle_opcode;

            self.set_flag(Flags6502::Unused, true);

            self.clock_count += 1;

            self.cycles_remaining -= 1;
        }
    }

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

    fn set_flag(&mut self, flag: Flags6502, v: bool) {
        if v {
            self.status |= flag as u8;
        }
    }

    fn unset_flag(&mut self, flag: Flags6502) {
        self.status &= !(flag as u8);
    }

    fn fetch(&mut self) {
        if !(CPU_INSTRUCTIONS[self.opcode as usize].addr_mode == AddressingMode::Implied) {
            self.fetched = self.read(self.addr_abs);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags() {
        let mut cpu = Cpu::new();

        cpu.set_flag(Flags6502::Carry, true);
        assert_eq!(cpu.get_flag(Flags6502::Carry),1);

        cpu.set_flag(Flags6502::Zero, true);
        assert_eq!(cpu.get_flag(Flags6502::Zero), 1);

        cpu.set_flag(Flags6502::InterruptDisable, true);
        assert_eq!(cpu.get_flag(Flags6502::InterruptDisable), 1);
        
        cpu.set_flag(Flags6502::DecimalMode, true);
        assert_eq!(cpu.get_flag(Flags6502::DecimalMode), 1);
        
        cpu.set_flag(Flags6502::BreakCommand, true);
        assert_eq!(cpu.get_flag(Flags6502::BreakCommand), 1);
        
        cpu.set_flag(Flags6502::Unused, true);
        assert_eq!(cpu.get_flag(Flags6502::Unused), 1);
        
        cpu.set_flag(Flags6502::Overflow, true);
        assert_eq!(cpu.get_flag(Flags6502::Overflow), 1);
        
        // cpu.set_flag(Flags6502::Negative, true);
        assert_eq!(cpu.get_flag(Flags6502::Negative), 0);
    }

    #[test]
    fn test_cpu_read_write() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new(&mut cpu);
        cpu.connect_to_bus(&mut bus);
        
        cpu.write(1, 21);
        assert_eq!(cpu.read(1), 21);
    }

    #[test]
    fn test_reset_function() {
        let mut cpu = Cpu::new();

        cpu.a_reg = 0x01;
        cpu.x_reg = 0x10;
        cpu.y_reg = 0xDA;

        cpu.reset();

        assert_eq!((cpu.a_reg, cpu.x_reg, cpu.y_reg), (0x00, 0x00, 0x00));
        assert_eq!(cpu.stk_ptr, 0xFD);
        assert_eq!(cpu.cycles_remaining, 8);
    }
}