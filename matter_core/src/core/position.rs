use super::xy::{XYGet, XYSet};

#[derive(Clone, Copy)]
pub struct Position {
    x: f64,
    y: f64,
}

impl XYGet for Position {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x: x, y: y }
    }
}

impl XYSet for Position {
    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}
