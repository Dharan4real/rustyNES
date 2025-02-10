use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub name: &'static str,
    pub opcode: Opcode,
    pub addr_mode: AddressingMode,
    pub cycles: u8 
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
    fn new(name: &'static str, opcode: Opcode, addr_mode: AddressingMode, cycles: u8) -> Self {
        Instruction {
            name,
            opcode,
            addr_mode,
            cycles
        }
    }
    
}

lazy_static!{
    pub static ref  CPU_INSTRUCTIONS: Vec<Instruction> = vec![
        Instruction::new( "BRK", Opcode::Brk, AddressingMode::Implied, 7 ),
        Instruction::new( "BPL", Opcode::Bpl, AddressingMode::Relative, 2 ),
        Instruction::new( "JSR", Opcode::Jsr, AddressingMode::Absolute, 6 ),
        Instruction::new( "BMI", Opcode::Bmi, AddressingMode::Relative, 2 ),
        Instruction::new( "RTI", Opcode::Rti, AddressingMode::Implied, 6 ),
        Instruction::new( "BVC", Opcode::Bvc, AddressingMode::Relative, 2),
        Instruction::new( "RTS", Opcode::Rts, AddressingMode::Implied, 6 ),
        Instruction::new( "BVS", Opcode::Bvs, AddressingMode::Relative, 2 ),
        Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "BCC", Opcode::Bcc, AddressingMode::Relative, 2 ),
        Instruction::new( "LDY", Opcode::Ldy, AddressingMode::Immediate, 2 ),
        Instruction::new( "BCS", Opcode::Bcs, AddressingMode::Relative, 2 ),
        Instruction::new( "CPY", Opcode::Cpy, AddressingMode::Immediate, 2 ),
        Instruction::new( "BNE", Opcode::Bne, AddressingMode::Relative, 2 ),
        Instruction::new( "CPX", Opcode::Cpx, AddressingMode::Immediate, 2 ),
        Instruction::new( "BEQ", Opcode::Beq, AddressingMode::Relative, 2 ),

        Instruction::new( "ORA", Opcode::Ora, AddressingMode::Indirect_X, 6 ),
        Instruction::new( "ORA", Opcode::Ora, AddressingMode::Indirect_Y, 5 ),
        Instruction::new( "AND", Opcode::And, AddressingMode::Indirect_X, 6 ),


        Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
    ];
}


mod tests {
    use super::*;

    #[test]
    fn cleck_cpu_instructions() {
        let inst = Instruction::new( "BRK", Opcode::Brk, AddressingMode::Implied, 7 );    
        assert_eq!(inst, CPU_INSTRUCTIONS[0]);
        let inst = Instruction::new( "ORA", Opcode::Ora, AddressingMode::Indirect_X, 6 );    
        assert_eq!(inst, CPU_INSTRUCTIONS[16]);
        let inst = Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 );    
        assert_eq!(inst, CPU_INSTRUCTIONS[19]);
    }
}