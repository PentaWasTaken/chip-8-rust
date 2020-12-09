use std::fs::File;
use std::io::Read;

mod chip8;
use chip8::Chip8;

mod cpu;
mod display;
mod errors;
mod ram;

const ROM_PATH: &str = "games/danm8ku.ch8";

fn main() {
    //Read the ROM file
    let mut file = File::open(ROM_PATH).expect(&format!("ROM '{}' not found!", ROM_PATH));
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).expect("Could not read file.");

    //Create a new Chip8 instance and load the rom into RAM
    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    println!("{:?}", chip8);

    loop {
        chip8.tick();
    }
}
