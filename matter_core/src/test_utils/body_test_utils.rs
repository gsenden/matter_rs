use crate::core::position::Position;
use crate::core::velocity::Velocity;
use crate::core::xy::XYGet;

use super::common_test_utils::assert_float;

pub fn assert_velocity(result: &Velocity, expected_x: f64, expected_y: f64) {
    assert_float(result.get_x(), expected_x);
    assert_float(result.get_y(), expected_y);
}

pub fn assert_position(result: &Position, expected_x: f64, expected_y: f64) {
    assert_float(result.get_x(), expected_x);
    assert_float(result.get_y(), expected_y);
}
