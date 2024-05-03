use uuid::Uuid;

use crate::{
    body_mod::body::Body, core::xy::XY, geometry::{
        bounds::{Bounds, BoundsPart},
        vector,
        vertices::Vertex,
    }
};

#[cfg(test)]
use super::super::geometry;
use super::{super::geometry::vector::Vector, common_test_utils::assert_float};

pub fn assert_xy(result: &impl XY, expected_x: f64, expected_y: f64) {
    assert_float(result.get_x(), expected_x);
    assert_float(result.get_y(), expected_y);
}

pub fn assert_vertex(
    result: &Vertex,
    expected_body: Option<Body>,
    expected_x: f64,
    expected_y: f64,
    expected_index: usize,
    expected_is_internal: bool,
) {
    match expected_body {
        Some(body) => assert_eq!(body.get_id(),result.get_body().unwrap().get_id()),
        None => assert_eq!(result.get_body().is_none(), true),
    }

    assert_float(result.get_x(), expected_x);
    assert_float(result.get_y(), expected_y);
    assert_eq!(result.get_index(), expected_index);
    assert_eq!(result.get_is_internal(), expected_is_internal);
}

pub fn assert_bounds(result: &Bounds, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
    assert_float(result.min.x, min_x);
    assert_float(result.min.y, min_y);
    assert_float(result.max.x, max_x);
    assert_float(result.max.y, max_y);
}

pub fn vec_vector_to_vec_vertex(vectors: Vec<Vector>) -> Vec<Vertex> {
    let mut index = 0_usize;
    vectors
        .iter()
        .map(|vector| {
            index += 1;
            Vertex::from_vector(None, vector, index - 1, false)
        })
        .collect()
}

pub fn test_square() -> Vec<Vector> {
    let point_a = vector::create(1.0, 1.0);
    let point_b = vector::create(3.0, 1.0);
    let point_c = vector::create(3.0, 3.0);
    let point_d = vector::create(1.0, 3.0);
    vec![point_a, point_b, point_c, point_d]
}

pub fn test_square_with_decimals() -> Vec<Vector> {
    let point_a = vector::create(0.0, 0.0);
    let point_b = vector::create(40.1, 0.0);
    let point_c = vector::create(40.1, 40.1);
    let point_d = vector::create(0.0, 40.1);
    vec![point_a, point_b, point_c, point_d]
}

pub fn test_square_with_decimals_signed() -> Vec<Vector> {
    let point_a = vector::create(0.0, 0.0);
    let point_b = vector::create(-40.1, 0.0);
    let point_c = vector::create(-40.1, -40.1);
    let point_d = vector::create(0.0, -40.1);
    vec![point_a, point_b, point_c, point_d]
}

pub fn test_shape_convex() -> Vec<Vector> {
    let point_a = vector::create(40.1, 40.1);
    let point_b = vector::create(0.0, 40.1);
    let point_c = vector::create(0.0, 0.0);
    let point_d = vector::create(40.1, 0.0);
    vec![point_a, point_b, point_c, point_d]
}

pub fn test_shape_non_convex() -> Vec<Vector> {
    let point_a = vector::create(1.0, 1.0);
    let point_b = vector::create(5.0, 1.0);
    let point_c = vector::create(5.0, 3.0);
    let point_d = vector::create(4.0, 4.0);
    let point_e = vector::create(3.0, 3.0);
    let point_f = vector::create(2.0, 4.0);
    let point_g = vector::create(1.0, 3.0);

    vec![
        point_a, point_b, point_c, point_d, point_e, point_f, point_g,
    ]
}

pub fn test_bounds() -> Bounds {
    Bounds {
        min: BoundsPart { x: 100.0, y: 150.0 },
        max: BoundsPart { x: 200.0, y: 250.0 },
    }
}
