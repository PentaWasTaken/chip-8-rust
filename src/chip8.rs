use crate::cpu::Cpu;
use crate::display::Display;
use crate::ram::Ram;

#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    ram: Ram,
    display: Display,
    keys: [bool; 16],
    delay_t: u8,
    sound_t: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            cpu: Cpu::new(),
            ram: Ram::new(),
            display: Display::new(),
            keys: [false; 16],
            delay_t: 0,
            sound_t: 0,
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200u16; //This offset takes into account the 512 bytes which were originally reserved for the interpreter

        for (index, value) in data.iter().enumerate() {
            self.ram.write_byte(index as u16 + offset, *value);
        }
    }

    pub fn tick(&mut self) {
        let error = self.cpu.tick(
            &mut self.ram,
            &mut self.display,
            &self.keys,
            &mut self.delay_t,
            &mut self.sound_t,
        );
        if error.is_err() {
            panic!("{}", error.unwrap_err());
        }
    }
}
