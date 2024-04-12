use super::xy::XYGet;

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
