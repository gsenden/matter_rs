use super::xy::XYGet;

#[derive(Clone, Copy)]
pub struct Force {
    x: f64,
    y: f64,
}

impl XYGet for Force {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

impl Force {
    pub fn new(x: f64, y: f64) -> Self {
        Force { x: x, y: y }
    }
}
