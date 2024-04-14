use crate::core::position::Position;

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
}
