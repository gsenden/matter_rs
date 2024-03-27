use std::fmt::LowerHex;

use crate::{
    core::common::{self, ShapeType},
    geometry::{bounds::Bounds, vector::Vector, vertices},
};
use uuid::Uuid;

pub struct Position {
    x: f64,
    y: f64,
}

pub struct Force {
    x: f64,
    y: f64,
}

pub struct ConstraintImpulse {
    x: f64,
    y: f64,
    angle: f64,
}

pub struct Velocity {
    x: f64,
    y: f64,
}

pub struct CollisionFilter {
    category: u16,
    mask: u32,
    group: u16,
}

pub struct Sprite {
    x_scale: f64,
    y_scale: f64,
    x_offset: f64,
    y_offset: f64,
}

pub struct Render {
    visible: bool,
    opacity: f64,
    //stroke_style: Option<?>,
    //fill_style: Option<?>,
    //line_width: Option<?>,
    sprite: Sprite,
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
    inverse_mass: f64,
    inertia: f64,
    inverse_inertia: f64,
    delta_time: f64,
    _original: Option<Box<Body>>,
}

pub struct BodyOptions {
    shape_type: Option<ShapeType>,
    //parts: Option<?>,
    //plugin: Option<?>,
    angle: Option<f64>,
    vertices: Option<Vec<Vector>>,
    position: Option<Position>,
    force: Option<Force>,
    torque: Option<f64>,
    position_impulse: Option<Position>,
    constraint_impulse: Option<ConstraintImpulse>,
    total_contacts: Option<u32>,
    speed: Option<f64>,
    angular_speed: Option<f64>,
    velocity: Option<Velocity>,
    angular_velocity: Option<f64>,
    is_sensor: Option<bool>,
    is_static: Option<bool>,
    is_sleeping: Option<bool>,
    motion: Option<f64>,
    sleep_threshold: Option<u32>,
    density: Option<f64>,
    resitution: Option<f64>,
    friction: Option<f64>,
    friction_static: Option<f64>,
    friction_air: Option<f64>,
    collision_filter: Option<CollisionFilter>,
    slop: Option<f64>,
    time_scale: Option<u16>,
    render: Option<Render>,
    //events: Option<?>,
    bounds: Option<Bounds>,
    chamfer: Option<Vec<Vector>>,
    circle_radius: Option<f64>,
    position_prev: Option<Position>,
    angle_prev: Option<f64>,
    parent: Option<Box<Body>>,
    axes: Option<Vec<Vector>>,
    area: Option<f64>,
    mass: Option<f64>,
    inertia: Option<f64>,
    delta_time: Option<f64>,
    _original: Option<Box<Body>>,
}

fn default_body() -> Body {
    Body {
        id: common::next_id(),
        shape_type: ShapeType::Body,
        //parts???
        //plugin??
        angle: 0.,
        vertices: vertices::from_path("L 0 0 L 40 0 L 40 40 L 0 40").unwrap(),
        position: Position { x: 0., y: 0. },
        force: Force { x: 0., y: 0. },
        torque: 0.,
        position_impulse: Position { x: 0., y: 0. },
        constraint_impulse: ConstraintImpulse {
            x: 0.,
            y: 0.,
            angle: 0.,
        },
        total_contacts: 0,
        speed: 0.,
        angular_speed: 0.,
        velocity: Velocity { x: 0., y: 0. },
        angular_velocity: 0.,
        is_sensor: false,
        is_static: false,
        is_sleeping: false,
        motion: 0.,
        sleep_threshold: 60,
        density: 0.001,
        resitution: 0.,
        friction: 0.1,
        friction_static: 0.5,
        friction_air: 0.01,
        collision_filter: CollisionFilter {
            category: 1,
            mask: u32::MAX,
            group: 0,
        },
        slop: 0.05,
        time_scale: 1,
        render: Render {
            visible: true,
            opacity: 1.,
            sprite: Sprite {
                x_scale: 1.,
                y_scale: 1.,
                x_offset: 0.,
                y_offset: 0.,
            },
        },
        bounds: None,
        chamfer: None,
        circle_radius: 0.,
        position_prev: None,
        angle_prev: 0.,
        parent: None,
        axes: None,
        area: 0.,
        mass: 0.,
        inverse_mass: 0.,
        inertia: 0.,
        inverse_inertia: 0.,
        delta_time: 1000. / 60.,
        _original: None,
    }
}

impl Body {
    fn get_mass(&self) -> f64 {
        self.mass
    }

    fn get_iverse_mass(&self) -> f64 {
        self.inverse_mass
    }

    fn get_inertia(&self) -> f64 {
        self.inertia
    }

    fn get_inverse_inertia(&self) -> f64 {
        self.inverse_inertia
    }
}

pub fn set_inertia(body: &mut Body, inertia: f64) {
    body.inertia = inertia;
    body.inverse_inertia = 1. / body.inertia;
}

pub fn set_mass(body: &mut Body, mass: f64) {
    let moment = body.inertia / (body.mass / 6.);
    set_inertia(body, moment * (mass / 6.));
    body.mass = mass;
    body.inverse_mass = 1. / body.mass;
    body.density = body.mass / body.area;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::common_test_utils::assert_float;

    #[test]
    fn set_mass_should_mutate_value_of_mass_inverse_mass_inertia_inverse_inertia_and_density_to_valid_values(
    ) {
        // Arrange
        let mut body = default_body();
        body.inertia = 1706.6666666666667;
        body.inverse_inertia = 0.0005859375;
        body.mass = 1.6;
        body.inverse_mass = 0.625;
        body.density = 0.001;
        body.area = 1600.;

        let mass = 42.1;

        // Act
        set_mass(&mut body, mass);

        // Assert
        assert_float(body.mass, 42.1);
        assert_float(body.inverse_mass, 0.023752969121140142);
        assert_float(body.inertia, 44906.666666666664);
        assert_float(body.inverse_inertia, 0.000022268408551068885);
        assert_float(body.density, 0.026312500000000003);
    }

    #[test]
    fn set_inertia_should_mutate_value_of_inertia_and_inverse_inertia_to_valid_values() {
        // Arrange
        let mut body = default_body();
        let inertia = 12.;

        // Act
        set_inertia(&mut body, inertia);

        // Assert
        assert_float(body.inertia, 12.);
        assert_float(body.inverse_inertia, 0.08333333333333333);
    }
}
