use super::xy::XYGet;

pub struct Velocity {
    x: f64,
    y: f64,
}

impl XYGet for Velocity {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

impl Velocity {
    pub fn new(x: f64, y: f64) -> Self {
        Velocity { x: x, y: y }
    }
}
