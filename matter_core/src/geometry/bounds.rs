use crate::core::{velocity::Velocity, xy::XYGet};

use super::{vector::Vector, vertices::Vertex};

#[derive(Clone, Copy)]
pub struct BoundsPart {
    pub x: f64,
    pub y: f64,
}

impl XYGet for BoundsPart {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

impl BoundsPart {
    pub fn new(x: f64, y: f64) -> Self {
        BoundsPart { x: x, y: y }
    }
}

#[derive(Clone, Copy)]
pub struct Bounds {
    pub min: BoundsPart,
    pub max: BoundsPart,
}

impl Bounds {
    pub fn new(min: BoundsPart, max: BoundsPart) -> Self {
        Bounds { min: min, max: max }
    }

    pub fn get_min(&self) -> BoundsPart {
        self.min
    }

    pub fn get_max(&self) -> BoundsPart {
        self.max
    }
}

pub fn update(bounds: &mut Bounds, vertices: &Vec<Vertex>, velocity: Option<&Velocity>) {
    bounds.min.x = f64::INFINITY;
    bounds.max.x = -f64::INFINITY;
    bounds.min.y = f64::INFINITY;
    bounds.max.y = -f64::INFINITY;

    for vertex in vertices {
        if vertex.get_x() > bounds.max.x {
            bounds.max.x = vertex.get_x();
        }
        if vertex.get_x() < bounds.min.x {
            bounds.min.x = vertex.get_x();
        }
        if vertex.get_y() > bounds.max.y {
            bounds.max.y = vertex.get_y();
        }
        if vertex.get_y() < bounds.min.y {
            bounds.min.y = vertex.get_y()
        }
    }

    if velocity.is_some() {
        let velocity = velocity.unwrap();
        if velocity.get_x() > 0.0 {
            bounds.max.x += velocity.get_x();
        } else {
            bounds.min.y += velocity.get_y();
        }

        if velocity.get_y() > 0.0 {
            bounds.max.y += velocity.get_y();
        } else {
            bounds.min.y += velocity.get_y();
        }
    }
}

pub fn create(vertices: Option<&Vec<Vertex>>) -> Bounds {
    let mut bounds = Bounds {
        min: BoundsPart::new(0., 0.),
        max: BoundsPart::new(0., 0.),
    };

    if vertices.is_some() {
        let vertices = vertices.unwrap();
        update(&mut bounds, vertices, None);
    }

    bounds
}

pub fn contains(bounds: &Bounds, point: &impl XYGet) -> bool {
    point.get_x() >= bounds.min.x
        && point.get_x() <= bounds.max.get_x()
        && point.get_y() >= bounds.min.get_y()
        && point.get_y() <= bounds.max.get_y()
}

pub fn overlaps(bounds_a: &Bounds, bounds_b: &Bounds) -> bool {
    bounds_a.min.x <= bounds_b.max.x
        && bounds_a.max.get_x() >= bounds_b.min.get_x()
        && bounds_a.max.get_y() >= bounds_b.min.get_y()
        && bounds_a.min.get_y() <= bounds_b.max.get_y()
}

pub fn translate(bounds: &mut Bounds, vector: &impl XYGet) {
    bounds.min.x += vector.get_x();
    bounds.max.x += vector.get_x();
    bounds.min.y += vector.get_y();
    bounds.max.y += vector.get_y();
}

pub fn shift(bounds: &mut Bounds, position: &impl XYGet) {
    let delta_x = bounds.max.x - bounds.min.x;
    let delta_y = bounds.max.y - bounds.min.y;

    bounds.min.x = position.get_x();
    bounds.max.x = position.get_x() + delta_x;
    bounds.min.y = position.get_y();
    bounds.max.y = position.get_y() + delta_y;
}

#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use crate::geometry::{vector, vertices};
    use crate::test_utils::geometry_test_utils::{
        assert_bounds, test_bounds, test_square_with_decimals,
    };

    use super::*;

    #[test]
    fn shift_should_mutate_bounds_by_vector() {
        // Arrange
        let mut bounds = test_bounds();
        let vector = vector::create(4.0, 1.0);

        // Act
        shift(&mut bounds, &vector);

        // Assert
        assert_bounds(&bounds, 4.0, 1.0, 104.0, 101.0);
    }

    #[test]
    fn translate_should_mutate_bounds_by_vector() {
        // Arrange
        let mut bounds = test_bounds();
        let vector = vector::create(4.0, 1.0);

        // Act
        translate(&mut bounds, &vector);

        // Assert
        assert_bounds(&bounds, 104.0, 151.0, 204.0, 251.0);
    }

    #[test]
    fn overlaps_should_return_false_for_two_non_overlapping_bounds() {
        // Arrange
        let bounds1 = test_bounds();
        let mut bounds2 = test_bounds();
        bounds2.min.x = 1.0;
        bounds2.min.y = 1.0;
        bounds2.max.x = 10.0;
        bounds2.max.y = 10.0;

        // Act
        let result = overlaps(&bounds1, &bounds2);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn overlaps_should_return_true_for_two_overlapping_bounds() {
        // Arrange
        let bounds1 = test_bounds();
        let mut bounds2 = test_bounds();
        bounds2.max.x += 100.0;
        bounds2.max.y += 100.0;

        // Act
        let result = overlaps(&bounds1, &bounds2);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn contains_should_return_false_for_vector_outside_bounds() {
        // Arrange
        let bounds = test_bounds();
        let vector = vector::create(1.0, 1.0);

        // Act
        let result = contains(&bounds, &vector);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn contains_should_return_true_for_vector_inside_bounds() {
        // Arrange
        let bounds = test_bounds();
        let vector = vector::create(101.0, 151.0);

        // Act
        let result = contains(&bounds, &vector);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn create_should_create_a_valid_bounds_without_vertices() {
        // Arrange

        // Act
        let result = create(None);

        // Assert
        assert_bounds(&result, 0.0, 0.0, 0.0, 0.0);
    }

    #[test]
    fn create_should_create_a_valid_bounds_from_vertices() {
        let points = test_square_with_decimals();
        let vertices = vertices::create(points, Uuid::new_v4());

        // Act
        let result = create(Some(&vertices));

        // Assert
        assert_bounds(&result, 0.0, 0.0, 40.1, 40.1);
    }

    #[test]
    fn update_should_mutate_bounds_with_vertices_without_velocity() {
        let points = test_square_with_decimals();
        let vertices = vertices::create(points, Uuid::new_v4());
        let mut bounds = test_bounds();

        // Act
        update(&mut bounds, &vertices, None);

        // Assert
        assert_bounds(&bounds, 0.0, 0.0, 40.1, 40.1);
    }

    #[test]
    fn update_should_mutate_bounds_with_vertices_and_velocity() {
        let points = test_square_with_decimals();
        let vertices = vertices::create(points, Uuid::new_v4());
        let velocity = Velocity::new(5., 6.);
        let mut bounds = test_bounds();

        // Act
        update(&mut bounds, &vertices, Some(&velocity));

        // Assert
        assert_bounds(&bounds, 0.0, 0.0, 45.1, 46.1);
    }
}
