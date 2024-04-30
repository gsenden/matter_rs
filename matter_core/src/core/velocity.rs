use super::xy::{XYNew, XY};

#[derive(Clone, Copy, Default)]
pub struct Velocity {
    x: f64,
    y: f64,
}

impl XY for Velocity {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}

impl XYNew for Velocity {
    type XY = Velocity;
    fn new(x: f64, y: f64) -> Self {
        Velocity { x: x, y: y }
    }
}
