// MARK: Usings
// region: Usings
use super::{
    body_option::BodyOption, body_orgiginal::BodyOriginal, body_properties::BodyProperties,
};
use crate::{
    core::{
        collision_filter::CollisionFilter,
        common::{self, ShapeType, BASE_DELTA},
        constraint_impulse::ConstraintImpulse,
        force::Force,
        position::{self, Position},
        render::Render,
        sprite::Sprite,
        velocity::{self, Velocity},
        xy::{XYNew, XY},
    },
    geometry::{
        axes::{self, Axes},
        bounds::{self, Bounds},
        vector::{self, Vector},
        vertices::{self, Vertices},
    },
};
use core::time;
use std::sync::{Arc, Mutex, Weak};
use uuid::Uuid;
// region: Usings

// MARK: consts
const INERTIA_SCALE: f64 = 4.;
const NEXT_COLLIDING_GROUP_ID: i32 = 1;
const NEXT_NON_COLLIDING_GROUP_ID: i32 = -1;
const NEXT_CATEGORY: u16 = 1;
const TIME_CORRECTION: bool = true;

// MARK: Structs
#[derive(Clone)]
pub struct Body {
    content: Arc<Mutex<BodyContent>>,
    parent: Weak<Mutex<BodyContent>>,
}

#[derive(Clone)]
pub struct BodyContent {
    angle_prev: f64,
    angle: f64,
    angular_speed: f64,
    angular_velocity: f64,
    area: f64,
    axes: Option<Axes>,
    bounds: Option<Bounds>,
    chamfer: Option<Vec<Vector>>,
    circle_radius: Option<f64>,
    collision_filter: CollisionFilter,
    constraint_impulse: ConstraintImpulse,
    delta_time: Option<f64>,
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
    vertices: Vertices,
    _original: Option<BodyOriginal>,
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
            delta_time: Some(common::BASE_DELTA),
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
            vertices: Vertices::from_path("L 0 0 L 40 0 L 40 40 L 0 40", None).unwrap(),
            _original: None,
        }
    }
}

// MARK: Content Macro's
// region: Content Macro's
macro_rules! content {
    ($a:expr) => {
        //$a.content.as_ref().borrow()
        $a.content.lock().unwrap()
    };
}

macro_rules! content_mut {
    ($a:expr) => {
        $a.content.lock().unwrap()
        //$a.content.as_ref().borrow_mut()
    };
}
// endregion: Content Macro's

impl Body {
    pub fn default_body() -> Self {
        let content = BodyContent::default_contant();
        let mut body = Body {
            content: Arc::new(Mutex::new(content)),
            parent: Weak::new(),
        };
        content_mut!(body).vertices.set_body(&body);
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

    pub fn get_axes(&self) -> Option<Axes> {
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
        let time_scale = common::BASE_DELTA / content.delta_time.unwrap_or(common::BASE_DELTA);

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

    pub fn get_vertices(&self) -> Vertices {
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
        self.get_velocity().magnitude()
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
        (content.angle - content.angle_prev) * common::BASE_DELTA
            / content.delta_time.unwrap_or(common::BASE_DELTA)
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

    pub fn get_delta_time(&self) -> Option<f64> {
        content!(self).delta_time
    }

    fn get_parts_prop(&self) -> Option<Vec<Body>> {
        content!(self).parts.clone()
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

    fn get_original(&self) -> Option<BodyOriginal> {
        content!(self)._original
    }

    pub fn get_moment(&self) -> f64 {
        let content = content!(self);
        content.inertia / (content.mass / 6.)
    }
    // endregion: Getters

    // MARK: Setters
    // region: Setters
    pub fn set_parent(&mut self, parent: &Body) {
        self.parent = Arc::downgrade(&parent.content);
    }

    fn set_inertia_prop(&mut self, value: f64) {
        content_mut!(self).inertia = value;
    }

    pub fn set_inertia(&mut self, value: f64) {
        self.set_inertia_prop(value);
        self.set_inverse_inertia(1. / self.get_inertia());
    }

    fn set_mass_prop(&mut self, value: f64) {
        content_mut!(self).mass = value;
    }

    pub fn set_mass(&mut self, value: f64) {
        let moment = self.get_moment();
        self.set_inertia(moment * (value / 6.));
        self.set_mass_prop(value);
        self.set_inverse_mass(1. / value);
        self.set_density_prop(value / self.get_area());
    }

    fn set_vertices_prop(&mut self, value: &Vertices) {
        content_mut!(self).vertices = value.clone();
    }

    pub fn set_vertices(&mut self, vertices: &Vertices) {
        let mut vertices = vertices.clone();
        vertices.set_body(&self);
        self.set_vertices_prop(&vertices);
        self.set_axes(&Axes::from_vertices(&vertices));
        self.set_area(vertices.area(None));
        self.set_mass(self.get_density() * self.get_area());
        let centre = vertices.centre();
        let mut vertices = self.get_vertices();
        vertices.translate(&centre, Some(-1.));
        self.set_vertices_prop(&vertices);
        self.set_inertia(INERTIA_SCALE * self.get_vertices().innertia(self.get_mass()));
        let mut vertices = self.get_vertices();
        vertices.translate(&self.get_position(), None);
        self.set_vertices_prop(&vertices);

        if let Some(bounds) = &mut self.get_bounds() {
            bounds.update(&self.get_vertices(), Some(&self.get_velocity_prop()));
            self.set_bounds(&bounds);
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
            let mut position = part.get_position();
            position.mult(mass);
            let mut centre = properties.get_centre();
            centre.add_xy(&position);
            properties.set_centre(&centre);

            index += 1;
        }

        let mut centre = properties.get_centre();
        centre.div(properties.get_mass());
        properties.set_centre(&centre);

        properties
    }

    fn set_position_prop(&mut self, value: &impl XY) {
        content_mut!(self).position.set_xy(value);
    }

    pub fn set_position(&mut self, position: Position, update_velocity: Option<bool>) {
        let update_velocity = update_velocity.unwrap_or(false);
        let mut delta = Velocity::new_from(&position);
        delta.sub(&self.get_position());

        if update_velocity {
            self.set_position_prev(&self.get_position());
            self.set_velocity_prop(&delta);
            let mut speed = delta.clone();
            self.set_speed_prop(speed.magnitude());
        } else {
            if let Some(position_prev) = &mut self.get_position_prev() {
                position_prev.add_xy(&delta);
                self.set_position_prev(&position_prev);
            } else {
                self.set_position_prev(&Position::new_from(&delta));
            }
        }

        for part in self.get_parts().iter_mut() {
            let mut position = part.get_position();
            position.add_xy(&delta);
            part.set_position_prop(&position);
            let mut vertices = part.get_vertices();
            vertices.translate(&delta, None);
            part.set_vertices_prop(&vertices);
            if let Some(bounds) = &mut part.get_bounds() {
                bounds.update(&part.get_vertices(), Some(&self.get_velocity_prop()));
                part.set_bounds(&bounds);
            }
        }
    }

    fn set_parts_prop(&mut self, value: &Vec<Body>) {
        content_mut!(self).parts = Some(value.clone());
    }

    pub fn set_parts(&mut self, parts: Vec<Body>, auto_hull: Option<bool>) {
        let mut parts = parts;
        for part in &mut parts {
            part.set_parent(self);
        }

        let auto_hull = auto_hull.unwrap_or(true);

        if parts.len() > 0 {
            self.set_parts_prop(&parts);
        } else {
            return;
        }

        if auto_hull {
            let mut vertices = Vertices::new(Vec::new(), Some(self.clone()));
            if let Some(parts) = self.get_parts_prop() {
                for part in parts.iter() {
                    vertices.append(&part.get_vertices());
                }
            }
            vertices.clockwise_sort();
            vertices.hull();
            let hull = vertices.clone();
            let hull_centre = hull.centre();

            self.set_vertices(&hull);
            let mut vertices = self.get_vertices();
            vertices.translate(&hull_centre, None);
            self.set_vertices_prop(&vertices);
        }

        let total = self.total_properties();

        self.set_area(total.get_area());
        self.set_parent(&self.clone());
        self.set_position_prop(&total.get_centre());
        self.set_position_prev(&total.get_centre());
        self.set_mass(total.get_mass());
        self.set_inertia(total.get_inertia());
        self.set_position(total.get_centre(), None);
    }

    pub fn set_centre(&mut self, centre: &Position, relative: Option<bool>) {
        let relative = relative.unwrap_or(false);

        if !relative {
            if let Some(position_prev) = &mut self.get_position_prev() {
                position_prev
                    .set_x(centre.get_x() - (self.get_position().get_x() - position_prev.get_x()));
                position_prev
                    .set_y(centre.get_y() - (self.get_position().get_y() - position_prev.get_y()));
                self.set_position_prev(&position_prev);
            } else {
                let position_prev = Position::new(
                    centre.get_x() - self.get_position().get_x(),
                    centre.get_y() - self.get_position().get_y(),
                );
                self.set_position_prev(&position_prev);
            }
            self.set_position_prop(centre);
        } else {
            if let Some(position_prev) = &mut self.get_position_prev() {
                position_prev.add_xy(centre);
                self.set_position_prev(&position_prev);
            } else {
                self.set_position_prev(&centre);
            }
            let mut position = self.get_position();
            position.add_xy(centre);
            self.set_position_prop(&position);
        }
    }

    fn set_angle_prop(&mut self, angle: f64) {
        content_mut!(self).angle = angle;
    }

    pub fn set_angle(&mut self, angle: f64, update_velocity: Option<bool>) {
        let update_velocity = update_velocity.unwrap_or(false);

        let delta = angle - self.get_angle();
        if update_velocity {
            self.set_angle_prev(self.get_angle());
            self.set_angular_velocity_prop(delta);
            self.set_angular_speed_prop(f64::abs(delta));
        } else {
            self.set_angle_prev(self.get_angle_prev() + delta);
        }

        for part in &mut self.get_parts() {
            part.set_angle_prop(part.get_angle() + delta);
            let mut vertices = part.get_vertices();
            vertices.rotate(delta, &self.get_position());
            part.set_vertices_prop(&vertices);

            if let Some(axes) = &mut part.get_axes() {
                axes.rotate(delta);
                part.set_axes(&axes);
            }

            if let Some(bounds) = &mut part.get_bounds() {
                bounds.update(&part.get_vertices(), Some(&self.get_velocity_prop()));
                part.set_bounds(&bounds);
            }
            if !self.is_part_parent(part) {
                let mut position = part.get_position();
                position.rotate_about(delta, &self.get_position());
                part.set_position_prop(&position);
            }
        }
    }

    fn set_velocity_prop(&mut self, value: &Velocity) {
        content_mut!(self).velocity = value.clone();
    }

    pub fn set_velocity(&mut self, velocity: &Velocity) {
        let time_scale = self.get_delta_time().unwrap_or(common::BASE_DELTA) / common::BASE_DELTA;

        let position_prev = Position::new(
            self.get_position().get_x() - velocity.get_x() * time_scale,
            self.get_position().get_y() - velocity.get_y() * time_scale,
        );

        let velocity = Velocity::new(
            (self.get_position().get_x() - position_prev.get_x()) / time_scale,
            (self.get_position().get_y() - position_prev.get_y()) / time_scale,
        );

        self.set_position_prev(&position_prev);
        self.set_velocity_prop(&velocity);
        self.set_speed_prop(self.get_velocity_prop().magnitude());
    }

    fn set_speed_prop(&mut self, value: f64) {
        content_mut!(self).speed = value;
    }

    pub fn set_speed(&mut self, speed: f64) {
        let mut velocity = self.get_velocity();
        velocity.normalise();
        velocity.mult(speed);
        self.set_velocity(&velocity);
    }

    pub fn set_angular_velocity(&mut self, velocity: f64) {
        let time_scale = self.get_delta_time().unwrap_or(common::BASE_DELTA) / common::BASE_DELTA;
        self.set_angle_prev(self.get_angle() - velocity * time_scale);
        self.set_angular_velocity_prop((self.get_angle() - self.get_angle_prev()) / time_scale);
        self.set_angular_speed_prop(f64::abs(self.get_angular_velocity_prop()));
    }

    fn set_angular_speed_prop(&mut self, value: f64) {
        content_mut!(self).angular_speed = value;
    }

    pub fn set_angular_speed(&mut self, speed: f64) {
        let velocity = common::sign(self.get_angular_velocity()) as f64 * speed;
        self.set_angular_velocity(velocity);
    }

    pub fn set_area(&mut self, value: f64) {
        content_mut!(self).area = value;
    }

    pub fn set_axes(&mut self, value: &Axes) {
        content_mut!(self).axes = Some(value.clone());
    }

    pub fn set_bounds(&mut self, value: &Bounds) {
        content_mut!(self).bounds = Some(*value);
    }

    pub fn set_chamfer(&mut self, value: &Vec<Vector>) {
        content_mut!(self).chamfer = Some(value.clone());
    }

    pub fn set_circle_radius(&mut self, value: f64) {
        content_mut!(self).circle_radius = Some(value);
    }

    pub fn set_collision_filter(&mut self, value: &CollisionFilter) {
        content_mut!(self).collision_filter = *value;
    }

    pub fn set_constraint_impulse(&mut self, value: &ConstraintImpulse) {
        content_mut!(self).constraint_impulse = *value;
    }

    pub fn set_delta_time(&mut self, value: f64) {
        content_mut!(self).delta_time = Some(value);
    }

    pub fn set_density_prop(&mut self, value: f64) {
        content_mut!(self).density = value;
    }

    pub fn set_density(&mut self, value: f64) {
        self.set_mass(value * self.get_area());
        self.set_density_prop(value);
    }

    pub fn set_friction_air(&mut self, value: f64) {
        content_mut!(self).friction_air = value;
    }

    pub fn set_friction_static(&mut self, value: f64) {
        content_mut!(self).friction_static = value;
    }

    pub fn set_friction(&mut self, value: f64) {
        content_mut!(self).friction = value;
    }

    pub fn set_is_sensor(&mut self, value: bool) {
        content_mut!(self).is_sensor = value;
    }

    pub fn set_is_sleeping(&mut self, value: bool) {
        content_mut!(self).is_sleeping = value;
    }

    pub fn set_motion(&mut self, value: f64) {
        content_mut!(self).motion = value;
    }

    pub fn set_position_impulse(&mut self, value: &Position) {
        content_mut!(self).position_impulse = *value;
    }

    pub fn set_render(&mut self, value: &Render) {
        content_mut!(self).render = *value;
    }

    pub fn set_resitution(&mut self, value: f64) {
        content_mut!(self).resitution = value;
    }

    pub fn set_shape_type(&mut self, value: &ShapeType) {
        content_mut!(self).shape_type = *value;
    }

    pub fn set_sleep_threshold(&mut self, value: u32) {
        content_mut!(self).sleep_threshold = value;
    }

    pub fn set_slop(&mut self, value: f64) {
        content_mut!(self).slop = value;
    }

    pub fn set_time_scale(&mut self, value: u16) {
        content_mut!(self).time_scale = value;
    }

    pub fn set_torque(&mut self, value: f64) {
        content_mut!(self).torque = value;
    }

    pub fn set_total_contacts(&mut self, value: u32) {
        content_mut!(self).total_contacts = value;
    }

    fn set_inverse_mass(&mut self, value: f64) {
        content_mut!(self).inverse_mass = value;
    }

    fn set_inverse_inertia(&mut self, value: f64) {
        content_mut!(self).inverse_inertia = value;
    }

    fn set_original(&mut self, value: Option<BodyOriginal>) {
        content_mut!(self)._original = value;
    }

    fn set_position_prev(&mut self, value: &Position) {
        content_mut!(self).position_prev = Some(*value);
    }

    fn set_angle_prev(&mut self, value: f64) {
        content_mut!(self).angle_prev = value;
    }

    fn set_angular_velocity_prop(&mut self, value: f64) {
        content_mut!(self).angular_velocity = value;
    }

    fn set_from_body_original(&mut self, value: &BodyOriginal) {
        self.set_density_prop(value.get_density());
        self.set_friction(value.get_friction());
        self.set_inertia_prop(value.get_inertia());
        self.set_inverse_inertia(value.get_inverse_inertia());
        self.set_inverse_mass(value.get_inverse_mass());
        self.set_mass_prop(value.get_mass());
        self.set_resitution(value.get_resitution());
    }

    fn set_is_static(&mut self, value: bool) {
        content_mut!(self).is_static = value;
    }

    pub fn set_static(&mut self, is_static: bool) {
        for part in &mut self.get_parts() {
            if is_static {
                part.set_original(Some(BodyOriginal::from(&part)));
                part.set_resitution(0.);
                part.set_friction(1.);
                part.set_mass_prop(f64::INFINITY);
                part.set_inertia_prop(f64::INFINITY);
                part.set_density_prop(f64::INFINITY);
                part.set_inverse_mass(0.);
                part.set_inverse_inertia(0.);
                part.set_position_prev(&part.get_position());
                part.set_angle_prev(part.get_angle());
                part.set_angular_velocity_prop(0.);
                part.set_speed_prop(0.);
                part.set_angular_speed_prop(0.);
                part.set_motion(0.);
            } else if let Some(original) = part.get_original() {
                part.set_from_body_original(&original);
                part.set_original(None);
            }
            part.set_is_static(is_static);
        }
    }
    // endregion: Setters

    // MARK: Actions
    // region: Actions
    pub fn is_part_parent(&self, part: &Body) -> bool {
        self.get_id() == part.get_id()
    }

    pub fn translate(&mut self, translation: &impl XY, update_velocity: Option<bool>) {
        let mut position = self.get_position();
        position.add_xy(translation);
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

        let point = if let Some(point) = point {
            Position::new_from(point)
        } else {
            self.get_position()
        };

        for part in self.get_parts().iter_mut() {
            let mut vertices = part.get_vertices();
            vertices.scale(scale_x, scale_y, Some(&point));
            part.set_vertices_prop(&vertices);

            part.set_axes(&Axes::from_vertices(&part.get_vertices()));
            part.set_area(part.get_vertices().area(None));
            part.set_mass(self.get_density() * part.get_area());
            let mut position = part.get_position();
            position.neg();
            let mut vertices = part.get_vertices();
            vertices.translate(&position, None);
            part.set_vertices_prop(&vertices);
            part.set_inertia(INERTIA_SCALE * part.get_vertices().innertia(part.get_mass()));
            let mut vertices = part.get_vertices();
            vertices.translate(&part.get_position(), None);
            part.set_vertices_prop(&vertices);

            if !self.is_part_parent(part) {
                total_area += part.get_area();
                total_inertia += part.get_inertia();
            }

            let x = point.get_x() + (part.get_position().get_x() - point.get_x()) * scale_x;
            let y = point.get_y() + (part.get_position().get_y() - point.get_y()) * scale_y;
            let position = Position::new(x, y);
            part.set_position_prop(&position);

            if let Some(bounds) = &mut part.get_bounds() {
                // what happens if bounds == None -> no idea. Need futher testing. ... famous last words
                bounds.update(&part.get_vertices(), Some(&self.get_velocity_prop()));
                part.set_bounds(&bounds);
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
                content_mut!(self).circle_radius = None;
            }
        }
    }

    pub fn update(&mut self, delta_time: Option<f64>) {
        let delta_time = delta_time.unwrap_or(common::BASE_DELTA) * self.get_time_scale() as f64;
        let delta_time_squared = delta_time * delta_time;
        let correction = if TIME_CORRECTION {
            delta_time / (self.get_delta_time().unwrap_or(common::BASE_DELTA))
        } else {
            1.
        };

        // from the previous step
        let friction_air = 1. - self.get_friction_air() * (delta_time / common::BASE_DELTA);
        let velocity_prev_x = (self.get_position().get_x()
            - self
                .get_position_prev()
                .unwrap_or(Position::new(0., 0.))
                .get_x())
            * correction;
        let velocity_prev_y = (self.get_position().get_y()
            - self
                .get_position_prev()
                .unwrap_or(Position::new(0., 0.))
                .get_y())
            * correction;

        // update velocity with Verlet integration
        let velocity_x = (velocity_prev_x * friction_air)
            + (self.get_force().get_x() / self.get_mass()) * delta_time_squared;
        let velocity_y = (velocity_prev_y * friction_air)
            + (self.get_force().get_y() / self.get_mass()) * delta_time_squared;

        self.set_velocity_prop(&Velocity::new(velocity_x, velocity_y));
        self.set_position_prev(&self.get_position());
        let mut position = self.get_position();
        position.add_xy(&self.get_velocity_prop());
        self.set_position_prop(&position);
        self.set_delta_time(delta_time);

        // update angular velocity with Verlet integration
        let angular_velocity =
            ((self.get_angle() - self.get_angle_prev()) * friction_air * correction)
                + (self.get_torque() / self.get_inertia()) * delta_time_squared;
        self.set_angular_velocity_prop(angular_velocity);
        self.set_angle_prev(self.get_angle());
        self.set_angle_prop(self.get_angle() + angular_velocity);

        // transform the body geometry
        for part in &mut self.get_parts() {
            let mut vertices = part.get_vertices();
            vertices.translate(&self.get_velocity_prop(), None);
            part.set_vertices_prop(&vertices);

            if !self.is_part_parent(part) {
                let mut position = part.get_position();
                position.add_xy(&self.get_velocity_prop());
                part.set_position_prop(&position);
            }

            if angular_velocity != 0. {
                let mut vertices = part.get_vertices();
                vertices.rotate(angular_velocity, &self.get_position());
                part.set_vertices_prop(&vertices);

                if let Some(axes) = &mut part.get_axes() {
                    axes.rotate(angular_velocity);
                    part.set_axes(&axes);
                }

                if !self.is_part_parent(part) {
                    let mut position = part.get_position();
                    position.rotate_about(angular_velocity, &self.get_position());
                    part.set_position_prop(&position);
                }
            }

            if let Some(bounds) = &mut part.get_bounds() {
                bounds.update(&part.get_vertices(), Some(&&self.get_velocity_prop()));
                part.set_bounds(&bounds);
            }
        }
    }

    fn set_force(&mut self, value: &Force) {
        content_mut!(self).force = *value;
    }

    pub fn apply_force(&mut self, position: &Position, force: &Force) {
        let offset = Vector::create(
            position.get_x() - self.get_position().get_x(),
            position.get_y() - self.get_position().get_y(),
        );

        let mut my_force = self.get_force();
        my_force.add_xy(force);
        self.set_force(&my_force);

        let mut torque = self.get_torque();
        torque += offset.get_x() * force.get_y() - offset.get_y() * force.get_x();
        self.set_torque(torque);
    }

    pub fn update_velocities(&mut self) {
        let time_scale = BASE_DELTA / self.get_delta_time().unwrap_or(1.);
        let mut body_velocity = self.get_velocity_prop();

        let position_prev = self.get_position_prev().unwrap_or(Position::new(0., 0.));

        body_velocity.set_x((self.get_position().get_x() - position_prev.get_x()) * time_scale);
        body_velocity.set_y((self.get_position().get_y() - position_prev.get_y()) * time_scale);

        self.set_speed_prop(f64::sqrt(
            (body_velocity.get_x() * body_velocity.get_x())
                + (body_velocity.get_y() * body_velocity.get_y()),
        ));

        self.set_angular_velocity_prop((self.get_angle() - self.get_angle_prev()) * time_scale);
        self.set_angular_speed_prop(f64::abs(self.get_angular_velocity_prop()));
    }
    // endregion: Actions

    pub fn set_one(&mut self, option: &BodyOption) {
        match option {
            BodyOption::Angle(_) => todo!(),
            BodyOption::AnglePrev(_) => todo!(),
            BodyOption::AngularSpeed(_) => todo!(),
            BodyOption::AngularVelocity(_) => todo!(),
            BodyOption::Area(value) => self.set_area(*value),
            BodyOption::Axes(_) => todo!(),
            BodyOption::Bounds(_) => todo!(),
            BodyOption::Chamfer(_) => todo!(),
            BodyOption::CircleRadius(_) => todo!(),
            BodyOption::CollisionFilter(_) => todo!(),
            BodyOption::ConstraintImpulse(_) => todo!(),
            BodyOption::DeltaTime(_) => todo!(),
            BodyOption::Density(_) => todo!(),
            BodyOption::Force(_) => todo!(),
            BodyOption::Friction(_) => todo!(),
            BodyOption::FrictionAir(_) => todo!(),
            BodyOption::FrictionStatic(_) => todo!(),
            BodyOption::Inertia(_) => todo!(),
            BodyOption::IsSensor(_) => todo!(),
            BodyOption::IsSleeping(_) => todo!(),
            BodyOption::IsStatic(_) => todo!(),
            BodyOption::Mass(_) => todo!(),
            BodyOption::Motion(_) => todo!(),
            BodyOption::Original(_) => todo!(),
            BodyOption::Parent(_) => todo!(),
            BodyOption::Parts(_) => todo!(),
            BodyOption::Position(_) => todo!(),
            BodyOption::PositionImpulse(_) => todo!(),
            BodyOption::PositionPrev(_) => todo!(),
            BodyOption::Render(_) => todo!(),
            BodyOption::Resitution(_) => todo!(),
            BodyOption::ShapeType(_) => todo!(),
            BodyOption::SleepThreshold(_) => todo!(),
            BodyOption::Slop(_) => todo!(),
            BodyOption::Speed(_) => todo!(),
            BodyOption::TimeScale(_) => todo!(),
            BodyOption::Torque(_) => todo!(),
            BodyOption::TotalContracts(_) => todo!(),
            BodyOption::Velocity(_) => todo!(),
            BodyOption::Vertices(_) => todo!(),
        }
    }
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
        geometry::{bounds::BoundsPart, vertex::{self, Vertex}},
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
            content: Arc::new(Mutex::new(content)),
            parent: Weak::new(),
        }
    }

    fn test_body() -> Body {
        let mut content = BodyContent::default_contant();
        content.id = common::next_id();
        content.angle = 42.;
        content.angle_prev = 41.;
        content.position = Position::new(2., 2.);
        content.position_prev = Some(Position::new(1., 1.));
        content.bounds = Some(test_bounds());
        content.velocity = Velocity::new(42., 42.);
        content.density = 1.1;
        content.time_scale = 93;
        content.delta_time = Some(3.2);
        content.friction_air = 9.7;
        content.force = Force::new(69., 79.);
        content.mass = 88.;
        content.torque = 52.;
        content.inertia = 32.;
        content.angular_velocity = 12.;
        content.resitution = 69.;
        content.friction = 666.;
        content.inverse_inertia = 16.;
        content.inverse_mass = 17.;
        content.area = 1600.;

        content.vertices = Vertices::new(test_square(), None);

        let axes = vec![
            Vertex::new(None, 1., 1., 0, false),
            Vertex::new(None, -1., -1., 1, false),
        ];
        content.axes = Some(Axes::new(&axes));

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
                part_content.time_scale = 93;
                part_content.delta_time = Some(3.2);
                part_content.friction_air = 9.7;
                part_content.force = Force::new(69., 79.);
                part_content.mass = 88.;
                part_content.torque = 52.;
                part_content.inertia = 32.;
                part_content.angular_velocity = 12.;
                part_content.resitution += increase;
                part_content.friction += increase;
                part_content.inverse_inertia += increase;
                part_content.inverse_mass += increase;

                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = Vertices::new(test_square(), None);
                for (index, vertex) in part_content.vertices.iter_mut().enumerate() {
                    vertex.set_x(vertex.get_x() + increase);
                    vertex.set_y(vertex.get_y() + increase);
                }

                body_from_content(part_content)
            })
            .collect_vec();
        content.parts = Some(parts);
        body_from_content(content)
    }
    // endregion: Helpers

    #[test]
    fn set_one_should_be_able_mutate_one_value_based_on_provided_option() {
        // Arrange
        let mut body = test_body();
        let option = BodyOption::Area(83.);

        // Act
        body.set_one(&option);

        // Assert
        assert_float(body.get_area(), 83.)
    }

    #[test]
    fn update_velocities_should_be_able_update_all_velocities_and_speeds() {
        // Arrange
        let mut body = test_body();

        // Act
        body.update_velocities();

        // Assert
        assert_float(body.get_speed_prop(), 7.365695637359869);
        assert_float(body.get_angular_velocity_prop(), 5.208333333333333);
        assert_float(body.get_angular_speed_prop(), 5.208333333333333);
    }

    #[test]
    fn set_static_should_be_able_to_set_a_default_body_to_static() {
        // Arrange
        let mut body = test_body();
        let is_static = true;

        // Act
        body.set_static(is_static);

        // Assert
        let part = body.get_parts()[0].clone();
        let original = part.get_original().unwrap();
        assert_float(part.get_resitution(), 0.);
        assert_float(part.get_friction(), 1.);
        assert_float(part.get_mass(), f64::INFINITY);
        assert_float(part.get_inertia(), f64::INFINITY);
        assert_float(part.get_density(), f64::INFINITY);
        assert_float(part.get_inverse_mass(), 0.);
        assert_xy(&part.get_position(), 2., 2.);
        assert_float(part.get_angle_prev(), 42.);
        assert_float(part.get_angular_velocity_prop(), 0.);
        assert_float(part.get_speed_prop(), 0.);
        assert_float(part.get_angular_speed_prop(), 0.);
        assert_float(part.get_motion(), 0.);
        assert_float(original.get_resitution(), 69.);
        assert_float(original.get_friction(), 666.);
        assert_float(original.get_mass(), 88.);
        assert_float(original.get_inertia(), 32.);
        assert_float(original.get_inverse_inertia(), 16.);
        assert_float(original.get_inverse_mass(), 17.);
        assert_float(original.get_density(), 1.1);

        let part = body.get_parts()[1].clone();
        let original = part.get_original().unwrap();
        assert_float(part.get_resitution(), 0.);
        assert_float(part.get_friction(), 1.);
        assert_float(part.get_mass(), f64::INFINITY);
        assert_float(part.get_inertia(), f64::INFINITY);
        assert_float(part.get_density(), f64::INFINITY);
        assert_float(part.get_inverse_mass(), 0.);
        assert_xy(&part.get_position(), 1., 1.);
        assert_float(part.get_angle_prev(), 43.);
        assert_float(part.get_angular_velocity_prop(), 0.);
        assert_float(part.get_speed_prop(), 0.);
        assert_float(part.get_angular_speed_prop(), 0.);
        assert_float(part.get_motion(), 0.);
        assert_float(original.get_density(), 2.1);
        assert_float(original.get_friction(), 667.);
        assert_float(original.get_inertia(), 32.);
        assert_float(original.get_inverse_inertia(), 17.);
        assert_float(original.get_inverse_mass(), 18.);
        assert_float(original.get_mass(), 88.);
        assert_float(original.get_resitution(), 70.);

        let part = body.get_parts()[2].clone();
        let original = part.get_original().unwrap();
        assert_float(part.get_resitution(), 0.);
        assert_float(part.get_friction(), 1.);
        assert_float(part.get_mass(), f64::INFINITY);
        assert_float(part.get_inertia(), f64::INFINITY);
        assert_float(part.get_density(), f64::INFINITY);
        assert_float(part.get_inverse_mass(), 0.);
        assert_xy(&part.get_position(), 2., 2.);
        assert_float(part.get_angle_prev(), 44.);
        assert_float(part.get_angular_velocity_prop(), 0.);
        assert_float(part.get_speed_prop(), 0.);
        assert_float(part.get_angular_speed_prop(), 0.);
        assert_float(part.get_motion(), 0.);
        assert_float(original.get_density(), 3.1);
        assert_float(original.get_friction(), 668.);
        assert_float(original.get_inertia(), 32.);
        assert_float(original.get_inverse_inertia(), 18.);
        assert_float(original.get_inverse_mass(), 19.);
        assert_float(original.get_mass(), 88.);
        assert_float(original.get_resitution(), 71.);
    }

    #[test]
    fn apply_force_should_be_able_to_update_a_body() {
        // Arrange
        let mut body = test_body();
        {
            let mut content = content_mut!(body);
            content.force = Force::new(3., 4.);
        }
        let position = Position::new(89., 99.);
        let force = Force::new(37., 42.);

        // Act
        body.apply_force(&position, &force);

        // Assert
        assert_xy(&body.get_force(), 40., 46.);
        assert_float(body.get_torque(), 117.);
    }

    #[test]
    fn update_should_be_able_to_update_a_body() {
        // Arrange
        let mut body = test_body();
        let delta_time = Some(common::BASE_DELTA);

        // Act
        body.update(delta_time);

        // Assert
        let mut part = body.get_parts()[0].clone();
        assert_float(part.get_angle(), 3467634.1875);
        assert_float(part.get_angle_prev(), 42.);
        assert_float(part.get_angular_velocity_prop(), 3467592.1875);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], -1.307352295833022, -0.5392865421833987);
        assert_xy(&axes[1], 1.307352295833022, 0.5392865421833987);
        assert_bounds(
            &part.get_bounds().unwrap(),
            1447308.789238613,
            1720320.1528749766,
            2894619.5005341135,
            3440642.227806841,
        );
        assert_float(part.get_delta_time().unwrap(), 1550.);
        assert_float(part.get_density(), 1.1);
        assert_xy(&part.get_force(), 69., 79.);
        assert_float(part.get_friction_air(), 9.7);
        assert_float(part.get_inertia(), 32.);
        assert_float(part.get_mass(), 88.);
        assert_xy(&part.get_position(), 1447310.096590909, 1720321.4602272725);
        assert_xy(&part.get_position_prev().unwrap(), 2., 2.);
        assert_float(part.get_time_scale() as f64, 93.);
        assert_float(part.get_torque(), 52.);
        assert_xy(
            &part.get_velocity_prop(),
            1447308.096590909,
            1720319.4602272725,
        );
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 1447311.4039432048, 1720321.9995138147);
        assert_xy(&vertices[1], 1447309.5573043667, 1720322.7675795683);
        assert_xy(&vertices[2], 1447308.789238613, 1720320.9209407303);
        assert_xy(&vertices[3], 1447310.6358774512, 1720320.1528749766);

        part = body.get_parts()[1].clone();
        assert_float(part.get_angle(), 43.);
        assert_float(part.get_angle_prev(), 42.);
        assert_float(part.get_angular_velocity_prop(), 12.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], -2.614704591666044, -1.0785730843667973);
        assert_xy(&axes[1], 2.614704591666044, 1.0785730843667973);
        assert_bounds(
            &part.get_bounds().unwrap(),
            1447307.4818863173,
            1720319.6135884344,
            2894618.193181818,
            3440641.6885202983,
        );
        assert_float(part.get_delta_time().unwrap(), 3.2);
        assert_float(part.get_density(), 2.1);
        assert_xy(&part.get_force(), 69., 79.);
        assert_float(part.get_friction_air(), 9.7);
        assert_float(part.get_inertia(), 32.);
        assert_float(part.get_mass(), 88.);
        assert_xy(&part.get_position(), 1447311.4039432048, 1720321.9995138147);
        assert_xy(&part.get_position_prev().unwrap(), 1., 1.);
        assert_float(part.get_time_scale() as f64, 93.);
        assert_float(part.get_torque(), 52.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 1447310.096590909, 1720321.4602272725);
        assert_xy(&vertices[1], 1447308.2499520709, 1720322.228293026);
        assert_xy(&vertices[2], 1447307.4818863173, 1720320.381654188);
        assert_xy(&vertices[3], 1447309.3285251553, 1720319.6135884344);

        part = body.get_parts()[2].clone();
        assert_float(part.get_angle(), 44.);
        assert_float(part.get_angle_prev(), 43.);
        assert_float(part.get_angular_velocity_prop(), 12.);
        let axes = part.get_axes().unwrap();
        assert_xy(&axes[0], -3.922056887499066, -1.6178596265501959);
        assert_xy(&axes[1], 3.922056887499066, 1.6178596265501959);
        assert_bounds(
            &part.get_bounds().unwrap(),
            1447306.1745340214,
            1720319.0743018922,
            2894616.8858295223,
            3440641.1492337566,
        );
        assert_float(part.get_delta_time().unwrap(), 3.2);
        assert_float(part.get_density(), 3.1);
        assert_xy(&part.get_force(), 69., 79.);
        assert_float(part.get_friction_air(), 9.7);
        assert_float(part.get_inertia(), 32.);
        assert_float(part.get_mass(), 88.);
        assert_xy(&part.get_position(), 1447310.096590909, 1720321.4602272725);
        assert_xy(&part.get_position_prev().unwrap(), 1., 1.);
        assert_float(part.get_time_scale() as f64, 93.);
        assert_float(part.get_torque(), 52.);
        assert_xy(&part.get_velocity_prop(), 42., 42.);
        let vertices = part.get_vertices();
        assert_xy(&vertices[0], 1447308.789238613, 1720320.9209407303);
        assert_xy(&vertices[1], 1447306.942599775, 1720321.6890064839);
        assert_xy(&vertices[2], 1447306.1745340214, 1720319.8423676458);
        assert_xy(&vertices[3], 1447308.0211728595, 1720319.0743018922);
    }

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
        let axes = vec![
            Vertex::new(None, 1., 1., 0, false),
            Vertex::new(None, -1., -1., 1, false),
        ];
        content.axes = Some(Axes::new(&axes));
        content.position = Position::new(0., 0.);
        content.bounds = Some(test_bounds());
        content.vertices = Vertices::new(test_square(), None);

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
                part_content.vertices = Vertices::new(test_square(), None);
                for (index, vertex) in part_content.vertices.iter_mut().enumerate() {
                    vertex.set_x(vertex.get_x() + increase);
                    vertex.set_y(vertex.get_y() + increase);
                }
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
        let axes = vec![
            Vertex::new(None, 1., 1., 0, false),
            Vertex::new(None, -1., -1., 1, false),
        ];
        content.axes = Some(Axes::new(&axes));
        content.position = Position::new(0., 0.);
        content.bounds = Some(test_bounds());
        content.vertices = Vertices::new(test_square(), None);

        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.angle += increase;
                part_content.angle_prev += increase;

                let mut axes = axes.clone();
                axes[0].set_x(*increase + 1.);
                axes[0].set_y(*increase + 1.);
                axes[1].set_x(-1. * (*increase + 1.));
                axes[1].set_y(-1. * (*increase + 1.));
                part_content.axes = Some(Axes::new(&axes));

                part_content.bounds = Some(test_bounds());
                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = Vertices::new(test_square(), None);
                for (index, vertex) in part_content.vertices.iter_mut().enumerate() {
                    vertex.set_x(vertex.get_x() + increase);
                    vertex.set_y(vertex.get_y() + increase);
                }
                body_from_content(part_content)
            })
            .collect_vec();
        content.parts = Some(parts);
        let mut body = body_from_content(content);

        let update_velocity = None;

        // Act
        body.set_angle(37., update_velocity);

        // Assert
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
        content.vertices = Vertices::new(test_square(), None);
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
                part_content.vertices = Vertices::new(test_square(), None);
                for (index, vertex) in part_content.vertices.iter_mut().enumerate() {
                    vertex.set_x(vertex.get_x() + increase);
                    vertex.set_y(vertex.get_y() + increase);
                }
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
        content.vertices = Vertices::new(test_square(), None);
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
                part_content.vertices = Vertices::new(test_square(), None);
                for (index, vertex) in part_content.vertices.iter_mut().enumerate() {
                    vertex.set_x(vertex.get_x() + increase);
                    vertex.set_y(vertex.get_y() + increase);
                }
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
        content.vertices = Vertices::new(test_square(), None);
        content.velocity = Velocity::new(42., 42.);

        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.bounds = Some(test_bounds());
                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = Vertices::new(test_square(), None);
                for (index, vertex) in part_content.vertices.iter_mut().enumerate() {
                    vertex.set_x(vertex.get_x() + increase);
                    vertex.set_y(vertex.get_y() + increase);
                }
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
        content.vertices = Vertices::new(test_square(), None);
        content.velocity = Velocity::new(42., 42.);

        let mut parts = [1., 2.]
            .iter()
            .map(|increase| {
                let mut part_content = content.clone();
                part_content.id = common::next_id();
                part_content.bounds = Some(test_bounds());
                part_content.position = Position::new(*increase, *increase);
                part_content.vertices = Vertices::new(test_square(), None);
                for (index, vertex) in part_content.vertices.iter_mut().enumerate() {
                    vertex.set_x(vertex.get_x() + increase);
                    vertex.set_y(vertex.get_y() + increase);
                }
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
        let vertices = Vertices::new(test_square(), None);

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
        body.set_vertices(&vertices);

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
        let mut body = test_body();
        let mass = 42.1;

        // Act
        body.set_mass(mass);

        // Assert
        assert_float(body.get_mass(), 42.1);
        assert_float(body.get_inverse_mass(), 0.023752969121140142);
        assert_float(body.get_inertia(), 15.30909090909091);
        assert_float(body.get_inverse_inertia(), 0.065320665083135387);
        assert_float(body.get_density(), 0.026312500000000003);
    }

    #[test]
    fn set_density_should_mutate_the_body_with_valid_values() {
        // Arrange
        let mut body = test_body();
        let density = 42.1;

        // Act
        body.set_density(density);

        // Assert
        assert_float(body.get_mass(), 67360.);
        assert_float(body.get_inverse_mass(), 0.000014845605700712589);
        assert_float(body.get_inertia(), 24494.545454545456);
        assert_float(body.get_inverse_inertia(), 0.00004082541567695962);
        assert_float(body.get_density(), 42.1);
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
