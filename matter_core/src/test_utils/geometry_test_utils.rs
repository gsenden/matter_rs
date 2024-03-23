use crate::geometry::vector;

#[cfg(test)]
use super::super::geometry;
use super::{super::geometry::vector::Vector, common_test_utils::assert_float};

pub fn assert_vector(result: &Vector, expected_x: f64, expected_y: f64) {
    assert_float(result.x, expected_x);
    assert_float(result.y, expected_y);
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
