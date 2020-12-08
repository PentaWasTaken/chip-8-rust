pub struct Cpu {
    vx: [u8; 16], //General purpose registers
    pc: u16,      //Program counter
    i: u16,       //Another register, mostly for memory addresses
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            vx: [0u8; 16],
            pc: 0x200,     //Points to the start of the executable by default
            i: 0,
        }
    }
}