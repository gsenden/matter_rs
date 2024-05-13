use super::body::Body;

#[derive(Copy, Clone)]
pub struct BodyOriginal {
    density: f64,
    friction: f64,
    inertia: f64,
    inverse_inertia: f64,
    inverse_mass: f64,
    mass: f64,
    resitution: f64,
}

impl BodyOriginal {
    pub fn from(value: &Body) -> Self {
        BodyOriginal {
            density: value.get_density(),
            friction: value.get_friction(),
            inertia: value.get_inertia(),
            inverse_inertia: value.get_inverse_inertia(),
            inverse_mass: value.get_inverse_mass(),
            mass: value.get_mass(),
            resitution: value.get_resitution(),
        }
    }

    pub fn set_density(&mut self, value: f64) {
        self.density = value;
    }

    pub fn set_friction(&mut self, value: f64) {
        self.friction = value;
    }

    pub fn set_inertia(&mut self, value: f64) {
        self.inertia = value;
    }

    pub fn set_inverse_inertia(&mut self, value: f64) {
        self.inverse_inertia = value;
    }

    pub fn set_inverse_mass(&mut self, value: f64) {
        self.inverse_mass = value;
    }

    pub fn set_mass(&mut self, value: f64) {
        self.mass = value;
    }

    pub fn set_resitution(&mut self, value: f64) {
        self.resitution = value;
    }

    pub fn get_density(&self) -> f64 {
        self.density
    }

    pub fn get_friction(&self) -> f64 {
        self.friction
    }

    pub fn get_inertia(&self) -> f64 {
        self.inertia
    }

    pub fn get_inverse_inertia(&self) -> f64 {
        self.inverse_inertia
    }

    pub fn get_inverse_mass(&self) -> f64 {
        self.inverse_mass
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_resitution(&self) -> f64 {
        self.resitution
    }
}
