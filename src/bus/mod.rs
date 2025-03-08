use crate::cpu_6502::*;

pub struct Bus {
    pub ram: [u8; 64 * 1024],
    pub cpu: *mut Cpu,
}

impl Bus {
    pub fn new(cpu: &mut Cpu) -> Self {
        Bus {
            cpu: cpu,
            ram: [0x00; 64 * 1024],
        }
    }

    // pub fn connect_to_cpu(&mut self, cpu_ptr: *mut Cpu) {
    //     self.cpu = cpu_ptr;
    // }

    #[allow(unused_comparisons)]
    pub fn read(&self, addr: u16, readonly: bool) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            return self.ram[addr as usize];
        }

        0x00
    }

    #[allow(unused_comparisons)]
    pub fn write(&mut self, addr: u16, data: u8) {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr as usize] = data;
        }
    }
}
