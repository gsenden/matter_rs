use crate::{
    body::body::Body,
    core::xy::{XYNew, XY},
};

use super::vector::Vector;

#[derive(Clone)]
pub struct Vertex {
    body: Option<Body>,
    x: f64,
    y: f64,
    index: usize,
    is_internal: bool,
}

impl XY for Vertex {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}

impl From<Vector> for Vertex {
    fn from(value: Vector) -> Self {
        Vertex::from_xy(None, &value, 0, false)
    }
}

impl Vertex {
    pub fn from_xy(body: Option<Body>, xy: &impl XY, index: usize, is_internal: bool) -> Self {
        Vertex::new(body, xy.get_x(), xy.get_y(), index, is_internal)
    }

    pub fn new(body: Option<Body>, x: f64, y: f64, index: usize, is_internal: bool) -> Self {
        Vertex {
            body: body,
            x: x,
            y: y,
            index: index,
            is_internal: is_internal,
        }
    }

    pub fn get_body(&self) -> Option<Body> {
        self.body.clone()
    }

    pub fn set_body(&mut self, body: Body) {
        self.body = Some(body);
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_is_internal(&self) -> bool {
        self.is_internal
    }

    pub fn set_is_interal(&mut self, is_internal: bool) {
        self.is_internal = is_internal
    }
}
