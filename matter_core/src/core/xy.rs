pub trait XYGet {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
}

pub trait XYSet {
    fn set_x(&mut self, x: f64);
    fn set_y(&mut self, y: f64);
}
