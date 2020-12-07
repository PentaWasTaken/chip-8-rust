use ram::Ram;

use crate::ram;

#[derive(Debug)]
pub struct Chip8 {
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 { ram: Ram::new() }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200u16; //This offset takes into account the 512 bytes which were originally reserved for the interpreter

        for (index, value) in data.iter().enumerate() {
            self.ram.write_byte(index as u16 + offset, *value);
        }
    }
}
