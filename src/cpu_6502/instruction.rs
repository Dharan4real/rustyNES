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
    Accumulator,
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
    pub static ref  CPU_INSTRUCTIONS: [Instruction; 256] = [
        Instruction::new( "BRK", Opcode::Brk, AddressingMode::Implied, 7 ),Instruction::new( "ORA", Opcode::Ora, AddressingMode::Indirect_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "ORA", Opcode::Ora, AddressingMode::ZeroPage, 3 ),Instruction::new( "ASL", Opcode::Asl, AddressingMode::ZeroPage, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "PHP", Opcode::Php, AddressingMode::Implied, 3 ),Instruction::new( "ORA", Opcode::Ora, AddressingMode::Immediate, 2 ),Instruction::new( "ASL", Opcode::Asl, AddressingMode::Accumulator, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "ORA", Opcode::Ora, AddressingMode::Absolute, 4 ),Instruction::new( "ASL", Opcode::Asl, AddressingMode::Absolute, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "BPL", Opcode::Bpl, AddressingMode::Relative, 2 ),Instruction::new( "ORA", Opcode::Ora, AddressingMode::Indirect_Y, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "ORA", Opcode::Ora, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "ASL", Opcode::Asl, AddressingMode::ZeroPage_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CLC", Opcode::Clc, AddressingMode::Implied, 2 ),Instruction::new( "ORA", Opcode::Ora, AddressingMode::Absolute_Y, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "ORA", Opcode::Ora, AddressingMode::Absolute_X, 4 ),Instruction::new( "ASL", Opcode::Asl, AddressingMode::Absolute_X, 7 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "JSR", Opcode::Jsr, AddressingMode::Absolute, 6 ),Instruction::new( "AND", Opcode::And, AddressingMode::Indirect_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "BIT", Opcode::Bit, AddressingMode::ZeroPage, 3 ),Instruction::new( "AND", Opcode::And, AddressingMode::ZeroPage, 3 ),Instruction::new( "ROL", Opcode::Rol, AddressingMode::ZeroPage, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "PLP", Opcode::Plp, AddressingMode::Implied, 4 ),Instruction::new( "AND", Opcode::And, AddressingMode::Immediate, 2 ),Instruction::new( "ROL", Opcode::Rol, AddressingMode::Accumulator, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "BIT", Opcode::Bit, AddressingMode::Absolute, 4 ),Instruction::new( "AND", Opcode::And, AddressingMode::Absolute, 4 ),Instruction::new( "ROL", Opcode::Rol, AddressingMode::Absolute, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "BMI", Opcode::Bmi, AddressingMode::Relative, 2 ),Instruction::new( "AND", Opcode::And, AddressingMode::Indirect_Y, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "AND", Opcode::And, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "ROL", Opcode::Rol, AddressingMode::ZeroPage_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "SEC", Opcode::Sec, AddressingMode::Implied, 2 ),Instruction::new( "AND", Opcode::And, AddressingMode::Absolute_Y, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "AND", Opcode::And, AddressingMode::Absolute_X, 4 ),Instruction::new( "ROL", Opcode::Rol, AddressingMode::Absolute_X, 7 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "RTI", Opcode::Rti, AddressingMode::Implied, 6 ),Instruction::new( "EOR", Opcode::Eor, AddressingMode::Indirect_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "EOR", Opcode::Eor, AddressingMode::ZeroPage, 3 ),Instruction::new( "LSR", Opcode::Lsr, AddressingMode::ZeroPage, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "PHA", Opcode::Pha, AddressingMode::Implied, 3 ),Instruction::new( "EOR", Opcode::Eor, AddressingMode::Immediate, 2 ),Instruction::new( "LSR", Opcode::Lsr, AddressingMode::Accumulator, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "JMP", Opcode::Jmp, AddressingMode::Absolute, 3 ),Instruction::new( "EOR", Opcode::Eor, AddressingMode::Absolute, 4 ),Instruction::new( "LSR", Opcode::Lsr, AddressingMode::Absolute, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "BVC", Opcode::Bvc, AddressingMode::Relative, 2),Instruction::new( "EOR", Opcode::Eor, AddressingMode::Indirect_Y, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "EOR", Opcode::Eor, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "LSR", Opcode::Lsr, AddressingMode::ZeroPage_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CLI", Opcode::Cli, AddressingMode::Implied, 2 ),Instruction::new( "EOR", Opcode::Eor, AddressingMode::Absolute_Y, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "EOR", Opcode::Eor, AddressingMode::Absolute_X, 4 ),Instruction::new( "LSR", Opcode::Lsr, AddressingMode::Absolute_X, 7 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "RTS", Opcode::Rts, AddressingMode::Implied, 6 ),Instruction::new( "ADC", Opcode::Adc, AddressingMode::Indirect_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "ADC", Opcode::Adc, AddressingMode::ZeroPage, 3 ),Instruction::new( "ROR", Opcode::Ror, AddressingMode::ZeroPage, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "PLA", Opcode::Pla, AddressingMode::Implied, 4 ),Instruction::new( "ADC", Opcode::Adc, AddressingMode::Immediate, 2 ),Instruction::new( "ROR", Opcode::Ror, AddressingMode::Accumulator, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "JMP", Opcode::Jmp, AddressingMode::Indirect, 3 ),Instruction::new( "ADC", Opcode::Adc, AddressingMode::Absolute, 4 ),Instruction::new( "ROR", Opcode::Ror, AddressingMode::Absolute, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "BVS", Opcode::Bvs, AddressingMode::Relative, 2 ),Instruction::new( "ADC", Opcode::Adc, AddressingMode::Indirect_Y, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "ADC", Opcode::Adc, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "ROR", Opcode::Ror, AddressingMode::ZeroPage_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "SEI", Opcode::Sei, AddressingMode::Implied, 2 ),Instruction::new( "ADC", Opcode::Adc, AddressingMode::Absolute_Y, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "ADC", Opcode::Adc, AddressingMode::Absolute_X, 4 ),Instruction::new( "ROR", Opcode::Ror, AddressingMode::Absolute_X, 7 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "STA", Opcode::Sta, AddressingMode::Indirect_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "STY", Opcode::Sty, AddressingMode::ZeroPage, 3 ),Instruction::new( "STA", Opcode::Sta, AddressingMode::ZeroPage, 3 ),Instruction::new( "STX", Opcode::Stx, AddressingMode::ZeroPage, 3 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "DEY", Opcode::Dey, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "TXA", Opcode::Txa, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "STY", Opcode::Sty, AddressingMode::Absolute, 4 ),Instruction::new( "STA", Opcode::Sta, AddressingMode::Absolute, 4 ),Instruction::new( "STX", Opcode::Stx, AddressingMode::Absolute, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "BCC", Opcode::Bcc, AddressingMode::Relative, 2 ),Instruction::new( "STA", Opcode::Sta, AddressingMode::Indirect_Y, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "STY", Opcode::Sty, AddressingMode::ZeroPage_Y, 4 ),Instruction::new( "STA", Opcode::Sta, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "STX", Opcode::Stx, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "TYA", Opcode::Tya, AddressingMode::Implied, 2 ),Instruction::new( "STA", Opcode::Sta, AddressingMode::Absolute_Y, 5 ),Instruction::new( "TXS", Opcode::Txs, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "STA", Opcode::Sta, AddressingMode::Absolute_X, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "LDY", Opcode::Ldy, AddressingMode::Immediate, 2 ),Instruction::new( "LDA", Opcode::Lda, AddressingMode::Indirect_X, 6 ),Instruction::new( "LDX", Opcode::Ldx, AddressingMode::Immediate, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "LDY", Opcode::Ldy, AddressingMode::ZeroPage, 3 ),Instruction::new( "LDA", Opcode::Lda, AddressingMode::ZeroPage, 3 ),Instruction::new( "LDX", Opcode::Ldx, AddressingMode::ZeroPage, 3 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "TAY", Opcode::Tay, AddressingMode::Implied, 2 ),Instruction::new( "LDA", Opcode::Lda, AddressingMode::Immediate, 2 ),Instruction::new( "TAX", Opcode::Tax, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "LDY", Opcode::Ldy, AddressingMode::Absolute, 4 ),Instruction::new( "LDA", Opcode::Lda, AddressingMode::Absolute, 4 ),Instruction::new( "LDX", Opcode::Ldx, AddressingMode::Absolute, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "BCS", Opcode::Bcs, AddressingMode::Relative, 2 ),Instruction::new( "LDA", Opcode::Lda, AddressingMode::Indirect_Y, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "LDY", Opcode::Ldy, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "LDA", Opcode::Lda, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "LDX", Opcode::Ldx, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CLV", Opcode::Clv, AddressingMode::Implied, 2 ),Instruction::new( "LDA", Opcode::Lda, AddressingMode::Absolute_Y, 4 ),Instruction::new( "TSX", Opcode::Tsx, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "LDY", Opcode::Ldy, AddressingMode::Absolute_X, 4 ),Instruction::new( "LDA", Opcode::Lda, AddressingMode::Absolute_X, 4 ),Instruction::new( "LDX", Opcode::Ldx, AddressingMode::Absolute_Y, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "CPY", Opcode::Cpy, AddressingMode::Immediate, 2 ),Instruction::new( "CMP", Opcode::Cmp, AddressingMode::Indirect_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CPY", Opcode::Cpy, AddressingMode::ZeroPage, 3 ),Instruction::new( "CMP", Opcode::Cmp, AddressingMode::ZeroPage, 3 ),Instruction::new( "DEC", Opcode::Dec, AddressingMode::ZeroPage, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "INY", Opcode::Iny, AddressingMode::Implied, 2 ),Instruction::new( "CMP", Opcode::Cmp, AddressingMode::Immediate, 2 ),Instruction::new( "DEX", Opcode::Dex, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CPY", Opcode::Cpy, AddressingMode::Absolute, 4 ),Instruction::new( "CMP", Opcode::Cmp, AddressingMode::Absolute, 4 ),Instruction::new( "DEC", Opcode::Dec, AddressingMode::Absolute, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "BNE", Opcode::Bne, AddressingMode::Relative, 2 ),Instruction::new( "CMP", Opcode::Cmp, AddressingMode::Indirect_Y, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CMP", Opcode::Cmp, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "DEC", Opcode::Dec, AddressingMode::ZeroPage_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CLD", Opcode::Cld, AddressingMode::Implied, 2 ),Instruction::new( "CMP", Opcode::Cmp, AddressingMode::Absolute_Y, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CMP", Opcode::Cmp, AddressingMode::Absolute_X, 4 ),Instruction::new( "DEC", Opcode::Dec, AddressingMode::Absolute_X, 7 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "CPX", Opcode::Cpx, AddressingMode::Immediate, 2 ),Instruction::new( "SBC", Opcode::Sbc, AddressingMode::Indirect_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CPX", Opcode::Cpx, AddressingMode::ZeroPage, 3 ),Instruction::new( "SBC", Opcode::Sbc, AddressingMode::ZeroPage, 3 ),Instruction::new( "INC", Opcode::Inc, AddressingMode::ZeroPage, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "INX", Opcode::Inx, AddressingMode::Implied, 2 ),Instruction::new( "SBC", Opcode::Sbc, AddressingMode::Immediate, 2 ),Instruction::new( "NOP", Opcode::Nop, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "CPX", Opcode::Cpx, AddressingMode::Absolute, 4 ),Instruction::new( "SBC", Opcode::Sbc, AddressingMode::Absolute, 4 ),Instruction::new( "INC", Opcode::Inc, AddressingMode::Absolute, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),
        Instruction::new( "BEQ", Opcode::Beq, AddressingMode::Relative, 2 ),Instruction::new( "SBC", Opcode::Sbc, AddressingMode::Indirect_Y, 5 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "SBC", Opcode::Sbc, AddressingMode::ZeroPage_X, 4 ),Instruction::new( "INC", Opcode::Inc, AddressingMode::ZeroPage_X, 6 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "SED", Opcode::Sed, AddressingMode::Implied, 2 ),Instruction::new( "SBC", Opcode::Sbc, AddressingMode::Absolute_Y, 4 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),Instruction::new( "SBC", Opcode::Sbc, AddressingMode::Absolute_X, 4 ),Instruction::new( "INC", Opcode::Inc, AddressingMode::Absolute_X, 7 ),Instruction::new( "KIL", Opcode::Kil, AddressingMode::Implied, 2 ),   
    ];
}


mod tests {
    use super::*;

    #[test]
    fn cleck_cpu_instructions() {
        let inst = Instruction::new( "BRK", Opcode::Brk, AddressingMode::Implied, 7 );    
        assert_eq!(inst, CPU_INSTRUCTIONS[0]);
        let inst = Instruction::new( "ORA", Opcode::Ora, AddressingMode::Indirect_X, 6 );    
        assert_eq!(inst, CPU_INSTRUCTIONS[1]);
        let inst = Instruction::new( "STA", Opcode::Sta, AddressingMode::Indirect_X, 6 );    
        assert_eq!(inst, CPU_INSTRUCTIONS[129]);
    }
}