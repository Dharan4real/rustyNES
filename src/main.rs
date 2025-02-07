
mod bus;
mod cpu_6502;

use bus::Bus;
use cpu_6502::opcodes;

fn main() {
    let mut bus = Bus::new();
    bus.write(0x00FF, 250);
    println!("{}", bus.read(0x00FF, false));
    println!("{}", opcodes::xxx());
}
