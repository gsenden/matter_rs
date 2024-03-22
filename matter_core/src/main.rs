pub mod core;
mod geometry;
use crate::geometry::vector;

fn main() {
    let vector = vector::create(2.0, 2.0);
    println!("Hello, world! {}", vector.x);
}
