use super::xy::XY;

#[derive(Clone, Copy, Default)]
pub struct ConstraintImpulse {
    x: f64,
    y: f64,
    angle: f64,
}

impl XY for ConstraintImpulse {
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

impl ConstraintImpulse {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        ConstraintImpulse {
            x: x,
            y: y,
            angle: angle,
        }
    }

    pub fn get_angle(&self) -> f64 {
        self.angle
    }

    // fn set_angle(&mut self, angle: f64) {
    //     self.angle = angle
    // }
}
