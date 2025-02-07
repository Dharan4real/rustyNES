
use crate::cpu_6502::Cpu;

pub struct Bus {
    cpu: Cpu,
    ram: [u8; 64 * 1024],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu: Cpu::new(),
            ram: [0x00; 64 * 1024],
        }
    }

    pub fn read(&self, addr: u16, readonly: bool) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            return self.ram[addr as usize];
        }

        0x00
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr as usize] = data;
        }
    }
}
