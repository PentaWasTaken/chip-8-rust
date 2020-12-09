use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Chip8Error {
    EOF,
    UnsupportedInstr(u16, u16), //(instruction value, line)
    ReturnOnEmptyStack(u16),    //(line)
}

impl Display for Chip8Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Chip8Error::EOF => write!(f, "End of file reached"),
            Chip8Error::UnsupportedInstr(instr, pc) => {
                write!(f, "Unsupported instruction: 0x{:x} on line {}", instr, pc)
            }
            Chip8Error::ReturnOnEmptyStack(pc) => write!(f, "Return on empty stack on line {}", pc),
        }
    }
}

impl Error for Chip8Error {}
