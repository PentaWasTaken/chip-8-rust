pub struct Ram {
    mem: [u8; 4096],
}

impl Ram {
    pub fn new() -> Self {
        let mut ram = Ram { mem: [0; 4096] };

        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], //0
            [0x20, 0x60, 0x20, 0x20, 0x70], //1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], //2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], //3
            [0x90, 0x90, 0xF0, 0x10, 0x10], //4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], //5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], //6
            [0xF0, 0x10, 0x20, 0x40, 0x40], //7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], //8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], //9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], //A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], //B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], //C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], //D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], //E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], //F
        ];

        for (index, b) in sprites.iter().flatten().enumerate() {
            ram.write_byte(index as u16, *b);
        }

        ram
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
}
