use super::{body_option::BodyOption, body_properties::BodyProperties};
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
        xy::{XYNew, XY},
    },
    geometry::{
        axes,
        bounds::{self, Bounds},
        vector::{self, Vector},
        vertices::{self, Vertex},
    },
};
use core::time;
use std::{
    cell::{RefCell, RefMut},
    rc::{Rc, Weak},
};
use uuid::Uuid;

const INERTIA_SCALE: f64 = 4.;
const NEXT_COLLIDING_GROUP_ID: i32 = 1;
const NEXT_NON_COLLIDING_GROUP_ID: i32 = -1;
const NEXT_CATEGORY: u16 = 1;
const BASE_DELTA: f64 = 1000. / 60.;
const TIME_CORRECTION: bool = true;

#[derive(Clone)]
pub struct Body {
    content: Rc<RefCell<BodyContent>>,
    parent: Weak<RefCell<BodyContent>>,
}

//let this = self.content.as_ref().borrow();

#[derive(Clone)]
pub struct BodyContent {
    angle_prev: f64,
    angle: f64,
    angular_speed: f64,
    angular_velocity: f64,
    area: f64,
    axes: Option<Vec<Vertex>>,
    bounds: Option<Bounds>,
    chamfer: Option<Vec<Vector>>,
    circle_radius: f64,
    collision_filter: CollisionFilter,
    constraint_impulse: ConstraintImpulse,
    delta_time: f64,
    density: f64,
    //events: Option<?>,
    force: Force,
    friction_air: f64,
    friction_static: f64,
    friction: f64,
    id: Uuid,
    inertia: f64,
    inverse_inertia: f64,
    inverse_mass: f64,
    is_sensor: bool,
    is_sleeping: bool,
    is_static: bool,
    mass: f64,
    motion: f64,
    parts: Option<Vec<Body>>,
    //plugin: Option<?>,
    position_impulse: Position,
    position_prev: Option<Position>,
    position: Position,
    render: Render,
    resitution: f64,
    shape_type: ShapeType,
    sleep_threshold: u32,
    slop: f64,
    speed: f64,
    time_scale: u16,
    torque: f64,
    total_contacts: u32,
    velocity: Velocity,
    vertices: Vec<Vertex>,
    _original: Option<Box<Body>>,
}

impl BodyContent {
    fn default_contant() -> Self {
        BodyContent {
            angle_prev: 0.,
            angle: 0.,
            angular_speed: 0.,
            angular_velocity: 0.,
            area: 0.,
            axes: None,
            bounds: None,
            chamfer: None,
            circle_radius: 0.,
            collision_filter: CollisionFilter::new(1, u32::MAX, 0),
            constraint_impulse: ConstraintImpulse::new(0., 0., 0.),
            delta_time: 1000. / 60.,
            density: 0.001,
            force: Force::new(0., 0.),
            friction_air: 0.01,
            friction_static: 0.5,
            friction: 0.1,
            id: common::next_id(),
            inertia: 0.,
            inverse_inertia: 0.,
            inverse_mass: 0.,
            is_sensor: false,
            is_sleeping: false,
            is_static: false,
            mass: 0.,
            motion: 0.,
            parts: None,
            //plugin??
            position_impulse: Position::new(0., 0.),
            position_prev: None,
            position: Position::new(0., 0.),
            render: Render::new(true, 1., Sprite::new(1., 1., 0., 0.)),
            resitution: 0.,
            shape_type: ShapeType::Body,
            sleep_threshold: 60,
            slop: 0.05,
            speed: 0.,
            time_scale: 1,
            torque: 0.,
            total_contacts: 0,
            velocity: Velocity::new(0., 0.),
            vertices: vertices::from_path("L 0 0 L 40 0 L 40 40 L 0 40", None).unwrap(),
            _original: None,
        }
    }
}

macro_rules! content {
    ($a:expr) => {
        $a.content.as_ref().borrow()
    };
}

macro_rules! content_mut {
    ($a:expr) => {
        $a.content.as_ref().borrow_mut()
    };
}

impl Body {
    pub fn default_body() -> Self {
        let content = BodyContent::default_contant();
        let mut body = Body {
            content: Rc::new(RefCell::new(content)),
            parent: Weak::new(),
        };
        vertices::set_body(&mut body.content.as_ref().borrow_mut().vertices, &body);
        body
    }

    // pub fn new(options: Vec<BodyOption>) -> Self {
    //     let content = BodyContent { x: x, parts: None };
    //     Body { content: Rc::new(RefCell::new(content)), parent: Weak::new() }
    // }

    pub fn clone(&self) -> Body {
        Body {
            content: self.content.clone(),
            parent: self.parent.clone(),
        }
    }

    // region: Getters
    pub fn get_parent(&self) -> Option<Body> {
        if let Some(content) = self.parent.upgrade() {
            Some(Body {
                content,
                parent: Weak::new(),
            })
        } else {
            None
        }
    }

    pub fn get_id(&self) -> Uuid {
        content!(self).id
    }

    pub fn get_inertia(&self) -> f64 {
        content!(self).inertia
    }

    pub fn get_inverse_inertia(&self) -> f64 {
        content!(self).inverse_inertia
    }

    pub fn get_mass(&self) -> f64 {
        content!(self).mass
    }

    pub fn get_inverse_mass(&self) -> f64 {
        content!(self).inverse_mass
    }

    pub fn get_density(&self) -> f64 {
        content!(self).density
    }

    pub fn get_area(&self) -> f64 {
        content!(self).area
    }

    pub fn get_axes(&self) -> Option<Vec<Vertex>> {
        content!(self).axes.clone()
    }

    pub fn get_position(&self) -> Position {
        content!(self).position
    }

    pub fn get_bounds(&self) -> Option<Bounds> {
        content!(self).bounds
    }

    pub fn get_velocity_prop(&self) -> Velocity {
        content!(self).velocity
    }

    pub fn get_velocity(&self) -> Velocity {
        let content = content!(self);
        let time_scale = BASE_DELTA / content.delta_time;

        if let Some(position_prev) = &content.position_prev {
            let x = (content.position.get_x() - position_prev.get_x()) * time_scale;
            let y = (content.position.get_y() - position_prev.get_y()) * time_scale;
            Velocity::new(x, y)
        } else {
            let x = content.position.get_x() * time_scale;
            let y = content.position.get_y() * time_scale;
            Velocity::new(x, y)
        }
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        content!(self).vertices.clone()
    }

    pub fn get_position_prev(&self) -> Option<Position> {
        content!(self).position_prev
    }

    pub fn get_angle(&self) -> f64 {
        content!(self).angle
    }

    pub fn get_force(&self) -> Force {
        content!(self).force
    }

    pub fn get_torque(&self) -> f64 {
        content!(self).torque
    }

    pub fn get_position_impulse(&self) -> Position {
        content!(self).position_impulse
    }

    pub fn get_constraint_impulse(&self) -> ConstraintImpulse {
        content!(self).constraint_impulse
    }

    pub fn get_total_contacts(&self) -> u32 {
        content!(self).total_contacts
    }

    pub fn get_speed_prop(&self) -> f64 {
        content!(self).speed
    }

    pub fn get_speed(&self) -> f64 {
        vector::magnitude(&self.get_velocity())
    }

    fn get_angular_speed_prop(&self) -> f64 {
        content!(self).angular_speed
    }

    fn get_angular_velocity_prop(&self) -> f64 {
        content!(self).angular_velocity
    }

    pub fn get_is_sensor(&self) -> bool {
        content!(self).is_sensor
    }

    pub fn get_is_static(&self) -> bool {
        content!(self).is_static
    }

    pub fn get_is_sleeping(&self) -> bool {
        content!(self).is_sleeping
    }

    pub fn get_motion(&self) -> f64 {
        content!(self).motion
    }

    pub fn get_sleep_threshold(&self) -> u32 {
        content!(self).sleep_threshold
    }

    pub fn get_resitution(&self) -> f64 {
        content!(self).resitution
    }

    pub fn get_friction(&self) -> f64 {
        content!(self).friction
    }

    pub fn get_friction_static(&self) -> f64 {
        content!(self).friction_static
    }

    pub fn get_friction_air(&self) -> f64 {
        content!(self).friction_air
    }

    pub fn get_collision_filter(&self) -> CollisionFilter {
        content!(self).collision_filter
    }

    pub fn get_slop(&self) -> f64 {
        content!(self).slop
    }

    pub fn get_time_scale(&self) -> u16 {
        content!(self).time_scale
    }

    pub fn get_render(&self) -> Render {
        content!(self).render
    }

    pub fn get_shape_type(&self) -> ShapeType {
        content!(self).shape_type
    }

    // pub fn get_events(&self) -> Option<Vec<Event>> {
    //     content!(self).events
    // }

    pub fn get_chamfer(&self) -> Option<Vec<Vector>> {
        content!(self).chamfer.clone()
    }

    pub fn get_circle_radius(&self) -> f64 {
        content!(self).circle_radius
    }

    pub fn get_angle_prev(&self) -> f64 {
        content!(self).angle_prev
    }

    pub fn get_delta_time(&self) -> f64 {
        content!(self).delta_time
    }

    pub fn get_parts(&self) -> Vec<Body> {
        let mut parts = vec![self.clone()];
        if let Some(my_parts) = &content!(self).parts {
            for part in my_parts.iter() {
                parts.push(part.clone());
            }
        }
        parts
    }
    // endregion: Getters

    // region: Setters
    pub fn set_parent(&mut self, parent: &Body) {
        self.parent = Rc::downgrade(&parent.content);
    }

    pub fn set_inertia(&mut self, inertia: f64) {
        let mut content = content_mut!(self);
        content.inertia = inertia;
        content.inverse_inertia = 1. / content.inertia;
    }

    fn get_moment(&self) -> f64 {
        let content = content!(self);
        content.inertia / (content.mass / 6.)
    }

    pub fn set_mass(&mut self, mass: f64) {
        let moment = self.get_moment();
        self.set_inertia(moment * (mass / 6.));

        let mut content = content_mut!(self);
        content.mass = mass;
        content.inverse_mass = 1. / content.mass;
        content.density = content.mass / content.area;
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex>) {
        let mut vertices = vertices;
        vertices::set_body(&mut vertices, self);

        let mut density_area = 0.;
        {
            let mut content = content_mut!(self);
            content.vertices = vertices;
            content.axes = Some(axes::from_vertices(&content.vertices));
            content.area = vertices::area(&content.vertices, false);
            density_area = content.density * content.area;
        }

        self.set_mass(density_area);

        let mut inertia = 0.;
        {
            let mut content = content_mut!(self);
            let centre = vertices::centre(&content.vertices);
            vertices::translate(&mut content.vertices, &centre, Some(-1.0));
            inertia = INERTIA_SCALE * vertices::innertia(&content.vertices, content.mass);
        }
        self.set_inertia(inertia);

        {
            let mut content = content_mut!(self);
            let position = content.position;

            vertices::translate(&mut content.vertices, &position, None);
            if let Some(mut bounds) = &content.bounds {
                bounds::update(&mut bounds, &content.vertices, &Some(&content.velocity));
                content.bounds = Some(bounds.clone());
            }
        }
    }

    fn total_properties(&self) -> BodyProperties {
        let mut properties = BodyProperties::new(0., 0., 0., Position::new(0., 0.));

        let parts = self.get_parts();
        let part_count = parts.len();
        let mut index: usize = if part_count == 1 { 0 } else { 1 };

        while index < part_count {
            let part = &parts[index];
            let mass = if part.get_mass() != f64::INFINITY {
                part.get_mass()
            } else {
                1.
            };
            properties.set_mass(properties.get_mass() + mass);
            properties.set_area(properties.get_area() + part.get_area());
            properties.set_inertia(properties.get_inertia() + part.get_inertia());
            let position_times_mass = vector::mult(&part.get_position(), mass);
            let centre = vector::add(&properties.get_centre(), &position_times_mass);
            properties.set_centre(&centre);

            index += 1;
        }

        let centre = vector::div(&properties.get_centre(), properties.get_mass());
        properties.set_centre(&centre);

        properties
    }

    pub fn set_position(&mut self, position: Position, update_velocity: Option<bool>) {
        let update_velocity = update_velocity.unwrap_or(false);
        let mut delta: Vector = vector::create(0., 0.);
        let mut parent_velocity = Velocity::new(0., 0.);

        {
            let mut content = content_mut!(self);
            delta = vector::sub(&position, &content.position);

            if update_velocity {
                content.position_prev = Some(content.position.clone());
                content.velocity.set_xy(&delta);
                content.speed = vector::magnitude(&delta);
            } else {
                if let Some(ref mut position_prev) = content.position_prev {
                    position_prev.add_xy(&delta)
                } else {
                    content.position_prev = Some(Position::new_from(&delta));
                }
            }
            parent_velocity = content.velocity.clone();
        }

        for part in self.get_parts().iter_mut() {
            let mut part_content = content_mut!(part);

            part_content.position.add_xy(&delta);
            vertices::translate(&mut part_content.vertices, &delta, None);
            let vertices = part_content.vertices.clone();
            if let Some(bounds) = &mut part_content.bounds {
                bounds::update(bounds, &vertices, &Some(&parent_velocity));
            }
        }
    }

    pub fn set_parts(&mut self, parts: Vec<Body>, auto_hull: Option<bool>) {
        let mut parts = parts;
        for part in &mut parts {
            part.set_parent(self);
        }

        let auto_hull = auto_hull.unwrap_or(true);
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut hull_centre: Vector = vector::create(0., 0.);
        {
            let mut content = content_mut!(self);
            if parts.len() > 0 {
                content.parts = Some(parts);
            } else {
                content.parts = None;
                return;
            }

            if auto_hull {
                if let Some(my_parts) = &content.parts {
                    for part in my_parts.iter() {
                        let part_content = content!(part);
                        vertices.append(&mut part_content.vertices.clone())
                    }
                }
                vertices::clockwise_sort(&mut vertices);
                vertices::hull(&mut vertices);
                hull_centre = vertices::centre(&vertices);
            }
        }
        if auto_hull {
            self.set_vertices(vertices);
            {
                let mut content = content_mut!(self);
                vertices::translate(&mut content.vertices, &hull_centre, None);
            }
        }

        let total = self.total_properties();
        {
            let mut content = content_mut!(self);
            content.area = total.get_area();
            content.position = total.get_centre();
            content.position_prev = Some(total.get_centre());
        }
        self.set_mass(total.get_mass());
        self.set_inertia(total.get_inertia());
        self.set_position(total.get_centre(), None);
    }

    pub fn set_centre(&mut self, centre: &Position, relative: Option<bool>) {
        let relative = relative.unwrap_or(false);
        let mut content = content_mut!(self);
        let position = content.position;
        if !relative {
            if let Some(position_prev) = &mut content.position_prev {
                position_prev.set_x(centre.get_x() - (position.get_x() - position_prev.get_x()));
                position_prev.set_y(centre.get_y() - (position.get_y() - position_prev.get_y()));
            } else {
                let position_prev = Position::new(
                    centre.get_x() - position.get_x(),
                    centre.get_y() - position.get_y(),
                );
                content.position_prev = Some(position_prev);
            }
            content.position.set_xy(centre);
        } else {
            if let Some(position_prev) = &mut content.position_prev {
                position_prev.add_xy(centre);
            } else {
                content.position_prev = Some(centre.clone());
            }
            content.position.add_xy(centre);
        }
    }

    pub fn set_angle(&mut self, angle: f64, update_velocity: Option<bool>) {
        let update_velocity = update_velocity.unwrap_or(false);

        let mut delta = 0.;
        let mut body_position = Position::new(0., 0.);
        let mut body_velocity = Velocity::new(0., 0.);
        let mut body_id: Uuid = Uuid::new_v4();

        {
            let mut content = content_mut!(self);
            delta = angle - content.angle;

            if update_velocity {
                content.angle_prev = content.angle;
                content.angular_velocity = delta;
                content.angular_speed = f64::abs(delta);
            } else {
                content.angle_prev += delta;
            }

            body_position = content.position;
            body_velocity = content.velocity;
            body_id = content.id;
        }

        for part in &mut self.get_parts() {
            let mut part_content = content_mut!(part);
            part_content.angle += delta;
            vertices::rotate(&mut part_content.vertices, delta, &body_position);
            if let Some(axes) = &mut part_content.axes {
                axes::rotate(axes, delta);
            }
            let vertices = part_content.vertices.clone();
            if let Some(bounds) = &mut part_content.bounds {
                bounds::update(bounds, &vertices, &Some(&body_velocity));
            }
            if part_content.id != body_id {
                vector::rotate_about(&mut part_content.position, delta, &body_position);
            }
        }
    }

    pub fn set_velocity(&mut self, velocity: &Velocity) {
        let mut content = content_mut!(self);
        let time_scale = content.delta_time / BASE_DELTA;

        let position_prev = Position::new(
            content.position.get_x() - velocity.get_x() * time_scale,
            content.position.get_y() - velocity.get_y() * time_scale,
        );

        let velocity = Velocity::new(
            (content.position.get_x() - position_prev.get_x()) / time_scale,
            (content.position.get_y() - position_prev.get_y()) / time_scale,
        );

        content.position_prev = Some(position_prev);
        content.velocity = velocity;
        content.speed = vector::magnitude(&content.velocity);
    }

    pub fn set_speed(&mut self, speed: f64) {
        let velocity = self.get_velocity();
        let normalised = vector::normalise(&velocity);
        let velocity = vector::mult(&normalised, speed);
        let velocity = Velocity::new_from(&velocity);
        self.set_velocity(&velocity)
    }

    pub fn set_angular_velocity(&mut self, velocity: f64) {
        let mut content = content_mut!(self);
        let time_scale = content.delta_time / BASE_DELTA;
        content.angle_prev = content.angle - velocity * time_scale;
        content.angular_velocity = (content.angle - content.angle_prev) / time_scale;
        content.angular_speed = f64::abs(content.angular_velocity);
    }

    // endregion: Setters
}

//MARK: Tests

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use crate::{
        core::{
            position::{self, Position},
            xy::XY,
        },
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

    fn body_from_content(content: BodyContent) -> Body {
        Body {
            content: Rc::new(RefCell::new(content)),
            parent: Weak::new(),
        }
    }

    #[test]
    fn set_angular_velocity_should_be_able_to_set_the_angular_velocity_on_a_body() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.angle = 42.;
        content.angle_prev = 41.;
        let mut body = body_from_content(content);
        let velocity = 37.;

        // Act
        body.set_angular_velocity(velocity);

        // Assert
        assert_float(body.get_angle(), 42.);
        assert_float(body.get_angle_prev(), 5.);
        assert_float(body.get_angular_velocity_prop(), 37.);
        assert_float(body.get_angular_speed_prop(), 37.);
    }

    #[test]
    fn set_speed_should_be_able_to_set_the_speed_on_a_body() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.position = Position::new(37., 37.);
        content.position_prev = Some(Position::new(36., 36.));
        let mut body = body_from_content(content);
        let speed = 42.;

        // Act
        body.set_speed(speed);

        // Assert
        assert_xy(&body.get_position(), 37., 37.);
        assert_xy(
            &body.get_position_prev().unwrap(),
            7.301515190165006,
            7.301515190165006,
        );
        assert_float(body.get_speed_prop(), 42.);
        assert_float(body.get_speed(), 42.);
        assert_xy(
            &body.get_velocity_prop(),
            29.698484809834994,
            29.698484809834994,
        );
        assert_xy(&body.get_velocity(), 29.698484809834994, 29.698484809834994);
    }

    #[test]
    fn should_be_able_to_set_the_velocity_on_a_body() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.position = Position::new(37., 37.);
        content.position_prev = Some(Position::new(36., 36.));
        let mut body = body_from_content(content);
        let velocity = Velocity::new(42., 43.);

        // Act
        body.set_velocity(&velocity);

        // Assert
        assert_xy(&body.get_position(), 37., 37.);
        assert_xy(&body.get_position_prev().unwrap(), -5., -6.);
        assert_float(body.get_speed_prop(), 60.108235708594876);
        assert_float(body.get_speed(), 60.108235708594876);
        assert_xy(&body.get_velocity_prop(), 42., 43.);
        assert_xy(&body.get_velocity(), 42., 43.);
    }

    #[test]
    fn set_angle_should_be_able_to_set_the_angle_on_a_default_body_updating_the_velocity() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.angle = 42.;
        content.angle_prev = 41.;
        content.axes = Some(vec![
            Vertex::new(None, 1., 1., 0, false),
            Vertex::new(None, -1., -1., 1, false),
        ]);
        content.position = Position::new(0., 0.);
        content.bounds = Some(test_bounds());
        content.vertices = vec_vector_to_vec_vertex(test_square());

        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.angle += increase;
                part_content.angle_prev += increase;
                if let Some(axes) = &mut part_content.axes {
                    axes[0].add_x_y(*increase, *increase);
                    axes[1].add_x_y(-1. * increase, -1. * increase);
                }
                part_content.bounds = Some(test_bounds());
                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = vec_vector_to_vec_vertex(test_square())
                    .iter_mut()
                    .map(|vertex| {
                        vertex.set_x(vertex.get_x() + increase);
                        vertex.set_y(vertex.get_y() + increase);
                        vertex.clone()
                    })
                    .collect_vec();
                body_from_content(part_content)
            })
            .collect_vec();
        content.parts = Some(parts);
        let mut body = body_from_content(content);

        let update_velocity = Some(true);

        // Act
        body.set_angle(37., update_velocity);

        let part = body.get_parts()[0].clone();
        assert_float(part.get_angle(), 37.);
        assert_float(part.get_angle_prev(), 42.);
        assert_float(part.get_angular_speed_prop(), 5.);

        assert_float(part.get_angular_velocity_prop(), -5.);
        assert_xy(
            &part.get_axes().unwrap()[0],
            -0.6752620891999122,
            1.2425864601263648,
        );
        assert_xy(
            &part.get_axes().unwrap()[1],
            0.6752620891999122,
            -1.2425864601263648,
        );
        assert_bounds(
            &part.get_bounds().unwrap(),
            -2.593110638526189,
            1.2425864601263648,
            -0.10793771827345966,
            3.7277593803790943,
        );
        assert_xy(&part.get_position(), 0., 0.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -0.6752620891999122, 1.2425864601263648);
        assert_xy(&vertices[1], -0.10793771827345966, 3.1604350094526414);
        assert_xy(&vertices[2], -2.0257862675997362, 3.7277593803790943);
        assert_xy(&vertices[3], -2.593110638526189, 1.8099108310528171);

        let part = body.get_parts()[1].clone();
        assert_float(part.get_angle(), 38.);
        assert_float(part.get_angle_prev(), 42.);
        assert_xy(
            &part.get_axes().unwrap()[0],
            -1.3505241783998243,
            2.4851729202527295,
        );
        assert_xy(
            &part.get_axes().unwrap()[1],
            1.3505241783998243,
            -2.4851729202527295,
        );
        assert_bounds(
            &part.get_bounds().unwrap(),
            -3.2683727277261014,
            2.4851729202527295,
            -0.7831998074733719,
            4.970345840505459,
        );
        assert_xy(
            &part.get_position(),
            -0.6752620891999122,
            1.2425864601263648,
        );
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -1.3505241783998243, 2.4851729202527295);
        assert_xy(&vertices[1], -0.7831998074733719, 4.403021469579007);
        assert_xy(&vertices[2], -2.7010483567996486, 4.970345840505459);
        assert_xy(&vertices[3], -3.2683727277261014, 3.052497291179182);

        let part = body.get_parts()[2].clone();
        assert_float(part.get_angle(), 39.);
        assert_float(part.get_angle_prev(), 43.);
        assert_xy(
            &part.get_axes().unwrap()[0],
            -2.0257862675997362,
            3.7277593803790943,
        );
        assert_xy(
            &part.get_axes().unwrap()[1],
            2.0257862675997362,
            -3.7277593803790943,
        );
        assert_bounds(
            &part.get_bounds().unwrap(),
            -3.9436348169260134,
            3.7277593803790943,
            -1.458461896673284,
            6.212932300631824,
        );
        assert_xy(
            &part.get_position(),
            -1.3505241783998243,
            2.4851729202527295,
        );
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -2.0257862675997362, 3.7277593803790943);
        assert_xy(&vertices[1], -1.458461896673284, 5.645607929705371);
        assert_xy(&vertices[2], -3.376310445999561, 6.212932300631824);
        assert_xy(&vertices[3], -3.9436348169260134, 4.295083751305547);
    }

    #[test]
    fn set_angle_should_be_able_to_set_the_angle_on_a_default_body_not_updating_the_velocity() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.angle = 42.;
        content.angle_prev = 41.;
        content.axes = Some(vec![
            Vertex::new(None, 1., 1., 0, false),
            Vertex::new(None, -1., -1., 1, false),
        ]);
        content.position = Position::new(0., 0.);
        content.bounds = Some(test_bounds());
        content.vertices = vec_vector_to_vec_vertex(test_square());

        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.angle += increase;
                part_content.angle_prev += increase;
                if let Some(axes) = &mut part_content.axes {
                    axes[0].add_x_y(*increase, *increase);
                    axes[1].add_x_y(-1. * increase, -1. * increase);
                }
                part_content.bounds = Some(test_bounds());
                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = vec_vector_to_vec_vertex(test_square())
                    .iter_mut()
                    .map(|vertex| {
                        vertex.set_x(vertex.get_x() + increase);
                        vertex.set_y(vertex.get_y() + increase);
                        vertex.clone()
                    })
                    .collect_vec();
                body_from_content(part_content)
            })
            .collect_vec();
        content.parts = Some(parts);
        let mut body = body_from_content(content);

        let update_velocity = None;

        // Act
        body.set_angle(37., update_velocity);

        let part = body.get_parts()[0].clone();
        assert_float(part.get_angle(), 37.);
        assert_float(part.get_angle_prev(), 36.);
        assert_xy(
            &part.get_axes().unwrap()[0],
            -0.6752620891999122,
            1.2425864601263648,
        );
        assert_xy(
            &part.get_axes().unwrap()[1],
            0.6752620891999122,
            -1.2425864601263648,
        );
        assert_bounds(
            &part.get_bounds().unwrap(),
            -2.593110638526189,
            1.2425864601263648,
            -0.10793771827345966,
            3.7277593803790943,
        );
        assert_xy(&part.get_position(), 0., 0.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -0.6752620891999122, 1.2425864601263648);
        assert_xy(&vertices[1], -0.10793771827345966, 3.1604350094526414);
        assert_xy(&vertices[2], -2.0257862675997362, 3.7277593803790943);
        assert_xy(&vertices[3], -2.593110638526189, 1.8099108310528171);

        let part = body.get_parts()[1].clone();
        assert_float(part.get_angle(), 38.);
        assert_float(part.get_angle_prev(), 42.);
        assert_xy(
            &part.get_axes().unwrap()[0],
            -1.3505241783998243,
            2.4851729202527295,
        );
        assert_xy(
            &part.get_axes().unwrap()[1],
            1.3505241783998243,
            -2.4851729202527295,
        );
        assert_bounds(
            &part.get_bounds().unwrap(),
            -3.2683727277261014,
            2.4851729202527295,
            -0.7831998074733719,
            4.970345840505459,
        );
        assert_xy(
            &part.get_position(),
            -0.6752620891999122,
            1.2425864601263648,
        );
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -1.3505241783998243, 2.4851729202527295);
        assert_xy(&vertices[1], -0.7831998074733719, 4.403021469579007);
        assert_xy(&vertices[2], -2.7010483567996486, 4.970345840505459);
        assert_xy(&vertices[3], -3.2683727277261014, 3.052497291179182);

        let part = body.get_parts()[2].clone();
        assert_float(part.get_angle(), 39.);
        assert_float(part.get_angle_prev(), 43.);
        assert_xy(
            &part.get_axes().unwrap()[0],
            -2.0257862675997362,
            3.7277593803790943,
        );
        assert_xy(
            &part.get_axes().unwrap()[1],
            2.0257862675997362,
            -3.7277593803790943,
        );
        assert_bounds(
            &part.get_bounds().unwrap(),
            -3.9436348169260134,
            3.7277593803790943,
            -1.458461896673284,
            6.212932300631824,
        );
        assert_xy(
            &part.get_position(),
            -1.3505241783998243,
            2.4851729202527295,
        );
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -2.0257862675997362, 3.7277593803790943);
        assert_xy(&vertices[1], -1.458461896673284, 5.645607929705371);
        assert_xy(&vertices[2], -3.376310445999561, 6.212932300631824);
        assert_xy(&vertices[3], -3.9436348169260134, 4.295083751305547);
    }

    #[test]
    fn set_centre_should_be_able_to_set_the_centre_on_a_default_body_relative() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.position = Position::new(2., 2.);
        content.position_prev = Some(Position::new(1., 1.));
        let mut body = body_from_content(content);

        let centre = Position::new(42., 43.);
        let relative = Some(true);

        // Act
        body.set_centre(&centre, relative);

        // Assert
        assert_xy(&body.get_position(), 44., 45.);
        assert_xy(&body.get_position_prev().unwrap(), 43., 44.);
    }

    #[test]
    fn set_centre_should_be_able_to_set_the_centre_on_a_default_body_not_relative() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.position = Position::new(2., 2.);
        content.position_prev = Some(Position::new(1., 1.));
        let mut body = body_from_content(content);

        let centre = Position::new(42., 43.);
        let relative = None;

        // Act
        body.set_centre(&centre, relative);

        // Assert
        assert_xy(&body.get_position(), 42., 43.);
        assert_xy(&body.get_position_prev().unwrap(), 41., 42.);
    }

    #[test]
    fn set_parts_should_update_body_with_parts_with_setting_autohull_to_true() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.mass = 1.6;
        content.area = 1600.;
        content.inertia = 1706.6666666666667;
        content.position = Position::new(2., 2.);
        content.position_prev = Some(Position::new(1., 1.));
        content.bounds = Some(test_bounds());
        content.vertices = vec_vector_to_vec_vertex(test_square());
        content.bounds = Some(test_bounds());

        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.mass += increase;
                part_content.area += increase;
                part_content.inertia += increase;
                part_content.bounds = Some(test_bounds());
                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = vec_vector_to_vec_vertex(test_square())
                    .iter_mut()
                    .map(|vertex| {
                        vertex.set_x(vertex.get_x() + increase);
                        vertex.set_y(vertex.get_y() + increase);
                        vertex.clone()
                    })
                    .collect_vec();
                body_from_content(part_content)
            })
            .collect_vec();

        let mut body = body_from_content(content);

        let auto_hull = true;

        // Act
        body.set_parts(parts, Some(auto_hull));

        // Assert
        assert_float(body.get_area(), 3203.);
        assert_bounds(&body.get_bounds().unwrap(), 4., 4., 7., 7.);
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
        assert_xy(&body.get_vertices()[0], 7., 7.);
        assert_xy(&body.get_vertices()[1], 5., 7.);
        assert_xy(&body.get_vertices()[2], 4., 6.);
        assert_xy(&body.get_vertices()[3], 4., 4.);
        assert_xy(&body.get_vertices()[4], 6., 4.);
        assert_xy(&body.get_vertices()[5], 7., 5.);

        assert_eq!(body.get_parts().len(), 3);
        assert_eq!(body.get_vertices().len(), 6);

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
        assert_eq!(parts[2].get_vertices().len(), 4);
    }

    #[test]
    fn set_parts_should_update_body_with_parts_without_setting_autohull() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.mass = 1.6;
        content.area = 1600.;
        content.inertia = 1706.6666666666667;
        content.position = Position::new(2., 2.);
        content.position_prev = Some(Position::new(1., 1.));
        content.bounds = Some(test_bounds());
        content.vertices = vec_vector_to_vec_vertex(test_square());
        content.bounds = Some(test_bounds());

        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.mass += increase;
                part_content.area += increase;
                part_content.inertia += increase;
                part_content.bounds = Some(test_bounds());
                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = vec_vector_to_vec_vertex(test_square())
                    .iter_mut()
                    .map(|vertex| {
                        vertex.set_x(vertex.get_x() + increase);
                        vertex.set_y(vertex.get_y() + increase);
                        vertex.clone()
                    })
                    .collect_vec();
                body_from_content(part_content)
            })
            .collect_vec();

        let mut body = body_from_content(content);

        let auto_hull = Some(false);

        // Act
        body.set_parts(parts, auto_hull);

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
        assert_eq!(body.get_parts().len(), 3);
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
        assert_eq!(parts[2].get_vertices().len(), 4);
    }

    #[test]
    fn set_position_should_update_body_with_position_and_setting_velocity() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.position = Position::new(2., 2.);
        content.position_prev = Some(Position::new(1., 1.));
        content.bounds = Some(test_bounds());
        content.vertices = vec_vector_to_vec_vertex(test_square());
        content.velocity = Velocity::new(42., 42.);

        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.bounds = Some(test_bounds());
                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = vec_vector_to_vec_vertex(test_square())
                    .iter_mut()
                    .map(|vertex| {
                        vertex.set_x(vertex.get_x() + increase);
                        vertex.set_y(vertex.get_y() + increase);
                        vertex.clone()
                    })
                    .collect_vec();
                body_from_content(part_content)
            })
            .collect_vec();
        content.parts = Some(parts);
        let mut body = body_from_content(content);

        let position = Position::new(37., 37.);
        let update_velocity = Some(true);

        // Act
        body.set_position(position, update_velocity);

        // Assert
        assert_xy(&body.get_position(), 37., 37.);
        assert_xy(&body.get_position_prev().unwrap(), 2., 2.);
        assert_float(body.get_speed_prop(), 49.49747468305833);
        assert_float(body.get_speed(), 49.49747468305833);
        assert_xy(&body.get_velocity_prop(), 35., 35.);

        let parts = &body.get_parts();
        let part = parts[0].clone();
        assert_bounds(&part.get_bounds().unwrap(), 36., 36., 73., 73.);
        assert_xy(&part.get_position(), 37., 37.);
        assert_xy(&part.get_velocity_prop(), 35., 35.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 36., 36.);
        assert_xy(&vertices[1], 38., 36.);
        assert_xy(&vertices[2], 38., 38.);
        assert_xy(&vertices[3], 36., 38.);

        let part = parts[1].clone();
        assert_bounds(&part.get_bounds().unwrap(), 37., 37., 74., 74.);
        assert_xy(&part.get_position(), 36., 36.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 37., 37.);
        assert_xy(&vertices[1], 39., 37.);
        assert_xy(&vertices[2], 39., 39.);
        assert_xy(&vertices[3], 37., 39.);

        let part = parts[2].clone();
        assert_bounds(&part.get_bounds().unwrap(), 38., 38., 75., 75.);
        assert_xy(&part.get_position(), 37., 37.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 38., 38.);
        assert_xy(&vertices[1], 40., 38.);
        assert_xy(&vertices[2], 40., 40.);
        assert_xy(&vertices[3], 38., 40.);
    }

    #[test]
    fn set_position_should_update_body_with_position_without_setting_velocity() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.position = Position::new(2., 2.);
        content.position_prev = Some(Position::new(1., 1.));
        content.bounds = Some(test_bounds());
        content.vertices = vec_vector_to_vec_vertex(test_square());
        content.velocity = Velocity::new(42., 42.);

        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.bounds = Some(test_bounds());
                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = vec_vector_to_vec_vertex(test_square())
                    .iter_mut()
                    .map(|vertex| {
                        vertex.set_x(vertex.get_x() + increase);
                        vertex.set_y(vertex.get_y() + increase);
                        vertex.clone()
                    })
                    .collect_vec();
                body_from_content(part_content)
            })
            .collect_vec();
        content.parts = Some(parts);
        let mut body = body_from_content(content);

        let position = Position::new(37., 37.);
        let update_velocity = None;

        // Act
        body.set_position(position, update_velocity);

        // Assert
        assert_xy(&body.get_position(), 37., 37.);
        assert_xy(&body.get_position_prev().unwrap(), 36., 36.);
        assert_xy(&body.get_velocity_prop(), 42., 42.);

        let parts = &body.get_parts();
        let part = parts[0].clone();
        assert_bounds(&part.get_bounds().unwrap(), 36., 36., 80., 80.);
        assert_xy(&part.get_position(), 37., 37.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 36., 36.);
        assert_xy(&vertices[1], 38., 36.);
        assert_xy(&vertices[2], 38., 38.);
        assert_xy(&vertices[3], 36., 38.);

        let part = parts[1].clone();
        assert_bounds(&part.get_bounds().unwrap(), 37., 37., 81., 81.);
        assert_xy(&part.get_position(), 36., 36.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 37., 37.);
        assert_xy(&vertices[1], 39., 37.);
        assert_xy(&vertices[2], 39., 39.);
        assert_xy(&vertices[3], 37., 39.);

        let part = parts[2].clone();
        assert_bounds(&part.get_bounds().unwrap(), 38., 38., 82., 82.);
        assert_xy(&part.get_position(), 37., 37.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 38., 38.);
        assert_xy(&vertices[1], 40., 38.);
        assert_xy(&vertices[2], 40., 40.);
        assert_xy(&vertices[3], 38., 40.);
    }

    #[test]
    fn total_properties_should_sum_the_properties_of_all_compound_parts_of_the_given_body() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.mass = 1.6;
        content.area = 1600.;
        content.inertia = 1706.6666666666667;
        let mut parts = [2., 3., 4., 5.] // different from Javascript since parent is not included in Rust
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.mass += increase;
                part_content.area += increase;
                part_content.inertia += increase;

                part_content.angle = *increase;
                part_content.position = Position::new(*increase, *increase);
                body_from_content(part_content)
            })
            .collect_vec();

        content.parts = Some(parts);
        let body = body_from_content(content);

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

        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.inertia = 1706.6666666666667;
        content.inverse_inertia = 0.0005859375;
        content.mass = 1.6;
        content.inverse_mass = 0.625;
        content.density = 0.001;
        content.area = 1600.;
        content.position = Position::new(2., 2.);
        content.bounds = Some(Bounds {
            max: BoundsPart { x: 20.0, y: 20.0 },
            min: BoundsPart { x: -20.0, y: -20.0 },
        });
        content.velocity = Velocity::new(0., 0.);
        let mut body = body_from_content(content);

        // Act
        body.set_vertices(vertices);

        // Assert
        assert_float(body.get_area(), 4.0);
        assert_xy(&body.get_axes().unwrap()[0], 0.0, 1.0);
        assert_xy(&body.get_axes().unwrap()[1], -1.0, 0.0);
        assert_bounds(&body.get_bounds().unwrap(), 1.0, 1.0, 3.0, 3.0);
        assert_float(body.get_density(), 0.001);
        assert_float(body.get_inertia(), 0.010666666666666666);
        assert_float(body.get_inverse_inertia(), 93.75);
        assert_float(body.get_inverse_mass(), 250.0);
        assert_float(body.get_mass(), 0.004);
        assert_position(&body.get_position(), 2.0, 2.0);
        assert_velocity(&body.get_velocity_prop(), 0.0, 0.0);
        let vertices = body.get_vertices();
        let body = Some(&body);
        assert_vertex(&vertices[0], body, 1.0, 1.0, 0, false);
        assert_vertex(&vertices[1], body, 3.0, 1.0, 1, false);
        assert_vertex(&vertices[2], body, 3.0, 3.0, 2, false);
        assert_vertex(&vertices[3], body, 1.0, 3.0, 3, false);
    }

    #[test]
    fn set_mass_should_mutate_value_of_mass_inverse_mass_inertia_inverse_inertia_and_density_to_valid_values(
    ) {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.inertia = 1706.6666666666667;
        content.inverse_inertia = 0.0005859375;
        content.mass = 1.6;
        content.inverse_mass = 0.625;
        content.density = 0.001;
        content.area = 1600.;
        let mut body = body_from_content(content);

        let mass = 42.1;

        // Act
        body.set_mass(mass);

        // Assert
        assert_float(body.get_mass(), 42.1);
        assert_float(body.get_inverse_mass(), 0.023752969121140142);
        assert_float(body.get_inertia(), 44906.666666666664);
        assert_float(body.get_inverse_inertia(), 0.000022268408551068885);
        assert_float(body.get_density(), 0.026312500000000003);
    }

    #[test]
    fn set_inertia_should_mutate_value_of_inertia_and_inverse_inertia_to_valid_values() {
        // Arrange
        let mut body = Body::default_body();
        let inertia = 12.;

        // Act
        body.set_inertia(inertia);

        // Assert
        assert_float(body.get_inertia(), 12.);
        assert_float(body.get_inverse_inertia(), 0.08333333333333333);
    }
}
