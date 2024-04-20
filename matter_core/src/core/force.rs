use super::xy::XY;

#[derive(Clone, Copy, Default)]
pub struct Force {
    x: f64,
    y: f64,
}

impl XY for Force {
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

impl Force {
    pub fn new(x: f64, y: f64) -> Self {
        Force { x: x, y: y }
    }
}
