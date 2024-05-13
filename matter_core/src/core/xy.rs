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

    fn magnitude_squared(&self) -> f64 {
        self.get_x().powi(2) + self.get_y().powi(2)
    }

    fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    fn rotate_about(&mut self, angle: f64, point: &impl XY) {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = point.get_x()
            + ((self.get_x() - point.get_x()) * cos - (self.get_y() - point.get_y()) * sin);
        let y = point.get_y()
            + ((self.get_x() - point.get_x()) * sin + (self.get_y() - point.get_y()) * cos);
        self.set_x_y(x, y);
    }

    fn cross3(vector_a: &impl XY, vector_b: &impl XY, vector_c: &impl XY) -> f64 {
        (vector_b.get_x() - vector_a.get_x()) * (vector_c.get_y() - vector_a.get_y())
            - (vector_b.get_y() - vector_a.get_y()) * (vector_c.get_x() - vector_a.get_x())
    }

    fn cross(vector_a: &impl XY, vector_b: &impl XY) -> f64 {
        (vector_a.get_x() * vector_b.get_y()) - (vector_a.get_y() * vector_b.get_x())
    }

    fn perp(&mut self, negate: bool) {
        let negate_factor = if negate { -1.0 } else { 1.0 };

        let x = negate_factor * (self.get_y() * -1.0);
        let y = negate_factor * self.get_x();
        self.set_x_y(x, y);
    }

    fn angle(vector_a: &impl XY, vector_b: &impl XY) -> f64 {
        f64::atan2(
            vector_b.get_y() - vector_a.get_y(),
            vector_b.get_x() - vector_a.get_x(),
        )
    }

    fn normalise(&mut self) {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            self.set_x_y(0., 0.);
        } else {
            self.set_x(self.get_x() / magnitude);
            self.set_y(self.get_y() / magnitude);
        }
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
