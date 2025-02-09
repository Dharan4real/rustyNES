
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

#[derive(Debug, PartialEq)]
struct Instruction {
    name: String,
    opcode: Opcode,
    addr_mode: AddressingMode,
    cycles: u8 
}

//Addressing modes
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Relative,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X,
    Indirect_Y
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Adc,    //Add with carry
    And,    //Logical AND
    Asl,    //Arithmetic shift left
    Bcc,    //Branch if carry clear
    Bcs,    //Branch if carry set
    Beq,    //Branch if equal
    Bit,    //Bit test
    Bmi,    //Branch if minus
    Bne,    //Branch if not equal
    Bpl,    //Branch if Positive
    Brk,    //Break //Force interrupt
    Bvc,    //Branch if overflow clear
    Bvs,    //Branch if overflow set
    Clc,    //Clear carry flag
    Cld,    //Clear decimal mode
    Cli,    //Clear interrupt disable
    Clv,    //Clear overflow flag
    Cmp,    //Compare
    Cpx,    //Compare X register
    Cpy,    //Compare Y register
    Dec,    //Decrement memory
    Dex,    //Decrement X register
    Dey,    //Decrement Y register
    Eor,    //Exclusive OR
    Inc,    //Increment memory
    Inx,    //Increment X register
    Iny,    //INCREMENT Y register
    Jmp,    //Jump
    Jsr,    //Jump to subroutine
    Lda,    //Load accumulator
    Ldx,    //Load X register
    Ldy,    //Load Y register
    Lsr,    //Logical shift right
    Nop,    //No operation
    Ora,    //Logical Inclusive OR
    Pha,    //Push Accumulator
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
    Kil
}

impl Instruction {
    fn new(name: String, opcode: Opcode, addr_mode: AddressingMode, cycles: u8) -> Self {
        Instruction {
            name,
            opcode,
            addr_mode,
            cycles
        }
    }
}

#[test]
fn cleck_instruction_creation() {
    let inst = Instruction::new(String::from("LDA"), Opcode::Lda, AddressingMode::Absolute, 3);

    assert_eq!(inst, Instruction { name: String::from("LDA"), opcode: Opcode::Lda, addr_mode: AddressingMode::Absolute, cycles: 3 })
}