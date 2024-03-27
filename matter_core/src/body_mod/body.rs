use std::fmt::LowerHex;

use uuid::Uuid;
use crate::{core::common::ShapeType, geometry::{bounds::Bounds, vector::Vector}};

pub struct Position {
    x: f64,
    y: f64
}

pub struct Force {
    x: f64,
    y: f64
}

pub struct ConstraintImpulse {
    x: f64,
    y: f64,
    angle: f64
}

pub struct Velocity {
    x: f64,
    y: f64
}

pub struct CollisionFilter {
    category: u16,
    mask: u32,
    group: u16
}

pub struct Sprite {
    x_scale: f64,
    y_scale: f64,
    x_offset: f64,
    y_offset: f64
}

pub struct Render {
    visible: bool,
    opacity: f64,
    //stroke_style: Option<?>,
    //fill_style: Option<?>,
    //line_width: Option<?>,
    sprite: Sprite
}

pub struct Body {
    id: Uuid,
    shape_type: ShapeType,
    //parts: Option<?>,
    //plugin: Option<?>,
    angle: f64,
    vertices: Vec<Vector>,
    position: Position,
    force: Force,
    torque: f64,
    position_impulse: Position,
    constraint_impulse: ConstraintImpulse,
    total_contacts: u32,
    speed: f64,
    angular_speed: f64,
    velocity: Velocity,
    angular_velocity: f64,
    is_sensor: bool,
    is_static: bool,
    is_sleeping: bool,
    motion: f64,
    sleep_threshold: u32,
    density: f64,
    resitution: f64,
    friction: f64,
    friction_static: f64,
    friction_air: f64,
    collision_filter: CollisionFilter,
    slop: f64,
    time_scale: u16,
    render: Render,
    //events: Option<?>,
    bounds: Option<Bounds>,
    chamfer: Option<Vec<Vector>>,
    circle_radius: f64,
    position_prev: Option<Position>,
    angle_prev: f64,
    parent: Option<Box<Body>>,
    axes: Option<Vec<Vector>>,
    area: f64,
    mass: f64,
    inertia: f64,
    delta_time: f64,
    _original: Option<Box<Body>>
}

impl Body {
    fn create() -> Body {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_should_return_a_valid_body() {

    }
}