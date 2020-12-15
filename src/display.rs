pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    data: [bool; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Display {
            data: [false; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.data = [false; WIDTH * HEIGHT];
    }

    pub fn display_sprite(&mut self, spr_data: &[u8], x_pos: u16, y_pos: u16) -> bool {
        //Returns true on collision
        let mut collision = false;
        for (y_index, &line) in spr_data.iter().enumerate() {
            //Loop through each bit in the byte
            for x_index in 0..8 {
                let bit = line & (1 << x_index);

                let final_x = (x_pos + x_index) % WIDTH as u16;
                let final_y = (y_pos + y_index as u16) % HEIGHT as u16;

                let i = Display::coords_to_index(final_x, final_y) as usize;
                if self.data[i] && bit > 0 {
                    collision = true;
                }

                self.data[i] = self.data[i] ^ (bit > 0);
            }
        }
        collision
    }

    #[inline]
    fn coords_to_index(x: u16, y: u16) -> u16 {
        y * WIDTH as u16 + x
    }
}
