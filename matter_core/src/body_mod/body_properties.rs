use crate::core::{
    position::Position,
    xy::{XYGet, XYSet},
};

#[derive(Clone, Copy)]
pub struct BodyProperties {
    mass: f64,
    area: f64,
    inertia: f64,
    centre: Position,
}

impl BodyProperties {
    pub fn new(mass: f64, area: f64, inertia: f64, centre: Position) -> Self {
        BodyProperties {
            mass: mass,
            area: area,
            inertia: inertia,
            centre: centre,
        }
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_area(&self) -> f64 {
        self.area
    }

    pub fn get_inertia(&self) -> f64 {
        self.inertia
    }

    pub fn get_centre(&self) -> Position {
        self.centre
    }

    pub fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }

    pub fn set_area(&mut self, area: f64) {
        self.area = area;
    }

    pub fn set_inertia(&mut self, inertia: f64) {
        self.inertia = inertia;
    }

    pub fn set_centre(&mut self, centre: &impl XYGet) {
        self.centre.set_x(centre.get_x());
        self.centre.set_y(centre.get_y());
    }
}

impl XYSet for BodyProperties {
    fn set_x(&mut self, x: f64) {
        self.centre.set_x(x);
    }

    fn set_y(&mut self, y: f64) {
        self.centre.set_y(y);
    }
}
