pub trait XY {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;

    fn set_x(&mut self, x: f64);
    fn set_y(&mut self, y: f64);
    fn set_xy(&mut self, xy: &impl XY) {
        self.set_x(xy.get_x());
        self.set_y(xy.get_y());
    }
    fn add_xy(&mut self, xy: &impl XY) {
        self.set_x(self.get_x() + xy.get_x());
        self.set_y(self.get_y() + xy.get_y());
    }
}

pub trait XYNew {
    type XY;

    fn new(x: f64, y: f64) -> Self::XY;
    fn new_from(xy: &impl XY) -> Self::XY {
        <Self as XYNew>::new(xy.get_x(), xy.get_y())
    }
}
