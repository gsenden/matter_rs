use std::{
    borrow::Borrow,
    cell::Ref,
    mem,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::{
    core::{
        collision_filter::CollisionFilter,
        common::{self, ShapeType},
        constraint_impulse::ConstraintImpulse,
        force::Force,
        position::{self, Position},
        render::Render,
        sprite::Sprite,
        velocity::{self, Velocity},
        xy::{XYFrom, XY},
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

use super::{body_option::BodyOption, body_properties::BodyProperties};

#[derive(Clone)]
pub struct Body {
    id: Uuid,
    shape_type: ShapeType,
    //plugin: Option<?>,
    angle: f64,
    parts: Option<Vec<Body>>,
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
    parent: Option<Weak<Body>>,
    axes: Option<Vec<Vertex>>,
    area: f64,
    mass: f64,
    inverse_mass: f64,
    inertia: f64,
    inverse_inertia: f64,
    delta_time: f64,
    _original: Option<Box<Body<'a>>>,
}
const INERTIA_SCALE: f64 = 4.;
const NEXT_COLLIDING_GROUP_ID: i32 = 1;
const NEXT_NON_COLLIDING_GROUP_ID: i32 = -1;
const NEXT_CATEGORY: u16 = 1;
const BASE_DELTA: f64 = 1000. / 60.;
const TIME_CORRECTION: bool = true;

impl<'a> Body {
    pub fn new(options: Option<Vec<BodyOption>>) -> Self {
        Self::default_body()
    }

    fn default_body() -> Self {
        let id = common::next_id();
        let mut body = Body {
            id: id,
            shape_type: ShapeType::Body,
            parts: None,
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
        };
        body.parts.push(Rc::new(&body));
        body
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_shape_type(&self) -> ShapeType {
        self.shape_type
    }

    pub fn get_parts(&'a self) -> Vec<&'a Body> {
        let mut parts: Vec<&'a Body> = Vec::new();
        parts.push(&self);
        if let Some(my_parts) = &self.parts {
            my_parts.iter().for_each(|part| parts.push(&part));
        }
        parts
    }

    pub fn get_angle(&self) -> f64 {
        self.angle
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_force(&self) -> Force {
        self.force
    }

    pub fn get_torque(&self) -> f64 {
        self.torque
    }

    pub fn get_position_impulse(&self) -> Position {
        self.position_impulse
    }

    pub fn get_constraint_impulse(&self) -> ConstraintImpulse {
        self.constraint_impulse
    }

    pub fn get_total_contacts(&self) -> u32 {
        self.total_contacts
    }

    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    pub fn get_angular_speed(&self) -> f64 {
        self.angular_speed
    }

    pub fn get_velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn get_angular_velocity(&self) -> f64 {
        self.angular_velocity
    }

    pub fn get_is_sensor(&self) -> bool {
        self.is_sensor
    }

    pub fn get_is_static(&self) -> bool {
        self.is_static
    }

    pub fn get_is_sleeping(&self) -> bool {
        self.is_sleeping
    }

    pub fn get_motion(&self) -> f64 {
        self.motion
    }

    pub fn get_sleep_threshold(&self) -> u32 {
        self.sleep_threshold
    }

    pub fn get_density(&self) -> f64 {
        self.density
    }

    pub fn get_resitution(&self) -> f64 {
        self.resitution
    }

    pub fn get_friction(&self) -> f64 {
        self.friction
    }

    pub fn get_friction_static(&self) -> f64 {
        self.friction_static
    }

    pub fn get_friction_air(&self) -> f64 {
        self.friction_air
    }

    pub fn get_collision_filter(&self) -> CollisionFilter {
        self.collision_filter
    }

    pub fn get_slop(&self) -> f64 {
        self.slop
    }

    pub fn get_time_scale(&self) -> u16 {
        self.time_scale
    }

    pub fn get_render(&self) -> Render {
        self.render
    }

    // pub fn get_events(&self) -> Option<Vec<Event>> {
    //     self.events
    // }

    pub fn get_bounds(&self) -> Option<Bounds> {
        self.bounds
    }

    pub fn get_chamfer(&self) -> Option<Vec<Vector>> {
        self.chamfer.clone()
    }

    pub fn get_circle_radius(&self) -> f64 {
        self.circle_radius
    }

    pub fn get_position_prev(&self) -> Option<Position> {
        self.position_prev
    }

    pub fn get_angle_prev(&self) -> f64 {
        self.angle_prev
    }

    pub fn get_axes(&self) -> Option<Vec<Vertex>> {
        self.axes.clone()
    }

    pub fn get_area(&self) -> f64 {
        self.area
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_inverse_mass(&self) -> f64 {
        self.inverse_mass
    }

    pub fn get_inertia(&self) -> f64 {
        self.inertia
    }

    pub fn get_inverse_inertia(&self) -> f64 {
        self.inverse_inertia
    }

    pub fn get_delta_time(&self) -> f64 {
        self.delta_time
    }

    pub fn update_bounds(&mut self, vertices: &Vec<Vertex>, velocity: Option<&Velocity>) {
        if self.bounds.is_some() {
            let mut bounds = self.bounds.unwrap();
            bounds::update(&mut bounds, &vertices, velocity);
            self.bounds = Some(bounds);
        }
    }

    pub fn set_inertia(&mut self, inertia: f64) {
        self.inertia = inertia;
        self.inverse_inertia = 1. / self.inertia;
    }

    pub fn set_mass(&mut self, mass: f64) {
        let moment = self.inertia / (self.mass / 6.);
        self.set_inertia(moment * (mass / 6.));
        self.mass = mass;
        self.inverse_mass = 1. / self.mass;
        self.density = self.mass / self.area;
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex>) {
        let mut vertices = vertices;
        vertices
            .iter_mut()
            .for_each(|vertex| vertex.set_body_id(self.id));

        self.vertices = vertices;
        self.axes = Some(axes::from_vertices(&self.vertices));
        self.area = vertices::area(&self.vertices, false);
        self.set_mass(self.density * self.area);
        let centre = vertices::centre(&self.vertices);
        vertices::translate(&mut self.vertices, &centre, Some(-1.0));

        self.set_inertia(INERTIA_SCALE * vertices::innertia(&self.vertices, self.mass));

        vertices::translate(&mut self.vertices, &self.position, None);
        if self.bounds.is_some() {
            let mut bounds = self.bounds.as_mut().unwrap();

            bounds::update(&mut bounds, &self.vertices, Some(&self.velocity));
            self.bounds = Some(bounds.clone());
        }
    }

    fn total_properties(&self) -> BodyProperties {
        let mut properties = BodyProperties::new(0., 0., 0., Position::new(0., 0.));

        //let mut skip_first = if self.parts.len() == 1 { false } else { true };
        if let Some(part) 
        self.parts.iter().for_each(|part| {
            // if skip_first {
            //     skip_first = false;
            // } else {
            let part = part.deref();
            let mass = if part.get_mass() != f64::INFINITY {
                part.get_mass()
            } else {
                1.
            };

            properties.set_mass(properties.get_mass() + mass);
            properties.set_area(properties.get_area() + part.get_area());
            properties.set_inertia(properties.get_inertia() + part.get_inertia());
            properties.set_centre(&vector::add(
                &properties.get_centre(),
                &vector::mult(&part.get_position(), mass),
            ));
            //}
        });

        properties.set_centre(&vector::div(
            &properties.get_centre(),
            properties.get_mass(),
        ));

        properties
    }

    fn set_position(&mut self, position: Position, update_velocity: bool) {
        let delta = vector::sub(&position, &self.position);
        if update_velocity {
            self.position_prev = Some(self.position.clone());
            self.velocity.set_xy(&delta);
            self.speed = vector::magnitude(&delta);
        } else {
            if let Some(ref mut position_prev) = self.position_prev {
                position_prev.add_xy(&delta)
            } else {
                self.position_prev = Some(Position::new_from_xy(&delta));
            }
        }

        let velocity = Some(self.get_velocity());

        self.parts.iter_mut().for_each(|part| {
            part.position.add_xy(&delta);
            vertices::translate(&mut part.vertices, &delta, None);
            part.update_bounds(&part.get_vertices(), velocity.as_ref());
        })
    }

    fn set_parts(&mut self, parts: Vec<Body>, auto_hull: Option<bool>) {
        let mut body_parts: Vec<Rc<Body>> = Vec::new();

        body_parts.push(Rc::new(&self));
        //self.parent = self.rc().parent.clone();
        let mut hull: Option<Vec<Vertex>> = None;
        let mut hull_centre: Option<Vector> = None;

        let auto_hull = auto_hull.unwrap_or(false);
        if auto_hull {
            let mut vertices: Vec<Vertex> = Vec::new();
            parts.iter().for_each(|part| {
                vertices.append(&mut part.get_vertices());
            });
            vertices::clockwise_sort(&mut vertices);
            vertices::hull(&mut vertices);
            hull = Some(vertices.clone());
            hull_centre = Some(vertices::centre(&vertices));
        }

        parts.into_iter().for_each(|part| {
            if part.get_id() != self.get_id() {
                let mut part = part;
                //part.parent = self.parent.clone();
                body_parts.push(Rc::new(part));
            }
        });
        let new_parts_len = body_parts.len();
        self.parts = body_parts;

        if new_parts_len == 1 {
            return;
        }

        if auto_hull && hull.is_some() && hull_centre.is_some() {
            self.set_vertices(hull.unwrap());
            vertices::translate(&mut self.vertices, &hull_centre.unwrap(), None);
        }

        let total = self.total_properties();
        self.area = total.get_area();
        self.position = total.get_centre();
        self.position_prev = Some(total.get_centre());
        self.set_mass(total.get_mass());
        self.set_inertia(total.get_inertia());
        self.set_position(total.get_centre(), false);

        // ??? //self.position = parts[0].position.clone();
        // self.vertices = parts[0].vertices.clone();
        // self.bounds = parts[0].bounds.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        core::position::{self, Position},
        geometry::bounds::BoundsPart,
        test_utils::{
            body_test_utils::{assert_position, assert_velocity},
            common_test_utils::assert_float,
            geometry_test_utils::{
                assert_bounds, assert_vertex, assert_xy, test_bounds, test_square,
                vec_vector_to_vec_vertex,
            },
        },
    };

    #[test]
    fn set_parts_should_update_body_with_parts_without_setting_autohull() {
        // Arrange
        let mut body = Body::default_body();
        body.id = common::next_id();
        body.mass = 1.6;
        body.area = 1600.;
        body.inertia = 1706.6666666666667;
        //body.parts = Some(vec![]);
        body.position = Position::new(2., 2.);
        body.position_prev = Some(Position::new(1., 1.));
        body.vertices = vec_vector_to_vec_vertex(test_square());
        body.bounds = Some(test_bounds().clone());
        let parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part = body.clone();
                part.id = common::next_id();
                part.mass += increase;
                part.area += increase;
                part.inertia += increase;
                part.position = Position::new(*increase, *increase);
                part
            })
            .collect_vec();
        let auto_hull = false;

        // Act
        body.set_parts(parts, Some(auto_hull));

        // Assert
        assert_float(body.get_area(), 3203.);
        assert_bounds(&body.get_bounds().unwrap(), 1., 1., 3., 3.);
        assert_float(body.get_density(), 0.0019356852950359038);
        assert_float(body.get_inertia(), 3416.3333333333335);
        assert_float(body.get_inverse_inertia(), 0.0002927114840472241);
        assert_float(body.get_inverse_mass(), 0.16129032258064516);
        assert_float(body.get_mass(), 6.2);
        assert_xy(&body.get_position(), 1.5806451612903227, 1.5806451612903227);
        assert_xy(
            &body.get_position_prev().unwrap(),
            1.5806451612903227,
            1.5806451612903227,
        );
        assert_xy(&body.get_vertices()[0], 1., 1.);
        assert_xy(&body.get_vertices()[1], 3., 1.);
        assert_xy(&body.get_vertices()[2], 3., 3.);
        assert_xy(&body.get_vertices()[3], 1., 3.);
        assert_eq!(body.get_parts().len(), 0);
        assert_eq!(body.get_vertices().len(), 4);

        let parts = body.get_parts();
        assert_float(parts[1].get_area(), 1601.);
        assert_bounds(&parts[1].get_bounds().unwrap(), 2., 2., 4., 4.);
        assert_float(parts[1].get_inertia(), 1707.6666666666667);
        assert_float(parts[1].get_mass(), 2.6);
        assert_xy(&parts[1].get_position(), 1., 1.);
        assert_xy(&parts[1].get_position_prev().unwrap(), 1., 1.);
        assert_xy(&parts[1].get_vertices()[0], 2., 2.);
        assert_xy(&parts[1].get_vertices()[1], 4., 2.);
        assert_xy(&parts[1].get_vertices()[2], 4., 4.);
        assert_xy(&parts[1].get_vertices()[3], 2., 4.);
        assert_eq!(parts[1].get_parts().len(), 0);
        assert_eq!(parts[1].get_vertices().len(), 4);

        assert_float(parts[2].get_area(), 1602.);
        assert_bounds(&parts[2].get_bounds().unwrap(), 3., 3., 5., 5.);
        assert_float(parts[2].get_inertia(), 1708.6666666666667);
        assert_float(parts[2].get_mass(), 3.6);
        assert_xy(&parts[2].get_position(), 2., 2.);
        assert_xy(&parts[2].get_position_prev().unwrap(), 1., 1.);
        assert_xy(&parts[2].get_vertices()[0], 3., 3.);
        assert_xy(&parts[2].get_vertices()[1], 5., 3.);
        assert_xy(&parts[2].get_vertices()[2], 5., 5.);
        assert_xy(&parts[2].get_vertices()[3], 3., 5.);
        assert_eq!(parts[2].get_parts().len(), 0);
        assert_eq!(parts[2].get_vertices().len(), 4);
    }

    #[test]
    fn set_bounds_should_update_body_with_position_and_setting_velocity() {
        // Arrange
        let mut body = Body::default_body();
        body.id = common::next_id();
        body.position = Position::new(2., 2.);
        body.position_prev = Some(Position::new(1., 1.));
        body.velocity = Velocity::new(42., 42.);
        body.bounds = Some(test_bounds());
        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part = body.clone();
                part.id = common::next_id();
                part.bounds = Some(test_bounds());
                part.position = Position::new(*increase, *increase);
                part.vertices = vec_vector_to_vec_vertex(test_square())
                    .iter_mut()
                    .map(|vertex| {
                        vertex.set_x(vertex.get_x() + increase);
                        vertex.set_y(vertex.get_y() + increase);
                        *vertex
                    })
                    .collect_vec();
                Rc::new(part)
            })
            .collect_vec();
        body.parts.append(&mut parts);

        let position = Position::new(37., 37.);
        let update_velocity = true;

        // Act
        body.set_position(position, update_velocity);

        // Assert
        assert_xy(&body.position, 2., 2.);
        assert_xy(&body.position_prev.unwrap(), 2., 2.);
        assert_xy(&body.velocity, 35., 35.);
        assert_float(body.speed, 49.49747468305833);
        assert_bounds(&body.get_parts()[0].bounds.unwrap(), 37., 37., 74., 74.);
        assert_xy(&body.get_parts()[0].position, 36., 36.);
        assert_xy(&body.get_parts()[0].velocity, 42., 42.);
        assert_xy(&body.get_parts()[0].vertices[0], 37., 37.);
        assert_xy(&body.get_parts()[0].vertices[1], 39., 37.);
        assert_xy(&body.get_parts()[0].vertices[2], 39., 39.);
        assert_xy(&body.get_parts()[0].vertices[3], 37., 39.);
        assert_bounds(&body.get_parts()[1].bounds.unwrap(), 38., 38., 75., 75.);
        assert_xy(&body.get_parts()[1].position, 37., 37.);
        assert_xy(&body.get_parts()[1].velocity, 42., 42.);
        assert_xy(&body.get_parts()[1].vertices[0], 38., 38.);
        assert_xy(&body.get_parts()[1].vertices[1], 40., 38.);
        assert_xy(&body.get_parts()[1].vertices[2], 40., 40.);
        assert_xy(&body.get_parts()[1].vertices[3], 38., 40.);
    }

    #[test]
    fn set_bounds_should_update_body_with_position_without_setting_velocity() {
        // Arrange
        let mut body = Body::default_body();
        body.id = common::next_id();
        body.position = Position::new(2., 2.);
        body.position_prev = Some(Position::new(1., 1.));
        body.velocity = Velocity::new(42., 42.);
        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part = body.clone();
                part.id = common::next_id();
                part.bounds = Some(test_bounds());
                part.position = Position::new(*increase, *increase);
                part.vertices = vec_vector_to_vec_vertex(test_square())
                    .iter_mut()
                    .map(|vertex| {
                        vertex.set_x(vertex.get_x() + increase);
                        vertex.set_y(vertex.get_y() + increase);
                        *vertex
                    })
                    .collect_vec();
                Rc::new(part)
            })
            .collect_vec();
        body.parts.append(&mut parts);

        let position = Position::new(37., 37.);
        let update_velocity = false;

        // Act
        body.set_position(position, update_velocity);

        // Assert
        assert_xy(&body.position, 2., 2.);
        assert_xy(&body.position_prev.unwrap(), 36., 36.);
        assert_xy(&body.velocity, 42., 42.);
        assert_bounds(&body.get_parts()[0].bounds.unwrap(), 37., 37., 81., 81.);
        assert_xy(&body.get_parts()[0].position, 36., 36.);
        assert_xy(&body.get_parts()[0].velocity, 42., 42.);
        assert_xy(&body.get_parts()[0].vertices[0], 37., 37.);
        assert_xy(&body.get_parts()[0].vertices[1], 39., 37.);
        assert_xy(&body.get_parts()[0].vertices[2], 39., 39.);
        assert_xy(&body.get_parts()[0].vertices[3], 37., 39.);
        assert_bounds(&body.get_parts()[1].bounds.unwrap(), 38., 38., 82., 82.);
        assert_xy(&body.get_parts()[1].position, 37., 37.);
        assert_xy(&body.get_parts()[1].velocity, 42., 42.);
        assert_xy(&body.get_parts()[1].vertices[0], 38., 38.);
        assert_xy(&body.get_parts()[1].vertices[1], 40., 38.);
        assert_xy(&body.get_parts()[1].vertices[2], 40., 40.);
        assert_xy(&body.get_parts()[1].vertices[3], 38., 40.);
    }

    #[test]
    fn total_properties_should_sum_the_properties_of_all_compound_parts_of_the_given_body() {
        // Arrange
        let mut body = Body::default_body();
        body.id = common::next_id();
        body.mass = 1.6;
        body.area = 1600.;
        body.inertia = 1706.6666666666667;
        let mut parts = [1., 2., 3., 4., 5.]
            .iter()
            .map(|increase| {
                let mut part = body.clone();
                part.id = common::next_id();
                part.mass += increase;
                part.area += increase;
                part.inertia += increase;
                part.position = Position::new(*increase, *increase);
                Rc::new(part)
            })
            .collect_vec();
        body.parts.append(&mut parts);

        // Act
        let result = body.total_properties();

        // Assert
        assert_float(result.get_area(), 6414.);
        assert_xy(&result.get_centre(), 3.7450980392156867, 3.7450980392156867);
        assert_float(result.get_inertia(), 6840.666666666667);
        assert_float(result.get_mass(), 20.4);
    }

    #[test]
    fn set_vertices_should_mutate_the_body_to_contain_valid_values() {
        // Arrange
        let vertices = vec_vector_to_vec_vertex(test_square());

        let mut body = Body::default_body();
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
        body.set_vertices(vertices);

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
        let mut body = Body::default_body();
        body.inertia = 1706.6666666666667;
        body.inverse_inertia = 0.0005859375;
        body.mass = 1.6;
        body.inverse_mass = 0.625;
        body.density = 0.001;
        body.area = 1600.;

        let mass = 42.1;

        // Act
        body.set_mass(mass);

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
        let mut body = Body::default_body();
        let inertia = 12.;

        // Act
        body.set_inertia(inertia);

        // Assert
        assert_float(body.inertia, 12.);
        assert_float(body.inverse_inertia, 0.08333333333333333);
    }
}
