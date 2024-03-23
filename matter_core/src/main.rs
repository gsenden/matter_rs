mod core;
use crate::core::common;

mod test_utils;
use crate::test_utils::geometry_test_utils;

mod geometry;
use crate::geometry::vector;

fn main() {
    let vector = vector::create(2.0, 2.0);
    println!("Hello, world! {}", vector.x);
}
