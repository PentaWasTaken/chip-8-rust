use crate::cpu::Cpu;
use crate::display;
use crate::display::Display;
use crate::ram::Ram;

use ggez::event::EventHandler;
use ggez::graphics::{self, DrawParam, Image};
use ggez::{timer, Context, GameResult};
use ggez::input::keyboard::{KeyCode, KeyMods};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    ram: Ram,
    display: Display,
    keys: [bool; 16],
    delay_t: u8,
    delay_dec: u8,
    sound_t: u8,
    keymap: HashMap<KeyCode, usize>,
}

impl Chip8 {
    pub fn new() -> Self {
        //Initialize the keymap
        //Layout:
        //---------    ---------
        //|A|S|D|F|    |1|2|3|C|
        //---------    ---------
        //|7|8|9|G|    |4|5|6|D|
        //--------- => ---------
        //|4|5|6|H|    |7|8|9|E|
        //---------    ---------
        //|1|2|3|J|    |A|0|B|F|
        //---------    ---------

        let keymap: HashMap<KeyCode, usize> = [
            (KeyCode::A, 0x1),
            (KeyCode::S, 0x2),
            (KeyCode::D, 0x3),
            (KeyCode::F, 0xC),
            (KeyCode::Numpad7, 0x4),
            (KeyCode::Numpad8, 0x5),
            (KeyCode::Numpad9, 0x6),
            (KeyCode::G, 0xD),
            (KeyCode::Numpad4, 0x7),
            (KeyCode::Numpad5, 0x8),
            (KeyCode::Numpad6, 0x9),
            (KeyCode::H, 0xE),
            (KeyCode::Numpad1, 0xA),
            (KeyCode::Numpad2, 0x0),
            (KeyCode::Numpad3, 0xB),
            (KeyCode::J, 0xF),
        ].iter().cloned().collect();

        Chip8 {
            cpu: Cpu::new(),
            ram: Ram::new(),
            display: Display::new(),
            keys: [false; 16],
            delay_t: 0,
            delay_dec: 8,
            sound_t: 0,
            keymap: keymap,
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
            if self.delay_dec > 0 {
                self.delay_dec -= 1;
            } else if self.delay_t > 0 {
                self.delay_dec = 8;
                self.delay_t -= 1;
            }
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

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if self.keymap.contains_key(&keycode) {
            self.keys[self.keymap[&keycode]] = true;
            if self.cpu.blocked.0 {
                self.cpu.vx[self.cpu.blocked.1] = self.keymap[&keycode] as u8;
                self.cpu.blocked = (false, 0);
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if self.keymap.contains_key(&keycode) {
            self.keys[self.keymap[&keycode]] = false;
        }
    }
}
