pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 64;

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
}
