use crate::core::{
    velocity::{self, Velocity},
    xy::{XYNew, XY},
};

use super::vertices::Vertices;

#[derive(Clone, Copy)]
pub struct BoundsPart {
    pub x: f64,
    pub y: f64,
}

impl XY for BoundsPart {
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

impl XYNew for BoundsPart {
    type XY = BoundsPart;
    fn new(x: f64, y: f64) -> Self {
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

    pub fn update(&mut self, vertices: &Vertices, velocity: Option<&Velocity>) {
        self.min.set_x(f64::INFINITY);
        self.max.set_x(-f64::INFINITY);
        self.min.set_y(f64::INFINITY);
        self.max.set_y(-f64::INFINITY);

        for vertex in vertices.iter() {
            if vertex.get_x() > self.max.get_x() {
                self.max.set_x(vertex.get_x());
            }
            if vertex.get_x() < self.min.get_x() {
                self.min.set_x(vertex.get_x());
            }
            if vertex.get_y() > self.max.get_y() {
                self.max.set_y(vertex.get_y());
            }
            if vertex.get_y() < self.min.get_y() {
                self.min.set_y(vertex.get_y());
            }
        }

        if let Some(velocity) = velocity {
            if velocity.get_x() > 0.0 {
                self.max.add_x(velocity.get_x());
            } else {
                self.min.add_x(velocity.get_x());
            }

            if velocity.get_y() > 0.0 {
                self.max.add_y(velocity.get_y());
            } else {
                self.min.add_y(velocity.get_y());
            }
        }
    }

    pub fn create(vertices: Option<Vertices>) -> Bounds {
        let mut bounds = Bounds {
            min: BoundsPart::new(0., 0.),
            max: BoundsPart::new(0., 0.),
        };

        if let Some(vertices) = vertices {
            bounds.update(&vertices, None);
        }
        bounds
    }

    pub fn contains(&self, point: &impl XY) -> bool {
        point.get_x() >= self.min.get_x()
            && point.get_x() <= self.max.get_x()
            && point.get_y() >= self.min.get_y()
            && point.get_y() <= self.max.get_y()
    }

    pub fn overlaps(bounds_a: &Bounds, bounds_b: &Bounds) -> bool {
        bounds_a.min.get_x() <= bounds_b.max.get_x()
            && bounds_a.max.get_x() >= bounds_b.min.get_x()
            && bounds_a.max.get_y() >= bounds_b.min.get_y()
            && bounds_a.min.get_y() <= bounds_b.max.get_y()
    }

    pub fn translate(&mut self, vector: &impl XY) {
        self.min.add_x(vector.get_x());
        self.max.add_x(vector.get_x());
        self.min.add_y(vector.get_y());
        self.max.add_y(vector.get_y());
    }

    pub fn shift(&mut self, position: &impl XY) {
        let delta_x = self.max.get_x() - self.min.get_x();
        let delta_y = self.max.get_y() - self.min.get_y();

        self.min.set_x(position.get_x());
        self.max.set_x(position.get_x() + delta_x);
        self.min.set_y(position.get_y());
        self.max.set_y(position.get_y() + delta_y);
    }
}

#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use crate::geometry::vector::Vector;
    use crate::geometry::{vector, vertices};
    use crate::test_utils::geometry_test_utils::{
        assert_bounds, test_bounds, test_square_with_decimals,
    };

    use super::*;

    #[test]
    fn shift_should_mutate_bounds_by_vector() {
        // Arrange
        let mut bounds = test_bounds();
        let vector = Vector::create(4.0, 1.0);

        // Act
        bounds.shift(&vector);

        // Assert
        assert_bounds(&bounds, 4.0, 1.0, 104.0, 101.0);
    }

    #[test]
    fn translate_should_mutate_bounds_by_vector() {
        // Arrange
        let mut bounds = test_bounds();
        let vector = Vector::create(4.0, 1.0);

        // Act
        bounds.translate(&vector);

        // Assert
        assert_bounds(&bounds, 104.0, 151.0, 204.0, 251.0);
    }

    #[test]
    fn overlaps_should_return_false_for_two_non_overlapping_bounds() {
        // Arrange
        let bounds_a = test_bounds();
        let mut bounds_b = test_bounds();
        bounds_b.min.x = 1.0;
        bounds_b.min.y = 1.0;
        bounds_b.max.x = 10.0;
        bounds_b.max.y = 10.0;

        // Act
        let result = Bounds::overlaps(&bounds_a, &bounds_b);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn overlaps_should_return_true_for_two_overlapping_bounds() {
        // Arrange
        let bounds_a = test_bounds();
        let mut bounds_b = test_bounds();
        bounds_b.max.x += 100.0;
        bounds_b.max.y += 100.0;

        // Act
        let result = Bounds::overlaps(&bounds_a, &bounds_b);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn contains_should_return_false_for_vector_outside_bounds() {
        // Arrange
        let bounds = test_bounds();
        let vector = Vector::create(1.0, 1.0);

        // Act
        let result = bounds.contains(&vector);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn contains_should_return_true_for_vector_inside_bounds() {
        // Arrange
        let bounds = test_bounds();
        let vector = Vector::create(101.0, 151.0);

        // Act
        let result = bounds.contains(&vector);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn create_should_create_a_valid_bounds_without_vertices() {
        // Arrange

        // Act
        let result = Bounds::create(None);

        // Assert
        assert_bounds(&result, 0.0, 0.0, 0.0, 0.0);
    }

    #[test]
    fn create_should_create_a_valid_bounds_from_vertices() {
        let points = test_square_with_decimals();
        let vertices = Vertices::create(points, None);

        // Act
        let result = Bounds::create(Some(&vertices));

        // Assert
        assert_bounds(&result, 0.0, 0.0, 40.1, 40.1);
    }

    #[test]
    fn update_should_mutate_bounds_with_vertices_without_velocity() {
        let points = test_square_with_decimals();
        let vertices = Vertices::create(points, None);
        let mut bounds = test_bounds();

        // Act
        bounds.update(&vertices, &None);

        // Assert
        assert_bounds(&bounds, 0.0, 0.0, 40.1, 40.1);
    }

    #[test]
    fn update_should_mutate_bounds_with_vertices_and_velocity() {
        let points = test_square_with_decimals();
        let vertices = Vertices::create(points, None);
        let velocity = Velocity::new(5., 6.);
        let mut bounds = test_bounds();

        // Act
        bounds.update(&vertices, &Some(&velocity));

        // Assert
        assert_bounds(&bounds, 0.0, 0.0, 45.1, 46.1);
    }
}
