use super::xy::{XYNew, XY};

#[derive(Clone, Copy, Default)]
pub struct Position {
    x: f64,
    y: f64,
}

impl XY for Position {
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

impl XYNew for Position {
    type XY = Position;

    fn new(x: f64, y: f64) -> Self {
        Position { x: x, y: y }
    }
}
