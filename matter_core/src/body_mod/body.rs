use std::{borrow::Borrow, cell::Ref};

use crate::{
    core::{
        collision_filter::CollisionFilter,
        common::{self, ShapeType},
        constraint_impulse::ConstraintImpulse,
        force::Force,
        position::Position,
        render::Render,
        sprite::Sprite,
        velocity::Velocity,
    },
    geometry::{
        axes,
        bounds::{self, Bounds},
        vector::{self, Vector},
        vertices::{self, Vertex},
    },
};
use itertools::Itertools;
use uuid::Uuid;

pub struct Body {
    id: Uuid,
    shape_type: ShapeType,
    //parts: Option<?>,
    //plugin: Option<?>,
    angle: f64,
    vertices: Vec<Vertex>,
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
    axes: Option<Vec<Vertex>>,
    area: f64,
    mass: f64,
    inverse_mass: f64,
    inertia: f64,
    inverse_inertia: f64,
    delta_time: f64,
    _original: Option<Box<Body>>,
}

pub enum BodyOption {
    ShapeType(ShapeType),
    //Parts:
    //Plugin:
    Angle(f64),
    Vertices(Vec<Vector>),
    Position(Position),
    Force(Force),
    Torque(f64),
    PositionImpulse(Position),
    ConstraintImpulse(ConstraintImpulse),
    TotalContracts(u32),
    Speed(f64),
    AngularSpeed(f64),
    Velocity(Velocity),
    AngularVelocity(f64),
    IsSensor(bool),
    IsStatic(bool),
    IsSleeping(bool),
    Motion(f64),
    SleepThreshold(u32),
    Density(f64),
    Resitution(f64),
    Friction(f64),
    FrictionStatic(f64),
    FrictionAir(f64),
    CollisionFilter(CollisionFilter),
    Slop(f64),
    TimeScale(u16),
    Render(Render),
    //Events
    Bounds(Bounds),
    Chamfer(Vec<Vector>),
    CircleRadius(f64),
    PositionPrev(Position),
    AnglePrev(f64),
    Parent(Body),
    Axes(Vec<Vector>),
    Area(f64),
    Mass(f64),
    Inertia(f64),
    DeltaTime(f64),
    Original(Body),
}

fn default_body<'a>() -> Body {
    let id = common::next_id();
    Body {
        id: id,
        shape_type: ShapeType::Body,
        //parts???
        //plugin??
        angle: 0.,
        vertices: vertices::from_path("L 0 0 L 40 0 L 40 40 L 0 40", id).unwrap(),
        position: Position::new(0., 0.),
        force: Force::new(0., 0.),
        torque: 0.,
        position_impulse: Position::new(0., 0.),
        constraint_impulse: ConstraintImpulse::new(0., 0., 0.),
        total_contacts: 0,
        speed: 0.,
        angular_speed: 0.,
        velocity: Velocity::new(0., 0.),
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
        collision_filter: CollisionFilter::new(1, u32::MAX, 0),
        slop: 0.05,
        time_scale: 1,
        render: Render::new(true, 1., Sprite::new(1., 1., 0., 0.)),
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

impl<'a> Body {
    fn get_bounds(&self) -> Option<Bounds> {
        self.bounds
    }

    fn get_axes(&self) -> Option<Vec<Vertex>> {
        self.axes.clone()
    }

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

const INERTIA_SCALE: f64 = 4.;
const NEXT_COLLIDING_GROUP_ID: i32 = 1;
const NEXT_NON_COLLIDING_GROUP_ID: i32 = -1;
const NEXT_CATEGORY: u16 = 1;
const BASE_DELTA: f64 = 1000. / 60.;
const TIME_CORRECTION: bool = true;

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

pub fn set_vertices(body: &mut Body, vertices: Vec<Vertex>) {
    let mut vertices = vertices;
    vertices
        .iter_mut()
        .for_each(|vertex| vertex.set_body_id(body.id));

    body.vertices = vertices;
    body.axes = Some(axes::from_vertices(&body.vertices));
    body.area = vertices::area(&body.vertices, false);
    set_mass(body, body.density * body.area);
    let centre = vertices::centre(&body.vertices);
    vertices::translate(&mut body.vertices, &centre, Some(-1.0));

    set_inertia(
        body,
        INERTIA_SCALE * vertices::innertia(&body.vertices, body.mass),
    );

    vertices::translate(&mut body.vertices, &body.position, None);
    if body.bounds.is_some() {
        let mut bounds = body.bounds.as_mut().unwrap();

        bounds::update(&mut bounds, &body.vertices, Some(&body.velocity));
        body.bounds = Some(bounds.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        geometry::bounds::BoundsPart,
        test_utils::{
            body_test_utils::{assert_position, assert_velocity},
            common_test_utils::assert_float,
            geometry_test_utils::{
                assert_bounds, assert_vertex, assert_xy, test_square, vec_vector_to_vec_vertex,
            },
        },
    };

    #[test]
    fn set_vertices_should_mutate_the_body_to_contain_valid_values() {
        // Arrange
        let vertices = vec_vector_to_vec_vertex(test_square());

        let mut body = default_body();
        body.id = common::next_id();
        body.inertia = 1706.6666666666667;
        body.inverse_inertia = 0.0005859375;
        body.mass = 1.6;
        body.inverse_mass = 0.625;
        body.density = 0.001;
        body.area = 1600.;
        body.position = Position::new(2., 2.);
        body.bounds = Some(Bounds {
            max: BoundsPart { x: 20.0, y: 20.0 },
            min: BoundsPart { x: -20.0, y: -20.0 },
        });
        body.velocity = Velocity::new(0., 0.);

        // Act
        set_vertices(&mut body, vertices);

        // Assert
        assert_float(body.area, 4.0);
        assert_xy(&body.get_axes().unwrap()[0], 0.0, 1.0);
        assert_xy(&body.get_axes().unwrap()[1], -1.0, 0.0);
        assert_bounds(&body.get_bounds().unwrap(), 1.0, 1.0, 3.0, 3.0);
        assert_float(body.density, 0.001);
        assert_float(body.inertia, 0.010666666666666666);
        assert_float(body.inverse_inertia, 93.75);
        assert_float(body.inverse_mass, 250.0);
        assert_float(body.mass, 0.004);
        assert_position(&body.position, 2.0, 2.0);
        assert_velocity(&body.velocity, 0.0, 0.0);
        assert_vertex(&body.vertices[0], body.id, 1.0, 1.0, 0, false);
        assert_vertex(&body.vertices[1], body.id, 3.0, 1.0, 1, false);
        assert_vertex(&body.vertices[2], body.id, 3.0, 3.0, 2, false);
        assert_vertex(&body.vertices[3], body.id, 1.0, 3.0, 3, false);
    }

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
