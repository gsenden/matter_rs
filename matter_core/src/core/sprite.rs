#[derive(Clone, Copy)]
pub struct Sprite {
    x_scale: f64,
    y_scale: f64,
    x_offset: f64,
    y_offset: f64,
}

impl Sprite {
    pub fn new(x_scale: f64, y_scale: f64, x_offset: f64, y_offset: f64) -> Self {
        Sprite {
            x_scale: x_scale,
            y_scale: y_scale,
            x_offset: x_offset,
            y_offset: y_offset,
        }
    }

    pub fn get_x_scale(&self) -> f64 {
        self.x_scale
    }

    pub fn get_y_scale(&self) -> f64 {
        self.y_scale
    }

    pub fn get_x_offset(&self) -> f64 {
        self.x_offset
    }

    pub fn get_y_offset(&self) -> f64 {
        self.y_offset
    }
}
