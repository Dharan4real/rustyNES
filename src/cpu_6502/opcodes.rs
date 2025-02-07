//All the 56 official opcodes. 
//As ach opcode is defined by 1 byte, there are 256 possible codes.
//Codes are responsible for switching individual parts of 
//CPU circuits on and off.
fn ADC() -> u8 { 0 }
fn AND() -> u8 { 0 }
fn ASL() -> u8 { 0 }
fn BCC() -> u8 { 0 }

fn BCS() -> u8 { 0 }
fn BEQ() -> u8 { 0 }
fn BIT() -> u8 { 0 }
fn BMI() -> u8 { 0 }

fn BNE() -> u8 { 0 }
fn BPL() -> u8 { 0 }
fn BRK() -> u8 { 0 }
fn BVC() -> u8 { 0 }

fn BVS() -> u8 { 0 }
fn CLC() -> u8 { 0 }
fn CLD() -> u8 { 0 }
fn CLI() -> u8 { 0 }

fn CLV() -> u8 { 0 }
fn CMP() -> u8 { 0 }
fn CPX() -> u8 { 0 }
fn CPY() -> u8 { 0 }

fn DEC() -> u8 { 0 }
fn DEX() -> u8 { 0 }
fn DEY() -> u8 { 0 }
fn EOR() -> u8 { 0 }

fn INC() -> u8 { 0 }
fn INX() -> u8 { 0 }
fn INY() -> u8 { 0 }
fn JMP() -> u8 { 0 }

fn JSR() -> u8 { 0 }
fn LDA() -> u8 { 0 }
fn LDX() -> u8 { 0 }
fn LDY() -> u8 { 0 }

fn LSR() -> u8 { 0 }
fn NOP() -> u8 { 0 }
fn ORA() -> u8 { 0 }
fn PHA() -> u8 { 0 }

fn PHP() -> u8 { 0 }
fn PLA() -> u8 { 0 }
fn PLP() -> u8 { 0 }
fn ROL() -> u8 { 0 }

fn ROR() -> u8 { 0 }
fn RTI() -> u8 { 0 }
fn RTS() -> u8 { 0 }
fn SBC() -> u8 { 0 }

fn SEC() -> u8 { 0 }
fn SED() -> u8 { 0 }
fn SEI() -> u8 { 0 }
fn STA() -> u8 { 0 }

fn STX() -> u8 { 0 }
fn STY() -> u8 { 0 }
fn TAX() -> u8 { 0 }
fn TAY() -> u8 { 0 }

fn TSX() -> u8 { 0 }
fn TXA() -> u8 { 0 }
fn TXS() -> u8 { 0 }
fn TYA() -> u8 { 0 }

//Catching all "unofficial" codes in this function.
pub fn xxx() -> u8 { 0 }
