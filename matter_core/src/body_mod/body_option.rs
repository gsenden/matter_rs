use crate::{
    core::{
        collision_filter::CollisionFilter, common::ShapeType,
        constraint_impulse::ConstraintImpulse, force::Force, position::Position, render::Render,
        velocity::Velocity,
    },
    geometry::{bounds::Bounds, vector::Vector},
};

use super::body::Body;

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
    //Parent(Body),
    Axes(Vec<Vector>),
    Area(f64),
    Mass(f64),
    Inertia(f64),
    DeltaTime(f64),
    Original(Body),
}
