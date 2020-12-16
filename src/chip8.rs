use crate::cpu::Cpu;
use crate::display;
use crate::display::Display;
use crate::ram::Ram;

use ggez::event::EventHandler;
use ggez::graphics::{self, DrawParam, Image};
use ggez::{timer, Context, GameResult};

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
}

impl EventHandler for Chip8 {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_HZ: u32 = 500;
        while timer::check_update_time(ctx, DESIRED_HZ) {
            let err = self.cpu
                .tick(
                    &mut self.ram,
                    &mut self.display,
                    &self.keys,
                    &mut self.delay_t,
                    &mut self.sound_t,
                );

            if let Err(e) = err {
                panic!("{}", e);
            }
        }

        //println!("{:?}", self);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        //Convert display to rgba
        let raw_display = self.display.to_raw();
        let mut image = Image::from_rgba8(
            ctx,
            display::WIDTH as u16,
            display::HEIGHT as u16,
            &raw_display,
        )
        .unwrap();
        image.set_filter(graphics::FilterMode::Nearest);

        graphics::draw(ctx, &image, DrawParam::default().scale([10.0, 10.0]))?;

        graphics::present(ctx)
    }
}
