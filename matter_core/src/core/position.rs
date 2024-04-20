use super::xy::{XYFrom, XY};

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

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x: x, y: y }
    }
}

impl XYFrom<Position> for Position {
    fn new_from_xy(xy_get: &impl XY) -> Position {
        Position {
            x: xy_get.get_x(),
            y: xy_get.get_y(),
        }
    }
}
