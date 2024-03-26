use std::fmt::LowerHex;

use uuid::Uuid;
use crate::{core::common::ShapeType, geometry::vector::Vector};

pub struct Position {
    pub x: f64,
    pub y: f64
}

pub struct Force {
    pub x: f64,
    pub y: f64
}

pub struct ConstraintImpulse {
    pub x: f64,
    pub y: f64,
    pub angle: f64
}

pub struct Velocity {
    pub x: f64,
    pub y: f64
}

pub struct CollisionFilter {
    pub category: LowerHex
}


pub struct Body {
    pub id: Uuid,
    pub shape_type: ShapeType,
    //parts
    //plugin
    pub angle: f64,
    pub vertices: Vec<Vector>,
    pub position: Position,
    pub force: Force,
    pub torque: f64,
    pub position_impulse: Position,
    pub constraint_impulse: ConstraintImpulse,
    pub total_contacts: u32,
    pub speed: f64,
    pub angular_speed: f64,
    pub velocity: Velocity,
    pub angular_velocity: f64,
    pub is_sensor: bool,
    pub is_static: bool,
    pub is_sleeping: bool,
    pub motion: f64,
    pub sleep_threshold: u32,
    pub density: f64,
    pub resitution: f64,
    pub friction: f64,
    pub friction_static: f64,
    pub friction_air: f64,


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_should_return_a_valid_body() {

    }
}