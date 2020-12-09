use crate::display::Display;
use crate::ram::Ram;
use crate::{cpu::Cpu, errors};

#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    ram: Ram,
    display: Display,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            cpu: Cpu::new(),
            ram: Ram::new(),
            display: Display::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200u16; //This offset takes into account the 512 bytes which were originally reserved for the interpreter

        for (index, value) in data.iter().enumerate() {
            self.ram.write_byte(index as u16 + offset, *value);
        }
    }

    pub fn tick(&mut self) {
        let error = self.cpu.tick(&mut self.ram, &mut self.display);
        if error.is_err() {
            panic!("{}", error.unwrap_err());
        }
    }
}
