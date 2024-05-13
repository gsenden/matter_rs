pub trait XY {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;

    fn set_x(&mut self, x: f64);
    fn set_y(&mut self, y: f64);
    fn set_xy(&mut self, xy: &impl XY) {
        self.set_x_y(xy.get_x(), xy.get_y())
    }
    fn set_x_y(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }

    fn add_xy(&mut self, xy: &impl XY) {
        self.add_x(xy.get_x());
        self.add_y(xy.get_y());
    }
    fn add_x_y(&mut self, x: f64, y: f64) {
        self.add_x(x);
        self.add_y(y);
    }
    fn add_x(&mut self, x: f64) {
        self.set_x(self.get_x() + x);
    }

    fn add_y(&mut self, y: f64) {
        self.set_y(self.get_y() + y);
    }

    fn sub(&mut self, value: &impl XY) {
        self.set_x(self.get_x() - value.get_x());
        self.set_y(self.get_y() - value.get_y());
    }

    fn mult(&mut self, scalar: f64) {
        self.set_x(self.get_x() * scalar);
        self.set_y(self.get_y() * scalar);
    }

    fn div(&mut self, scalar: f64) {
        self.set_x(self.get_x() / scalar);
        self.set_y(self.get_y() / scalar);
    }

    fn neg(&mut self) {
        self.mult(-1.);
    }

    fn dot(&self, multiplier: &impl XY) -> f64 {
        self.get_x() * multiplier.get_x() + self.get_y() * multiplier.get_y()
    }
}

pub trait XYNew {
    type XY;

    fn new(x: f64, y: f64) -> Self::XY;
    fn new_from(xy: &impl XY) -> Self::XY {
        <Self as XYNew>::new(xy.get_x(), xy.get_y())
    }
    fn add(a: &impl XY, b: &impl XY) -> Self::XY {
        <Self as XYNew>::new(a.get_x() + b.get_x(), a.get_y() + b.get_y())
    }
}
