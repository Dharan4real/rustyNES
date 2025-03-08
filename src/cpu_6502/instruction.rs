use std::fmt::Display;

use lazy_static::lazy_static;
use super::*;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub addr_mode: AddressingMode,
    pub cycles: u8 
}

impl Instruction {
    fn new(opcode: Opcode, addr_mode: AddressingMode, cycles: u8) -> Self {
        Instruction {
            opcode,
            addr_mode,
            cycles
        }
    }    
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

impl AddressingMode {
    pub fn addr_mode_operation(&self, cpu: &mut Cpu) -> u8 {
        use self::AddressingMode::*;

        match self {
            Implied => { 
                cpu.fetched = cpu.a_reg;

                0
             }
             Immediate => {
                cpu.pc += 1;
                cpu.addr_abs = cpu.pc;

                0                
             }
             ZeroPage => {
                cpu.addr_abs = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;
                cpu.addr_abs &= 0x00FF;

                0
             }
             ZeroPage_X => {
                cpu.addr_abs = cpu.read(cpu.pc + (cpu.x_reg as u16)) as u16;
                cpu.pc += 1;
                cpu.addr_abs &= 0x00FF;

                0
             }
             ZeroPage_Y => {
                cpu.addr_abs = cpu.read(cpu.pc + (cpu.y_reg as u16)) as u16;
                cpu.pc += 1;
                cpu.addr_abs &= 0x00FF;

                0
             }
             Relative => {
                cpu.addr_rel = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;

                if cpu.addr_rel & 0x80 != 0 {
                    cpu.addr_rel |= 0xFF00;
                }

                0
             }
             Absolute => {
                let lo: u16 = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;
                let hi: u16 = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;

                cpu.addr_abs = hi << 8 | lo;

                0
             }
             Absolute_X => {
                let lo: u16 = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;
                let hi: u16 = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;

                cpu.addr_abs = (hi << 8 | lo) + cpu.x_reg as u16;

                if (cpu.addr_abs & 0xFF00) != (hi << 8) {
                    return 1;
                } 
                else {
                    return 0;
                }
             }
             Absolute_Y => {
                let lo: u16 = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;
                let hi: u16 = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;

                cpu.addr_abs = (hi << 8 | lo) + cpu.y_reg as u16;

                if (cpu.addr_abs & 0xFF00) != (hi << 8) {
                    return 1;
                } 
                else {
                    return 0;
                }
             }
             Indirect => {
                let ptr_lo: u16 = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;
                let ptr_hi: u16 = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;

                let ptr: u16 = (ptr_hi << 8) | ptr_lo;

                if ptr_lo == 0x00FF {
                    cpu.addr_abs = ((cpu.read(ptr & 0xFF00) as u16) << 8) | (cpu.read(ptr + 0) as u16);
                }
                else {
                    cpu.addr_abs = ((cpu.read(ptr + 1) as u16) << 8) | (cpu.read(ptr + 0) as u16);
                }

                0
             }
             Indirect_X => {
                let temp = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;

                let lo = cpu.read((temp + (cpu.x_reg as u16)) & 0x00FF) as u16;
                let hi = cpu.read((temp + (cpu.x_reg as u16) + 1) & 0x00FF) as u16;

                cpu.addr_abs = (hi << 8) | lo;

                1
             }
             Indirect_Y => {
                let temp = cpu.read(cpu.pc) as u16;
                cpu.pc += 1;

                let lo = cpu.read(temp & 0x00FF) as u16;
                let hi = cpu.read((temp + 1) & 0x00FF) as u16;

                cpu.addr_abs = ((hi << 8) | lo) + cpu.x_reg as u16;

                if cpu.addr_abs & 0xFF00 != (hi << 8) {
                    return 1;
                }
                else {
                    return 0;
                }
             }
        }
    }
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

impl Opcode {
    pub fn opcode_operation(&self, cpu: &mut Cpu) -> u8 {
        use self::Opcode::*;
        use self::AddressingMode::*;
        use Flags6502::*;

        match self {
            Adc => {
                cpu.fetch();

                let temp = cpu.a_reg as u16 + cpu.fetched as u16 + cpu.get_flag(Carry) as u16;

                cpu.set_flag(Carry, temp > 255);
                cpu.set_flag(Zero, (temp & 0x00FF) == 0);
                cpu.set_flag(Overflow, (((cpu.a_reg as u16 ^ temp) & !(cpu.a_reg as u16 ^ cpu.fetched as u16)) & 0x0080) != 0);
                cpu.set_flag(Negative, (temp & 0x80) != 0);

                cpu.a_reg = (temp & 0x00FF) as u8;

                1
            }
            And => {
                cpu.fetch();

                cpu.a_reg &= cpu.fetched;

                cpu.set_flag(Zero, cpu.a_reg == 0);
                cpu.set_flag(Negative, (cpu.a_reg & 0x80) != 0);          

                1
            }
            Asl => {
                cpu.fetch();

                let temp = (cpu.fetched as u16) << 1;

                cpu.set_flag(Carry, (temp & 0xFF00) > 0);
                cpu.set_flag(Zero, (temp & 0x00FF) == 0);
                cpu.set_flag(Negative, (temp & 0x80) != 0);

                if CPU_INSTRUCTIONS[cpu.opcode as usize].addr_mode == Implied {
                    cpu.a_reg = (temp & 0x00FF) as u8;
                }
                else {
                    cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);
                }

                0
            }
            Bcc => {
                if cpu.get_flag(Carry) == 0 {
                    cpu.cycles_remaining += 1;
                    cpu.addr_abs = cpu.pc + cpu.addr_rel;

                    if (cpu.addr_abs & 0xFF00) != (cpu.pc &0xFF00) {
                        cpu.cycles_remaining += 1;
                    }
                    
                    cpu.pc = cpu.addr_abs;
                }

                0
            }
            Bcs => {
                if cpu.get_flag(Carry) == 1 {
                    cpu.cycles_remaining += 1;
                    cpu.addr_abs = cpu.pc + cpu.addr_rel;

                    if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                        cpu.cycles_remaining += 1;
                    }

                    cpu.pc = cpu.addr_abs;
                }

                0
            }
            Beq => {
                if cpu.get_flag(Zero) == 1 {
                    cpu.cycles_remaining += 1;
                    cpu.addr_abs = cpu.pc + cpu.addr_rel;

                    if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                        cpu.cycles_remaining += 1;
                    }

                    cpu.pc = cpu.addr_abs;                    
                }

                0
            }
            Bit => {
                cpu.fetch();

                let temp = cpu.a_reg & cpu.fetched;

                cpu.set_flag(Zero, temp == 0);
                cpu.set_flag(Overflow, (cpu.fetched & (1 << 6)) != 0);
                cpu.set_flag(Negative, (cpu.fetched & (1 << 7)) != 0);
                
                0
            }
            Bmi => {
                if cpu.get_flag(Negative) == 1 {
                    cpu.cycles_remaining += 1;
                    cpu.addr_abs = cpu.pc + cpu.addr_rel;

                    if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                        cpu.cycles_remaining += 1;
                    }

                    cpu.pc = cpu.addr_abs;
                }
                0
            }
            Bne => {
                if cpu.get_flag(Zero) == 0 {
                    cpu.cycles_remaining += 1;
                    cpu.addr_abs = cpu.pc + cpu.addr_rel;

                    if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0x00FF) {
                        cpu.cycles_remaining += 1;
                    } 

                    cpu.pc = cpu.addr_abs;
                }

                0
            }
            Bpl => {
                if cpu.get_flag(Negative) == 0 {
                    cpu.cycles_remaining += 1;
                    cpu.addr_abs = cpu.pc + cpu.addr_rel;

                    if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                        cpu.cycles_remaining += 1;
                    }

                    cpu.pc = cpu.addr_abs;
                }

                0
            }
            Brk => {
                cpu.pc += 1;

                cpu.set_flag(InterruptDisable, true);
                cpu.write(0x0100 + cpu.stk_ptr as u16, ((cpu.pc >> 8) & 0x00FF) as u8);
                cpu.stk_ptr -= 1;
                cpu.write(0x0100 + cpu.stk_ptr as u16, (cpu.pc & 0x00FF) as u8);
                cpu.stk_ptr -= 1;

                cpu.set_flag(BreakCommand, true);
                cpu.write(0x0100 + cpu.stk_ptr as u16, cpu.status);
                cpu.stk_ptr -= 1;
                cpu.set_flag(BreakCommand, false);

                cpu.pc = cpu.read(0xFFFE) as u16 | (cpu.read(0xFFFF) as u16) << 8;

                0
            }
            Bvc => {
                if cpu.get_flag(Overflow) == 0 {
                    cpu.cycles_remaining += 1;
                    cpu.addr_abs = cpu.pc + cpu.addr_rel;

                    if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                        cpu.cycles_remaining += 1;
                    }

                    cpu.pc = cpu.addr_abs;
                }

                0
            }
            Bvs => {
                if cpu.get_flag(Overflow) == 1 {
                    cpu.cycles_remaining += 1;
                    cpu.addr_abs = cpu.pc + cpu.addr_abs;

                    if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                        cpu.cycles_remaining += 1;
                    }

                    cpu.pc = cpu.addr_abs;
                }

                0
            }
            Clc => {
                cpu.set_flag(Carry, false);

                0
            }
            Cld => {
                cpu.set_flag(DecimalMode, false);

                0
            }
            Cli => {
                cpu.set_flag(InterruptDisable, false);

                0
            }
            Clv => {
                cpu.set_flag(Overflow, false);

                0
            }
            Cmp => {
                cpu.fetch();

                let temp = cpu.a_reg as u16 - cpu.fetched as u16;

                cpu.set_flag(Carry, cpu.a_reg >= cpu.fetched);
                cpu.set_flag(Zero, temp & 0x00FF == 0x0000);
                cpu.set_flag(Negative, (temp & 0x0080) != 0);

                1
            }
            Cpx => {
                cpu.fetch();

                let temp = cpu.x_reg as u16 - cpu.fetched as u16;

                cpu.set_flag(Carry, cpu.x_reg >= cpu.fetched);
                cpu.set_flag(Zero, (temp & 0x00FF) != 0);
                cpu.set_flag(Negative, (temp & 0x0080) != 0);

                0
            }
            Cpy => {
                cpu.fetch();

                let temp = cpu.y_reg as u16 - cpu.fetched as u16;

                cpu.set_flag(Carry, cpu.x_reg >= cpu.fetched);
                cpu.set_flag(Zero, (temp & 0x00FF) != 0);
                cpu.set_flag(Negative, (temp & 0x0080) != 0);

                0
            }
            Dec => {
                cpu.fetch();

                let temp = cpu.fetched as u16 - 0x0001;
                cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);

                cpu.set_flag(Zero, (temp & 0x00FF) == 0x0000);
                cpu.set_flag(Negative, (temp & 0x0080) != 0);

                0
            }
            Dex => {
                cpu.x_reg -= 1;

                cpu.set_flag(Zero, cpu.x_reg == 0x0000);
                cpu.set_flag(Negative, (cpu.x_reg & 0x80) != 0);

                0
            }
            Dey => {
                cpu.y_reg -= 1;

                cpu.set_flag(Zero, cpu.y_reg == 0x0000);
                cpu.set_flag(Negative, (cpu.y_reg & 0x80) != 0);

                0
            }
            Eor => {
                cpu.fetch();

                cpu.a_reg ^= cpu.fetched;

                cpu.set_flag(Zero, cpu.a_reg == 0x00);
                cpu.set_flag(Negative, (cpu.a_reg & 0x80) != 0);

                1
            }
            Inc => {
                cpu.fetch();

                let temp = cpu.fetched + 1;
                cpu.write(cpu.addr_abs, temp);

                cpu.set_flag(Zero, temp == 0x00);
                cpu.set_flag(Negative, (temp & 0x80) != 0);

                0
            }
            Inx => {
                cpu.x_reg += 1;

                cpu.set_flag(Zero, cpu.x_reg == 0x00);
                cpu.set_flag(Negative, (cpu.x_reg & 0x80) != 0);

                0
            }
            Iny => {
                cpu.y_reg += 1;

                cpu.set_flag(Zero, cpu.y_reg == 0x00);
                cpu.set_flag(Negative, (cpu.y_reg & 0x80) != 0);

                0
            }
            Jmp => {
                cpu.pc = cpu.addr_abs;

                0
            }
            Jsr => {
                cpu.pc -= 1;

                cpu.write(0x0100 + cpu.stk_ptr as u16, ((cpu.pc >> 8) & 0x00FF) as u8);
                cpu.stk_ptr -= 1;
                cpu.write(0x0100 + cpu.stk_ptr as u16, (cpu.pc & 0x00FF) as u8);
                cpu.stk_ptr -= 1;

                cpu.pc = cpu.addr_abs;
                
                0
            }
            Lda => {
                cpu.fetch();

                cpu.a_reg = cpu.fetched;

                cpu.set_flag(Zero, cpu.a_reg == 0x00);
                cpu.set_flag(Negative, (cpu.a_reg & 0x80) != 0);

                1
            }
            Ldx => {
                cpu.fetch();

                cpu.x_reg = cpu.fetched;

                cpu.set_flag(Zero, cpu.x_reg == 0x00);
                cpu.set_flag(Negative, (cpu.x_reg & 0x80) != 0);

                1
            }
            Ldy => {
                cpu.fetch();

                cpu.y_reg = cpu.fetched;

                cpu.set_flag(Zero, cpu.y_reg == 0x00);
                cpu.set_flag(Negative, (cpu.y_reg & 0x80) != 0);

                1
            }
            Lsr => {
                cpu.fetch();

                let temp = cpu.fetched >> 1;

                cpu.set_flag(Carry, (cpu.fetched & 0x01) != 1);
                cpu.set_flag(Zero, temp == 0x00);
                cpu.set_flag(Negative, (temp & 0x80) != 0);

                if CPU_INSTRUCTIONS[cpu.opcode as usize].addr_mode == Implied{
                    cpu.a_reg = temp;
                }
                else {
                    cpu.write(cpu.addr_abs, temp);
                }
                
                0
            }
            Nop => {
                match cpu.opcode {
                    0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => {
                        return 1;
                    }
                    _ => {
                        return 0;
                    }
                }
            }
            Ora => {
                cpu.fetch();

                cpu.a_reg |= cpu.fetched;

                cpu.set_flag(Zero, cpu.a_reg == 0x00);
                cpu.set_flag(Negative, (cpu.a_reg & 0x80) != 0);

                1
            }
            Pha => {
                cpu.write(0x0100 + cpu.stk_ptr as u16, cpu.a_reg);
                cpu.stk_ptr -= 1;

                0
            }
            Php => {
                cpu.write(0x0100 + cpu.stk_ptr as u16, cpu.status);
                cpu.stk_ptr -= 1;

                0
            }
            Pla => {
                cpu.stk_ptr += 1;
                cpu.a_reg = cpu.read(0x0100 + cpu.stk_ptr as u16);

                cpu.set_flag(Zero, cpu.a_reg == 0x00);
                cpu.set_flag(Negative, (cpu.a_reg & 0x80) != 0);

                0
            }
            Plp => {
                cpu.stk_ptr += 1;
                cpu.status = cpu.read(0x0100 + cpu.stk_ptr as u16);

                cpu.set_flag(Unused, true);

                0
            }
            Rol => {
                cpu.fetch();

                let temp = (cpu.fetched << 1) as u16 | cpu.get_flag(Carry) as u16;

                cpu.set_flag(Carry, (temp & 0xFF00) != 0);
                cpu.set_flag(Zero, (temp & 0x00FF) == 0x00);
                cpu.set_flag(Negative, (temp & 0x0080) != 0);

                if CPU_INSTRUCTIONS[cpu.opcode as usize].addr_mode == Implied {
                    cpu.a_reg = (temp & 0x00FF) as u8;
                }
                else {
                    cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);
                }

                0
            }
            Ror => {
                cpu.fetch();

                let temp = (cpu.fetched >> 1) as u16 | (cpu.get_flag(Carry) << 7) as u16;

                cpu.set_flag(Carry, (temp & 0x01) != 0);
                cpu.set_flag(Zero, (temp & 0x00FF) == 0x00);
                cpu.set_flag(Negative, (temp & 0x0080) != 0);

                if CPU_INSTRUCTIONS[cpu.opcode as usize].addr_mode == Implied {
                    cpu.a_reg = (temp & 0x00FF) as u8;
                }
                else {
                    cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);
                }

                0
            }
            Rti => {
                cpu.stk_ptr += 1;
                cpu.status = cpu.read(0x0100 + cpu.stk_ptr as u16);
                cpu.status &= !(BreakCommand as u8);
                cpu.status &= !(Unused as u8);

                cpu.stk_ptr += 1;
                cpu.pc = cpu.read(0x0100 + cpu.stk_ptr as u16) as u16;
                cpu.stk_ptr += 1;
                cpu.pc |= (cpu.read(0x0100 + cpu.stk_ptr as u16) as u16) << 8;

                0
            }
            Rts => {
                cpu.stk_ptr += 1;
                cpu.pc = cpu.read(0x0100 + cpu.stk_ptr as u16) as u16 - 1;
                cpu.stk_ptr += 1;
                cpu.pc |= (cpu.read(0x0100 + cpu.stk_ptr as u16) as u16) << 8;

                0
            }
            Sbc => {
                cpu.fetch();

                let value = (cpu.fetched as u16) ^ 0x00FF;

                let temp = cpu.a_reg as u16 + cpu.fetched as u16 + cpu.get_flag(Carry) as u16;

                cpu.set_flag(Carry, temp > 255);
                cpu.set_flag(Zero, (temp  & 0x00FF) != 0);
                cpu.set_flag(Overflow, (((cpu.a_reg as u16 ^ temp) & (value ^ temp)) & 0x0080) != 0);
                cpu.set_flag(Negative, (temp & 0x80) != 0);
                
                cpu.a_reg = (temp & 0x00FF) as u8;

                1
            }
            Sec => {
                cpu.set_flag(Carry, true);

                0
            }
            Sed => {
                cpu.set_flag(DecimalMode, true);

                0
            }
            Sei => {
                cpu.set_flag(InterruptDisable, true);

                0
            }
            Sta => {
                cpu.write(cpu.addr_abs, cpu.a_reg);

                0
            }
            Stx => {
                cpu.write(cpu.addr_abs, cpu.x_reg);

                0
            }
            Sty => {
                cpu.write(cpu.addr_abs, cpu.y_reg);

                0
            }
            Tax => {
                cpu.x_reg = cpu.a_reg;

                cpu.set_flag(Zero, cpu.x_reg == 0x00);
                cpu.set_flag(Negative, (cpu.x_reg & 0x80) != 0);

                0
            }
            Tay => {
                cpu.y_reg = cpu.a_reg;

                cpu.set_flag(Zero, cpu.y_reg == 0x00);
                cpu.set_flag(Negative, (cpu.y_reg & 0x80) != 0);

                0
            }
            Tsx => {
                cpu.x_reg = cpu.stk_ptr;

                cpu.set_flag(Zero, cpu.x_reg == 0x00);
                cpu.set_flag(Negative, (cpu.x_reg & 0x80) != 0);

                0
            }
            Txa => {
                cpu.a_reg = cpu.x_reg;

                cpu.set_flag(Zero, cpu.a_reg == 0x00);
                cpu.set_flag(Negative, (cpu.a_reg & 0x80) != 0);

                0
            }
            Txs => {
                cpu.stk_ptr = cpu.x_reg;

                0
            }
            Tya => {
                cpu.a_reg = cpu.y_reg;

                cpu.set_flag(Zero, cpu.a_reg == 0x00);
                cpu.set_flag(Negative, (cpu.a_reg & 0x80) != 0);

                0
            }
            Kil => {
                0
            }
        }
    }
}

use Opcode::*;
use AddressingMode::*;

lazy_static!{
    pub static ref CPU_INSTRUCTIONS: [Instruction; 256] = [
        Instruction::new( Brk, Implied, 7 ),      Instruction::new( Ora, Indirect_X, 6 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Kil, Implied, 3 ),      Instruction::new( Ora, ZeroPage, 3 ),     Instruction::new( Asl, ZeroPage, 5 ),      Instruction::new( Kil, Implied, 5 ),    Instruction::new( Php, Implied, 3 ),    Instruction::new( Ora, Immediate, 2 ),     Instruction::new( Asl, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Nop, Implied, 4 ),     Instruction::new( Ora, Absolute, 4 ),      Instruction::new( Asl, Absolute, 6 ),      Instruction::new( Kil, Implied, 6 ),
        Instruction::new( Bpl, Relative, 2 ),     Instruction::new( Ora, Indirect_Y, 5 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Kil, Implied, 4 ),      Instruction::new( Ora, ZeroPage_X, 4 ),   Instruction::new( Asl, ZeroPage_X, 6 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Clc, Implied, 2 ),    Instruction::new( Ora, Absolute_Y, 4 ),    Instruction::new( Nop, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Nop, Implied, 4 ),     Instruction::new( Ora, Absolute_X, 4 ),    Instruction::new( Asl, Absolute_X, 7 ),    Instruction::new( Kil, Implied, 7 ),
        Instruction::new( Jsr, Absolute, 6 ),     Instruction::new( And, Indirect_X, 6 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Bit, ZeroPage, 3 ),     Instruction::new( And, ZeroPage, 3 ),     Instruction::new( Rol, ZeroPage, 5 ),      Instruction::new( Kil, Implied, 5 ),    Instruction::new( Plp, Implied, 4 ),    Instruction::new( And, Immediate, 2 ),     Instruction::new( Rol, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Bit, Absolute, 4 ),    Instruction::new( And, Absolute, 4 ),      Instruction::new( Rol, Absolute, 6 ),      Instruction::new( Kil, Implied, 6 ),
        Instruction::new( Bmi, Relative, 2 ),     Instruction::new( And, Indirect_Y, 5 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Kil, Implied, 4 ),      Instruction::new( And, ZeroPage_X, 4 ),   Instruction::new( Rol, ZeroPage_X, 6 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Sec, Implied, 2 ),    Instruction::new( And, Absolute_Y, 4 ),    Instruction::new( Nop, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Nop, Implied, 4 ),     Instruction::new( And, Absolute_X, 4 ),    Instruction::new( Rol, Absolute_X, 7 ),    Instruction::new( Kil, Implied, 7 ),
        Instruction::new( Rti, Implied, 6 ),      Instruction::new( Eor, Indirect_X, 6 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Kil, Implied, 3 ),      Instruction::new( Eor, ZeroPage, 3 ),     Instruction::new( Lsr, ZeroPage, 5 ),      Instruction::new( Kil, Implied, 5 ),    Instruction::new( Pha, Implied, 3 ),    Instruction::new( Eor, Immediate, 2 ),     Instruction::new( Lsr, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Jmp, Absolute, 3 ),    Instruction::new( Eor, Absolute, 4 ),      Instruction::new( Lsr, Absolute, 6 ),      Instruction::new( Kil, Implied, 6 ),
        Instruction::new( Bvc, Relative, 2),      Instruction::new( Eor, Indirect_Y, 5 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Kil, Implied, 4 ),      Instruction::new( Eor, ZeroPage_X, 4 ),   Instruction::new( Lsr, ZeroPage_X, 6 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Cli, Implied, 2 ),    Instruction::new( Eor, Absolute_Y, 4 ),    Instruction::new( Nop, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Nop, Implied, 4 ),     Instruction::new( Eor, Absolute_X, 4 ),    Instruction::new( Lsr, Absolute_X, 7 ),    Instruction::new( Kil, Implied, 7 ),
        Instruction::new( Rts, Implied, 6 ),      Instruction::new( Adc, Indirect_X, 6 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Kil, Implied, 3 ),      Instruction::new( Adc, ZeroPage, 3 ),     Instruction::new( Ror, ZeroPage, 5 ),      Instruction::new( Kil, Implied, 5 ),    Instruction::new( Pla, Implied, 4 ),    Instruction::new( Adc, Immediate, 2 ),     Instruction::new( Ror, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Jmp, Indirect, 5 ),    Instruction::new( Adc, Absolute, 4 ),      Instruction::new( Ror, Absolute, 6 ),      Instruction::new( Kil, Implied, 6 ),
        Instruction::new( Bvs, Relative, 2 ),     Instruction::new( Adc, Indirect_Y, 5 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Kil, Implied, 4 ),      Instruction::new( Adc, ZeroPage_X, 4 ),   Instruction::new( Ror, ZeroPage_X, 6 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Sei, Implied, 2 ),    Instruction::new( Adc, Absolute_Y, 4 ),    Instruction::new( Nop, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Nop, Implied, 4 ),     Instruction::new( Adc, Absolute_X, 4 ),    Instruction::new( Ror, Absolute_X, 7 ),    Instruction::new( Kil, Implied, 7 ),
        Instruction::new( Kil, Implied, 2 ),      Instruction::new( Sta, Indirect_X, 6 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Sty, ZeroPage, 3 ),     Instruction::new( Sta, ZeroPage, 3 ),     Instruction::new( Stx, ZeroPage, 3 ),      Instruction::new( Kil, Implied, 5 ),    Instruction::new( Dey, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),       Instruction::new( Txa, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Sty, Absolute, 4 ),    Instruction::new( Sta, Absolute, 4 ),      Instruction::new( Stx, Absolute, 4 ),      Instruction::new( Kil, Implied, 4 ),
        Instruction::new( Bcc, Relative, 2 ),     Instruction::new( Sta, Indirect_Y, 6 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Sty, ZeroPage_X, 4 ),   Instruction::new( Sta, ZeroPage_X, 4 ),   Instruction::new( Stx, ZeroPage_Y, 4 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Tya, Implied, 2 ),    Instruction::new( Sta, Absolute_Y, 5 ),    Instruction::new( Txs, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Nop, Implied, 5 ),     Instruction::new( Sta, Absolute_X, 5 ),    Instruction::new( Kil, Implied, 5 ),       Instruction::new( Kil, Implied, 5 ),
        Instruction::new( Ldy, Immediate, 2 ),    Instruction::new( Lda, Indirect_X, 6 ),    Instruction::new( Ldx, Immediate, 2 ),  Instruction::new( Kil, Implied, 6 ),    Instruction::new( Ldy, ZeroPage, 3 ),     Instruction::new( Lda, ZeroPage, 3 ),     Instruction::new( Ldx, ZeroPage, 3 ),      Instruction::new( Kil, Implied, 5 ),    Instruction::new( Tay, Implied, 2 ),    Instruction::new( Lda, Immediate, 2 ),     Instruction::new( Tax, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Ldy, Absolute, 4 ),    Instruction::new( Lda, Absolute, 4 ),      Instruction::new( Ldx, Absolute, 4 ),      Instruction::new( Kil, Implied, 4 ),
        Instruction::new( Bcs, Relative, 2 ),     Instruction::new( Lda, Indirect_Y, 5 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 5 ),    Instruction::new( Ldy, ZeroPage_X, 4 ),   Instruction::new( Lda, ZeroPage_X, 4 ),   Instruction::new( Ldx, ZeroPage_Y, 4 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Clv, Implied, 2 ),    Instruction::new( Lda, Absolute_Y, 4 ),    Instruction::new( Tsx, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Ldy, Absolute_X, 4 ),  Instruction::new( Lda, Absolute_X, 4 ),    Instruction::new( Ldx, Absolute_Y, 4 ),    Instruction::new( Kil, Implied, 4 ),
        Instruction::new( Cpy, Immediate, 2 ),    Instruction::new( Cmp, Indirect_X, 6 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Cpy, ZeroPage, 3 ),     Instruction::new( Cmp, ZeroPage, 3 ),     Instruction::new( Dec, ZeroPage, 5 ),      Instruction::new( Kil, Implied, 5 ),    Instruction::new( Iny, Implied, 2 ),    Instruction::new( Cmp, Immediate, 2 ),     Instruction::new( Dex, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Cpy, Absolute, 4 ),    Instruction::new( Cmp, Absolute, 4 ),      Instruction::new( Dec, Absolute, 6 ),      Instruction::new( Kil, Implied, 6 ),
        Instruction::new( Bne, Relative, 2 ),     Instruction::new( Cmp, Indirect_Y, 5 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Kil, Implied, 4 ),      Instruction::new( Cmp, ZeroPage_X, 4 ),   Instruction::new( Dec, ZeroPage_X, 6 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Cld, Implied, 2 ),    Instruction::new( Cmp, Absolute_Y, 4 ),    Instruction::new( Nop, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Nop, Implied, 4 ),     Instruction::new( Cmp, Absolute_X, 4 ),    Instruction::new( Dec, Absolute_X, 7 ),    Instruction::new( Kil, Implied, 7 ),
        Instruction::new( Cpx, Immediate, 2 ),    Instruction::new( Sbc, Indirect_X, 6 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Cpx, ZeroPage, 3 ),     Instruction::new( Sbc, ZeroPage, 3 ),     Instruction::new( Inc, ZeroPage, 5 ),      Instruction::new( Kil, Implied, 5 ),    Instruction::new( Inx, Implied, 2 ),    Instruction::new( Sbc, Immediate, 2 ),     Instruction::new( Nop, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Cpx, Absolute, 4 ),    Instruction::new( Sbc, Absolute, 4 ),      Instruction::new( Inc, Absolute, 6 ),      Instruction::new( Kil, Implied, 6 ),
        Instruction::new( Beq, Relative, 2 ),     Instruction::new( Sbc, Indirect_Y, 5 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Kil, Implied, 8 ),    Instruction::new( Kil, Implied, 4 ),      Instruction::new( Sbc, ZeroPage_X, 4 ),   Instruction::new( Inc, ZeroPage_X, 6 ),    Instruction::new( Kil, Implied, 6 ),    Instruction::new( Sed, Implied, 2 ),    Instruction::new( Sbc, Absolute_Y, 4 ),    Instruction::new( Nop, Implied, 2 ),    Instruction::new( Kil, Implied, 2 ),    Instruction::new( Nop, Implied, 4 ),     Instruction::new( Sbc, Absolute_X, 4 ),    Instruction::new( Inc, Absolute_X, 7 ),    Instruction::new( Kil, Implied, 7 ),   
    ];
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_opcode = match self {
            Adc => "ADC",
            And => "AND",
            Asl => "ASL",
            Bcs => "BCS",
            Bcc => "BCC",
            Beq => "BEQ",
            Bit => "BIT",
            Bmi => "BMI",
            Bne => "BNE",
            Bpl => "BPL",
            Brk => "BRK",
            Bvc => "BVC",
            Bvs => "BVS",
            Clc => "CLC",
            Cld => "CLD",
            Cli => "CLI",
            Clv => "CLV",
            Cmp => "CMP",
            Cpx => "CPX",
            Cpy => "CPY",
            Dec => "DEC",
            Dex => "DEX",
            Dey => "DEY",
            Eor => "EOR",
            Inc => "INC",
            Inx => "INX",
            Iny => "INY",
            Jmp => "JMP",
            Jsr => "JSR",
            Lda => "LDA",
            Ldx => "LDX",
            Ldy => "LDY",
            Lsr => "LSR",
            Nop => "NOP",
            Ora => "ORA",
            Pha => "PHA",
            Php => "PHP",
            Pla => "PLA",
            Plp => "PLP",
            Rol => "ROL",
            Ror => "ROR",
            Rti => "RTI",
            Rts => "RTS",
            Sbc => "SBC",
            Sec => "SEC",
            Sed => "SED",
            Sei => "SEI",
            Sta => "STA",
            Stx => "STX",
            Sty => "STY",
            Tax => "TAX",
            Tay => "TAY",
            Tsx => "TSX",
            Txa => "TXA",
            Txs => "TXS",
            Tya => "TYA",
            Kil => "KIL"
        };

        write!(f, "{}", formatted_opcode)
    }
}