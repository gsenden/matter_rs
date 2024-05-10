// MARK: Usings
// region: Usings
use super::{body_option::BodyOption, body_properties::BodyProperties};
use crate::{
    core::{
        collision_filter::CollisionFilter,
        common::{self, OrderedHashMap, ShapeType},
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
// region: Usings

// MARK: consts
const INERTIA_SCALE: f64 = 4.;
const NEXT_COLLIDING_GROUP_ID: i32 = 1;
const NEXT_NON_COLLIDING_GROUP_ID: i32 = -1;
const NEXT_CATEGORY: u16 = 1;
const BASE_DELTA: f64 = 1000. / 60.;
const TIME_CORRECTION: bool = true;

// MARK: Structs
#[derive(Clone)]
pub struct Body {
    content: Rc<RefCell<BodyContent>>,
    parent: Weak<RefCell<BodyContent>>,
}

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
    circle_radius: Option<f64>,
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

// MARK: Default Body
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
            circle_radius: None,
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

// MARK: Content Macro's
// region: Content Macro's
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
// endregion: Content Macro's

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

    // MARK: Getters
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

    pub fn get_angular_speed_prop(&self) -> f64 {
        content!(self).angular_speed
    }

    pub fn get_angular_speed(&self) -> f64 {
        f64::abs(self.get_angular_velocity())
    }

    pub fn get_angular_velocity_prop(&self) -> f64 {
        content!(self).angular_velocity
    }

    pub fn get_angular_velocity(&self) -> f64 {
        let content = content!(self);
        (content.angle - content.angle_prev) * BASE_DELTA / content.delta_time
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

    pub fn get_circle_radius(&self) -> Option<f64> {
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

    // MARK: Setters
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
            content.area = vertices::area(&content.vertices, None);
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

    pub fn set_angular_speed(&mut self, speed: f64) {
        let velocity = common::sign(self.get_angular_velocity()) as f64 * speed;
        self.set_angular_velocity(velocity);
    }

    pub fn set_area(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.area = value;
    }

    pub fn set_axes(&mut self, value: &Vec<Vertex>) {
        let mut content = content_mut!(self);
        content.axes = Some(value.clone());
    }

    pub fn set_bounds(&mut self, value: &Bounds) {
        let mut content = content_mut!(self);
        content.bounds = Some(value.clone());
    }

    pub fn set_chamfer(&mut self, value: &Vec<Vector>) {
        let mut content = content_mut!(self);
        content.chamfer = Some(value.clone());
    }

    pub fn set_circle_radius(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.circle_radius = Some(value);
    }

    pub fn set_collision_filter(&mut self, value: &CollisionFilter) {
        let mut content = content_mut!(self);
        content.collision_filter = value.clone();
    }

    pub fn set_constraint_impulse(&mut self, value: &ConstraintImpulse) {
        let mut content = content_mut!(self);
        content.constraint_impulse = value.clone();
    }

    pub fn set_delta_time(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.delta_time = value;
    }

    pub fn set_density(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.density = value;
    }

    pub fn set_friction_air(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.friction_air = value;
    }

    pub fn set_friction_static(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.friction_static = value;
    }

    pub fn set_friction(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.friction = value;
    }

    pub fn set_is_sensor(&mut self, value: bool) {
        let mut content = content_mut!(self);
        content.is_sensor = value;
    }

    pub fn set_is_sleeping(&mut self, value: bool) {
        let mut content = content_mut!(self);
        content.is_sleeping = value;
    }

    pub fn set_is_static(&mut self, value: bool) {
        let mut content = content_mut!(self);
        content.is_static = value;
    }

    pub fn set_motion(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.motion = value;
    }

    pub fn set_position_impulse(&mut self, value: &Position) {
        let mut content = content_mut!(self);
        content.position_impulse = value.clone();
    }

    pub fn set_render(&mut self, value: &Render) {
        let mut content = content_mut!(self);
        content.render = value.clone();
    }

    pub fn set_resitution(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.resitution = value;
    }

    pub fn set_shape_type(&mut self, value: &ShapeType) {
        let mut content = content_mut!(self);
        content.shape_type = value.clone();
    }

    pub fn set_sleep_threshold(&mut self, value: u32) {
        let mut content = content_mut!(self);
        content.sleep_threshold = value;
    }

    pub fn set_slop(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.slop = value;
    }

    pub fn set_time_scale(&mut self, value: u16) {
        let mut content = content_mut!(self);
        content.time_scale = value;
    }

    pub fn set_torque(&mut self, value: f64) {
        let mut content = content_mut!(self);
        content.torque = value;
    }

    pub fn set_total_contacts(&mut self, value: u32) {
        let mut content = content_mut!(self);
        content.total_contacts = value;
    }

    // endregion: Setters

    // MARK: Actions
    // region: Actions
    pub fn translate(&mut self, tranlation: &impl XY, update_velocity: Option<bool>) {
        let mut position: Position = Position::new(0., 0.);
        {
            let content = content!(self);
            position = Position::new_from(&vector::add(&content.position, tranlation));
        }
        self.set_position(position, update_velocity);
    }

    pub fn rotate(
        &mut self,
        rotation: f64,
        point: Option<&impl XY>,
        update_velocity: Option<bool>,
    ) {
        if point.is_none() {
            self.set_angle(self.get_angle() + rotation, update_velocity);
        } else if let Some(point) = point {
            let cos = f64::cos(rotation);
            let sin = f64::sin(rotation);
            let dx = self.get_position().get_x() - point.get_x();
            let dy = self.get_position().get_y() - point.get_y();
            let x = point.get_x() + (dx * cos - dy * sin);
            let y = point.get_y() + (dx * sin + dy * cos);
            let position = Position::new(x, y);

            self.set_position(position, update_velocity);
            self.set_angle(self.get_angle() + rotation, update_velocity);
        }
    }

    pub fn scale(&mut self, scale_x: f64, scale_y: f64, point: Option<&impl XY>) {
        let mut total_area = 0.;
        let mut total_inertia = 0.;

        let point = if let Some(p) = point {
            Position::new_from(p)
        } else {
            self.get_position()
        };

        let parent_id = self.get_id();
        let parent_velocity = self.get_velocity_prop();
        for part in self.get_parts().iter_mut() {
            {
                let mut part_content = content_mut!(part);
                vertices::scale(&mut part_content.vertices, scale_x, scale_y, Some(&point));
                part_content.axes = Some(axes::from_vertices(&part_content.vertices));
                part_content.area = vertices::area(&part_content.vertices, None);
            }
            part.set_mass(self.get_density() * part.get_area());
            {
                let mut part_content = content_mut!(part);
                let position = Position::new(
                    part_content.position.get_x() * -1.,
                    part_content.position.get_y() * -1.,
                );
                vertices::translate(&mut part_content.vertices, &position, None);
            }
            let inertia = INERTIA_SCALE * vertices::innertia(&part.get_vertices(), part.get_mass());
            part.set_inertia(inertia);
            {
                let mut part_content = content_mut!(part);
                let position = part_content.position.clone();
                vertices::translate(&mut part_content.vertices, &position, None);
            }
            if part.get_id() != parent_id {
                total_area += part.get_area();
                total_inertia += part.get_inertia();
            }
            {
                let mut part_content = content_mut!(part);
                let x = point.get_x() + (part_content.position.get_x() - point.get_x()) * scale_x;
                let y = point.get_y() + (part_content.position.get_y() - point.get_y()) * scale_y;
                part_content.position.set_x(x);
                part_content.position.set_y(y);

                let vertices = part_content.vertices.clone();
                if let Some(bounds) = &mut part_content.bounds {
                    // what happens if bounds == None -> no idea. Need futher testing. ... famous last words
                    bounds::update(bounds, &vertices, &Some(&parent_velocity));
                }
            }
        }

        if self.get_parts().len() > 1 {
            self.set_area(total_area);

            if !self.get_is_static() {
                self.set_mass(self.get_density() * total_area);
                self.set_inertia(total_inertia);
            }
        }

        if self.get_circle_radius().is_some() {
            if scale_x == scale_y {
                let circle_radius = self.get_circle_radius().unwrap_or(0.);

                self.set_circle_radius(circle_radius * scale_x);
            } else {
                let mut content = content_mut!(self);
                content.circle_radius = None;
            }
        }
    }

    // endregion: Actions
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

    // region: Helpers
    fn body_from_content(content: BodyContent) -> Body {
        Body {
            content: Rc::new(RefCell::new(content)),
            parent: Weak::new(),
        }
    }

    fn test_body() -> Body {
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.angle = 42.;
        content.angle_prev = 41.;
        content.axes = Some(vec![
            Vertex::new(None, 1., 1., 0, false),
            Vertex::new(None, -1., -1., 1, false),
        ]);
        content.position = Position::new(2., 2.);
        content.position_prev = Some(Position::new(1., 1.));
        content.bounds = Some(test_bounds());
        content.velocity = Velocity::new(42., 42.);
        content.density = 1.1;
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
                part_content.density = 1.1 + increase;
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
        body_from_content(content)
    }
    // endregion: Helpers

    #[test]
    fn scale_should_be_able_to_scale_a_circular_body() {
        // Arrange
        let mut body = test_body();
        body.set_circle_radius(37.37);
        let scale_x = 37.;
        let scale_y = 37.;
        let point = Position::new(93., 94.);
        let point: Option<&Position> = Some(&point);

        // Act
        body.scale(scale_x, scale_y, point);

        // Assert
        assert_float(body.get_circle_radius().unwrap(), 1382.6899999999998);
    }

    #[test]
    fn scale_should_be_able_to_scale_a_body_using_a_point() {
        // Arrange
        let mut body = test_body();
        let scale_x = 37.;
        let scale_y = 38.;
        let point = Position::new(93., 94.);
        let point: Option<&Position> = Some(&point);

        // Act
        body.scale(scale_x, scale_y, point);

        // Assert
        let part = body.get_parts()[0].clone();
        assert_float(part.get_angle(), 42.);
        assert_float(part.get_angle_prev(), 41.);
        assert_float(part.get_area(), 11248.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 0., 1.);
        assert_xy(&axes[1], -1., 0.);
        assert_bounds(&part.get_bounds().unwrap(), -3311., -3440., -3195., -3322.);
        assert_float(part.get_density(), 1.1);
        assert_float(part.get_inertia(), 533832310912.02997);
        assert_float(part.get_inverse_inertia(), 1.8732474216323515E-12);
        assert_float(part.get_inverse_mass(), 0.00008082244924350186);
        assert_float(part.get_mass(), 12372.800000000001);
        assert_xy(&part.get_position(), -3274., -3402.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -3311., -3440.);
        assert_xy(&vertices[1], -3237., -3440.);
        assert_xy(&vertices[2], -3237., -3364.);
        assert_xy(&vertices[3], -3311., -3364.);

        let part = body.get_parts()[1].clone();
        assert_float(part.get_angle(), 43.);
        assert_float(part.get_angle_prev(), 42.);
        assert_float(part.get_area(), 5624.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 0., 1.);
        assert_xy(&axes[1], -1., 0.);
        assert_bounds(&part.get_bounds().unwrap(), -3274., -3402., -3158., -3284.);
        assert_float(part.get_density(), 1.1);
        assert_float(part.get_inertia(), 269882416676.12234);
        assert_float(part.get_inverse_inertia(), 3.7053173464059704e-12);
        assert_float(part.get_inverse_mass(), 0.00016164489848700372);
        assert_float(part.get_mass(), 6186.400000000001);
        assert_xy(&part.get_position(), -3311., -3440.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -3274., -3402.);
        assert_xy(&vertices[1], -3200., -3402.);
        assert_xy(&vertices[2], -3200., -3326.);
        assert_xy(&vertices[3], -3274., -3326.);

        let part = body.get_parts()[2].clone();
        assert_float(part.get_angle(), 44.);
        assert_float(part.get_angle_prev(), 43.);
        assert_float(part.get_area(), 5624.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 0., 1.);
        assert_xy(&axes[1], -1., 0.);
        assert_bounds(&part.get_bounds().unwrap(), -3237., -3364., -3121., -3246.);
        assert_float(part.get_density(), 1.1);
        assert_float(part.get_inertia(), 263949894235.90762);
        assert_float(part.get_inverse_inertia(), 3.788597843143066e-12);
        assert_float(part.get_inverse_mass(), 0.00016164489848700372);
        assert_float(part.get_mass(), 6186.400000000001);
        assert_xy(&part.get_position(), -3274., -3402.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -3237., -3364.);
        assert_xy(&vertices[1], -3163., -3364.);
        assert_xy(&vertices[2], -3163., -3288.);
        assert_xy(&vertices[3], -3237., -3288.);
    }

    #[test]
    fn scale_should_be_able_to_scale_a_body_not_using_a_point() {
        // Arrange
        let mut body = test_body();
        let scale_x = 37.;
        let scale_y = 38.;
        let point: Option<&Position> = None;

        // Act
        body.scale(scale_x, scale_y, point);

        // Assert
        let part = body.get_parts()[0].clone();
        assert_float(part.get_angle(), 42.);
        assert_float(part.get_angle_prev(), 41.);
        assert_float(part.get_area(), 11248.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 0., 1.);
        assert_xy(&axes[1], -1., 0.);
        assert_bounds(&part.get_bounds().unwrap(), -35., -36., 81., 82.);
        assert_float(part.get_density(), 1.1);
        assert_float(part.get_inertia(), 291919290.36388415);
        assert_float(part.get_inverse_inertia(), 3.4256043811064247e-9);
        assert_float(part.get_inverse_mass(), 0.00008082244924350186);
        assert_float(part.get_mass(), 12372.800000000001);
        assert_xy(&part.get_position(), 2., 2.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -35., -36.);
        assert_xy(&vertices[1], 39., -36.);
        assert_xy(&vertices[2], 39., 40.);
        assert_xy(&vertices[3], -35., 40.);

        let part = body.get_parts()[1].clone();
        assert_float(part.get_angle(), 43.);
        assert_float(part.get_angle_prev(), 42.);
        assert_float(part.get_area(), 5624.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 0., 1.);
        assert_xy(&axes[1], -1., 0.);
        assert_bounds(&part.get_bounds().unwrap(), 2., 2., 118., 120.);
        assert_float(part.get_density(), 1.1);
        assert_float(part.get_inertia(), 94692734.09721743);
        assert_float(part.get_inverse_inertia(), 1.056047234810369e-8);
        assert_float(part.get_inverse_mass(), 0.00016164489848700372);
        assert_float(part.get_mass(), 6186.400000000001);
        assert_xy(&part.get_position(), -35., -36.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 2., 2.);
        assert_xy(&vertices[1], 76., 2.);
        assert_xy(&vertices[2], 76., 78.);
        assert_xy(&vertices[3], 2., 78.);

        let part = body.get_parts()[2].clone();
        assert_float(part.get_angle(), 44.);
        assert_float(part.get_angle_prev(), 43.);
        assert_float(part.get_area(), 5624.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 0., 1.);
        assert_xy(&axes[1], -1., 0.);
        assert_bounds(&part.get_bounds().unwrap(), 39., 40., 155., 158.);
        assert_float(part.get_density(), 1.1);
        assert_float(part.get_inertia(), 197226556.2666667);
        assert_float(part.get_inverse_inertia(), 5.070311112910762e-9);
        assert_float(part.get_inverse_mass(), 0.00016164489848700372);
        assert_float(part.get_mass(), 6186.400000000001);
        assert_xy(&part.get_position(), 2., 2.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 39., 40.);
        assert_xy(&vertices[1], 113., 40.);
        assert_xy(&vertices[2], 113., 116.);
        assert_xy(&vertices[3], 39., 116.);
    }

    #[test]
    fn rotate_should_be_able_to_rotate_a_body_with_a_point() {
        // Arrange
        let mut body = test_body();

        let rotation = 37.;

        let point = Position::new(93., 94.);
        let point = Some(&point);
        let update_velocity: Option<bool> = None;

        // Act
        body.rotate(rotation, point, update_velocity);

        let part = body.get_parts()[0].clone();
        assert_float(part.get_angle(), 79.);
        assert_float(part.get_angle_prev(), 78.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 1.408952185302343, 0.1218759185883439);
        assert_xy(&axes[1], -1.408952185302343, -0.1218759185883439);
        assert_bounds(
            &part.get_bounds().unwrap(),
            -37.26713918117254,
            80.73492517121302,
            7.550765189432141,
            125.5528295418177,
        );
        assert_xy(&part.get_position(), -35.8581869958702, 82.14387735651536);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -37.26713918117254, 82.02200143792702);
        assert_xy(&vertices[1], -35.736311077281854, 80.73492517121302);
        assert_xy(&vertices[2], -34.44923481056786, 82.2657532751037);
        assert_xy(&vertices[3], -35.98006291445854, 83.5528295418177);

        let part = body.get_parts()[1].clone();
        assert_float(part.get_angle(), 80.);
        assert_float(part.get_angle_prev(), 42.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 2.817904370604686, 0.2437518371766878);
        assert_xy(&axes[1], -2.817904370604686, -0.2437518371766878);
        assert_bounds(
            &part.get_bounds().unwrap(),
            -35.8581869958702,
            80.85680108980137,
            8.959717374734488,
            125.67470546040605,
        );
        assert_xy(&part.get_position(), -37.26713918117254, 82.02200143792702);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -35.8581869958702, 82.14387735651536);
        assert_xy(&vertices[1], -34.327358891979515, 80.85680108980137);
        assert_xy(&vertices[2], -33.04028262526551, 82.38762919369205);
        assert_xy(&vertices[3], -34.5711107291562, 83.67470546040605);

        let part = body.get_parts()[2].clone();
        assert_float(part.get_angle(), 81.);
        assert_float(part.get_angle_prev(), 43.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 4.226856555907029, 0.3656277557650318);
        assert_xy(&axes[1], -4.226856555907029, -0.3656277557650318);
        let bounds = part.get_bounds().unwrap();
        assert_bounds(
            &bounds,
            -34.44923481056786,
            80.97867700838971,
            10.368669560036828,
            125.79658137899439,
        );
        assert_xy(&part.get_position(), -35.8581869958702, 82.14387735651536);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], -34.44923481056786, 82.2657532751037);
        assert_xy(&vertices[1], -32.91840670667717, 80.97867700838971);
        assert_xy(&vertices[2], -31.631330439963172, 82.5095051122804);
        assert_xy(&vertices[3], -33.162158543853856, 83.79658137899439);
    }

    #[test]
    fn rotate_should_be_able_to_rotate_a_body_without_a_point() {
        // Arrange
        let mut body = test_body();
        let rotation = 37.;
        let point: Option<&Position> = None;
        let update_velocity: Option<bool> = None;

        // Act
        body.rotate(rotation, point, update_velocity);

        let part = body.get_parts()[0].clone();
        assert_float(part.get_angle(), 79.);
        assert_float(part.get_angle_prev(), 78.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 1.408952185302343, 0.1218759185883439);
        assert_xy(&axes[1], -1.408952185302343, -0.1218759185883439);
        assert_bounds(
            &part.get_bounds().unwrap(),
            0.591047814697657,
            0.591047814697657,
            45.40895218530234,
            45.40895218530234,
        );
        assert_xy(&part.get_position(), 2., 2.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 0.591047814697657, 1.8781240814116562);
        assert_xy(&vertices[1], 2.121875918588344, 0.591047814697657);
        assert_xy(&vertices[2], 3.408952185302343, 2.121875918588344);
        assert_xy(&vertices[3], 1.8781240814116562, 3.408952185302343);

        let part = body.get_parts()[1].clone();
        assert_float(part.get_angle(), 80.);
        assert_float(part.get_angle_prev(), 42.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 2.817904370604686, 0.2437518371766878);
        assert_xy(&axes[1], -2.817904370604686, -0.2437518371766878);
        assert_bounds(
            &part.get_bounds().unwrap(),
            2.,
            0.712923733286001,
            46.81790437060469,
            45.530828103890684,
        );
        assert_xy(&part.get_position(), 0.591047814697657, 1.8781240814116562);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 2., 2.);
        assert_xy(&vertices[1], 3.530828103890687, 0.712923733286001);
        assert_xy(&vertices[2], 4.817904370604686, 2.2437518371766876);
        assert_xy(&vertices[3], 3.287076266713999, 3.530828103890687);

        let part = body.get_parts()[2].clone();
        assert_float(part.get_angle(), 81.);
        assert_float(part.get_angle_prev(), 43.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], 4.226856555907029, 0.3656277557650318);
        assert_xy(&axes[1], -4.226856555907029, -0.3656277557650318);
        let bounds = part.get_bounds().unwrap();
        assert_bounds(
            &bounds,
            3.408952185302343,
            0.834799651874345,
            48.22685655590703,
            45.65270402247903,
        );
        assert_xy(&part.get_position(), 2., 2.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 3.408952185302343, 2.121875918588344);
        assert_xy(&vertices[1], 4.939780289193029, 0.834799651874345);
        assert_xy(&vertices[2], 6.226856555907029, 2.365627755765032);
        assert_xy(&vertices[3], 4.696028452016342, 3.6527040224790306);
    }

    #[test]
    fn translate_should_be_able_to_translate_a_body() {
        // Arrange
        let mut body = test_body();

        let translation = Position::new(37., 38.);
        let update_velocity = None;

        // Act
        body.translate(&translation, update_velocity);

        // Assert
        assert_xy(&body.get_position(), 39., 40.);
        assert_xy(&body.get_position_prev().unwrap(), 38., 39.);
        assert_xy(&body.get_velocity_prop(), 42., 42.);

        let parts = &body.get_parts();
        let part = parts[0].clone();
        assert_bounds(&part.get_bounds().unwrap(), 38., 39., 82., 83.);
        assert_xy(&part.get_position(), 39., 40.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 38., 39.);
        assert_xy(&vertices[1], 40., 39.);
        assert_xy(&vertices[2], 40., 41.);
        assert_xy(&vertices[3], 38., 41.);

        let part = parts[1].clone();
        assert_bounds(&part.get_bounds().unwrap(), 39., 40., 83., 84.);
        assert_xy(&part.get_position(), 38., 39.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 39., 40.);
        assert_xy(&vertices[1], 41., 40.);
        assert_xy(&vertices[2], 41., 42.);
        assert_xy(&vertices[3], 39., 42.);

        let part = parts[2].clone();
        assert_bounds(&part.get_bounds().unwrap(), 40., 41., 84., 85.);
        assert_xy(&part.get_position(), 39., 40.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 40., 41.);
        assert_xy(&vertices[1], 42., 41.);
        assert_xy(&vertices[2], 42., 43.);
        assert_xy(&vertices[3], 40., 43.);
    }

    #[test]
    fn set_angular_speed_be_able_to_set_the_angular_speed_on_a_body() {
        // Arrange
        let mut content = BodyContent::default_contant();
        content.angle = 42.;
        content.angle_prev = 41.;
        let mut body = body_from_content(content);
        let speed = 37.;

        // Act
        body.set_angular_speed(speed);

        // Assert
        assert_float(body.get_angle(), 42.);
        assert_float(body.get_angle_prev(), 5.);
        assert_float(body.get_angular_velocity_prop(), 37.);
        assert_float(body.get_angular_velocity(), 37.);
        assert_float(body.get_angular_speed_prop(), 37.);
        assert_float(body.get_angular_speed(), 37.);
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
        assert_float(body.get_angular_velocity(), 37.);
        assert_float(body.get_angular_speed_prop(), 37.);
        assert_float(body.get_angular_speed(), 37.);
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
    fn set_velocity_should_be_able_to_set_the_velocity_on_a_body() {
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
