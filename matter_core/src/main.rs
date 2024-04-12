mod core;
use crate::core::common;

mod test_utils;
use crate::core::xy::XYGet;
use crate::test_utils::geometry_test_utils;

mod geometry;
use crate::geometry::vector;

mod body_mod;
use crate::body_mod::body;

fn main() {
    let vector = vector::create(2.0, 2.0);
    println!("Hello, world! {}", vector.get_x());
}
