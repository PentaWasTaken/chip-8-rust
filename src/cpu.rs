use crate::errors::Chip8Error;
use crate::ram::Ram;
use crate::{display::Display, ram};

#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16], //General purpose registers
    pc: u16,      //Program counter
    i: u16,       //Another register, mostly for memory addresses
    stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            vx: [0u8; 16],
            pc: 0x200, //Points to the start of the executable by default
            i: 0,
            stack: Vec::with_capacity(16),
        }
    }

    pub fn tick(&mut self, ram: &mut Ram, display: &mut Display) -> Result<(), Chip8Error> {
        if self.pc >= (ram.length() - 1) as u16 {
            return Err(Chip8Error::EOF);
        }
        let instr_u = ram.read_byte(self.pc); //Upper (most significant) byte of the instruction
        let instr_l = ram.read_byte(self.pc + 1); //Lower (leasz significant) byte of the instruction
        let instr = ((instr_u as u16) << 8) | (instr_l as u16);

        //Clears the display
        if instr == 0x00E0 {
            display.clear();

        //Returns from a subroutine
        } else if instr == 0x00EE {
            if self.stack.len() == 0 {
                return Err(Chip8Error::ReturnOnEmptyStack(self.pc));
            }
            self.pc = *self.stack.last().unwrap();
            self.stack.pop();
            return Ok(());

        //Jumps to another location
        } else if instr & 0xF000 == 0x1000 {
            let l = instr & 0x0FFF;
            self.pc = l;
            return Ok(());

        //Calls a subroutine
        } else if instr & 0xF000 == 0x2000 {
            self.stack.push(self.pc);
            let l = instr & 0x0FFF;
            self.pc = l;
            return Ok(());

        //Skips the next instruction if the register is equal to the value of the lower 8 bits
        } else if instr & 0xF000 == 0x3000 {
            let index = (instr & 0x0F00) >> 8;
            let value = instr & 0x00FF;
            if self.vx[index as usize] == value as u8 {
                self.pc += 2;
            }
        } else {
            return Err(Chip8Error::UnsupportedInstr(instr, self.pc));
        }

        self.pc += 2;
        Ok(())
    }
}
