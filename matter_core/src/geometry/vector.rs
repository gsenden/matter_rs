use crate::core::xy::{XYNew, XY};

#[derive(Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl XY for Vector {
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

impl XYNew for Vector {
    type XY = Vector;

    fn new(x: f64, y: f64) -> Self {
        Vector { x: x, y: y }
    }
}

impl Vector {
    pub fn create(x: f64, y: f64) -> Vector {
        Vector::new(x, y)
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.get_x().powi(2) + self.get_y().powi(2)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn rotate_about(&mut self, angle: f64, point: &impl XY) {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = point.get_x()
            + ((self.get_x() - point.get_x()) * cos - (self.get_y() - point.get_y()) * sin);
        let y = point.get_y()
            + ((self.get_x() - point.get_x()) * sin + (self.get_y() - point.get_y()) * cos);
        self.set_x_y(x, y);
    }

    pub fn rotate(&mut self, angle: f64) {
        let point = Vector::create(0.0, 0.0);
        self.rotate_about(angle, &point);
    }

    pub fn normalise(&mut self) {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            Vector::new(0., 0.);
        } else {
            self.set_x(self.get_x() / magnitude);
            self.set_y(self.get_y() / magnitude);
        }
    }

    pub fn cross3(vector_a: &impl XY, vector_b: &impl XY, vector_c: &impl XY) -> f64 {
        (vector_b.get_x() - vector_a.get_x()) * (vector_c.get_y() - vector_a.get_y())
            - (vector_b.get_y() - vector_a.get_y()) * (vector_c.get_x() - vector_a.get_x())
    }

    pub fn cross(vector_a: &impl XY, vector_b: &impl XY) -> f64 {
        (vector_a.get_x() * vector_b.get_y()) - (vector_a.get_y() * vector_b.get_x())
    }

    pub fn perp(&mut self, negate: bool) {
        let negate_factor = if negate { -1.0 } else { 1.0 };

        let x = negate_factor * (self.get_y() * -1.0);
        let y = negate_factor * self.get_x();
        self.set_x_y(x, y);
    }

    pub fn angle(vector_a: &impl XY, vector_b: &impl XY) -> f64 {
        f64::atan2(
            vector_b.get_y() - vector_a.get_y(),
            vector_b.get_x() - vector_a.get_x(),
        )
    }
}
/*
pub fn create(x: f64, y: f64) -> Vector {
    Vector::new(x, y)
}

pub fn magnitude(vector: &impl XY) -> f64 {
    magnitude_squared(vector).sqrt()
}

pub fn magnitude_squared(vector: &impl XY) -> f64 {
    vector.get_x().powi(2) + vector.get_y().powi(2)
}

pub fn rotate(vector: &mut impl XY, angle: f64) {
    let point = create(0.0, 0.0);
    rotate_about(vector, angle, &point);
}

pub fn rotate_about(vector: &mut impl XY, angle: f64, point: &impl XY) {
    let cos = angle.cos();
    let sin = angle.sin();
    let x = point.get_x()
        + ((vector.get_x() - point.get_x()) * cos - (vector.get_y() - point.get_y()) * sin);
    let y = point.get_y()
        + ((vector.get_x() - point.get_x()) * sin + (vector.get_y() - point.get_y()) * cos);
    vector.set_x(x);
    vector.set_y(y);
}

pub fn normalise(vector: &impl XY) -> Vector {
    let magnitude = magnitude(vector);
    if magnitude == 0.0 {
        create(0.0_f64, 0.0_f64)
    } else {
        create(vector.get_x() / magnitude, vector.get_y() / magnitude)
    }
}

pub fn dot(vector: &impl XY, multiplier: &impl XY) -> f64 {
    vector.get_x() * multiplier.get_x() + vector.get_y() * multiplier.get_y()
}

pub fn cross3(vector_a: &impl XY, vector_b: &impl XY, vector_c: &impl XY) -> f64 {
    (vector_b.get_x() - vector_a.get_x()) * (vector_c.get_y() - vector_a.get_y())
        - (vector_b.get_y() - vector_a.get_y()) * (vector_c.get_x() - vector_a.get_x())
}

pub fn cross(vector_a: &impl XY, vector_b: &impl XY) -> f64 {
    (vector_a.get_x() * vector_b.get_y()) - (vector_a.get_y() * vector_b.get_x())
}

pub fn add(vector: &impl XY, vector_b: &impl XY) -> Vector {
    create(
        vector.get_x() + vector_b.get_x(),
        vector.get_y() + vector_b.get_y(),
    )
}

pub fn sub(vector_a: &impl XY, vector_b: &impl XY) -> Vector {
    create(
        vector_a.get_x() - vector_b.get_x(),
        vector_a.get_y() - vector_b.get_y(),
    )
}

pub fn mult(vector: &impl XY, scalar: f64) -> Vector {
    create(vector.get_x() * scalar, vector.get_y() * scalar)
}

pub fn div(vector: &impl XY, scalar: f64) -> Vector {
    create(vector.get_x() / scalar, vector.get_y() / scalar)
}

pub fn perp(vector: &impl XY, negate: bool) -> Vector {
    let negate_factor = if negate { -1.0 } else { 1.0 };
    let x = negate_factor * (vector.get_y() * -1.0);
    let y = negate_factor * vector.get_x();
    create(x, y)
}

pub fn neg(vector: &impl XY) -> Vector {
    mult(vector, -1.0)
}

pub fn angle(vector_a: &impl XY, vector_b: &impl XY) -> f64 {
    f64::atan2(
        vector_b.get_y() - vector_a.get_y(),
        vector_b.get_x() - vector_a.get_x(),
    )
}
*/
#[cfg(test)]
mod tests {
    use crate::{
        core::xy::XY,
        geometry::vector::{self, Vector},
        test_utils::{common_test_utils::assert_float, geometry_test_utils::assert_xy},
    };

    #[test]
    fn angle_should_calculate_a_valid_result() {
        // Arrange
        let vector_a: Vector = Vector::new(2.0, 4.0);
        let vector_b: Vector = Vector::new(5.0, 6.0);

        // Act
        let result = Vector::angle(&vector_a, &vector_b);

        // Assert
        assert_float(result, 0.5880026035475675_f64);
    }

    #[test]
    fn neg_should_calculate_a_valid_result() {
        // Arrange
        let mut vector: Vector = Vector::create(2.0, 4.0);

        // Act
        vector.neg();

        // Assert
        assert_xy(&vector, -2_f64, -4_f64);
    }

    #[test]
    fn perp_should_calculate_a_valid_result_for_negate_false() {
        // Arrange
        let mut vector: Vector = Vector::create(2.0, 4.0);
        let negate = false;

        // Act
        vector.perp(negate);

        // Assert
        assert_xy(&vector, -4_f64, 2_f64);
    }

    #[test]
    fn perp_should_calculate_a_valid_result_for_negate_true() {
        // Arrange
        let mut vector: Vector = Vector::create(2.0, 4.0);
        let negate = true;

        // Act
        vector.perp(negate);

        // Assert
        assert_xy(&vector, 4_f64, -2_f64);
    }

    #[test]
    fn div_should_calculate_a_valid_result() {
        // Arrange
        let mut vector: Vector = Vector::create(2.0, 4.0);
        let scalar = 2_f64;

        // Act
        vector.div(scalar);

        // Assert
        assert_xy(&vector, 1_f64, 2_f64);
    }

    #[test]
    fn mult_should_calculate_a_valid_result() {
        // Arrange
        let mut vector: Vector = Vector::create(2.0, 3.0);
        let scalar = 2_f64;

        // Act
        vector.mult(scalar);

        // Assert
        assert_xy(&vector, 4_f64, 6_f64);
    }

    #[test]
    fn sub_should_calculate_a_valid_result() {
        // Arrange
        let mut vector_a: Vector = Vector::create(4.0, 5.0);
        let vector_b: Vector = Vector::create(2.0, 3.0);

        // Act
        vector_a.sub(&vector_b);

        // Assert
        assert_xy(&vector_a, 2_f64, 2_f64);
    }

    #[test]
    fn add_should_calculate_a_valid_result() {
        // Arrange
        let vector_a: Vector = Vector::create(2.0, 3.0);
        let vector_b: Vector = Vector::create(4.0, 5.0);

        // Act
        let result = Vector::add(&vector_a, &vector_b);

        // Assert
        assert_xy(&result, 6_f64, 8_f64);
    }

    #[test]
    fn cross3_should_calculate_a_valid_result() {
        // Arrange
        let vector_a: Vector = Vector::create(2.0, 3.0);
        let vector_b: Vector = Vector::create(4.0, 5.0);
        let vector_c: Vector = Vector::create(6.0, 8.0);

        // Act
        let result = Vector::cross3(&vector_a, &vector_b, &vector_c);

        // Assert
        assert_float(result, 2.0);
    }

    #[test]
    fn cross_should_calculate_a_valid_result() {
        // Arrange
        let vector_a: Vector = Vector::create(2.0, 3.0);
        let vector_b: Vector = Vector::create(4.0, 5.0);

        // Act
        let result = Vector::cross(&vector_a, &vector_b);

        // Assert
        assert_float(result, -2.0);
    }

    #[test]
    fn dot_should_calculate_a_valid_result() {
        // Arrange
        let mut vector: Vector = Vector::create(2.0, 3.0);
        let multiplier: Vector = Vector::create(4.0, 5.0);

        // Act
        vector.dot(&multiplier);

        // Assert
        assert_float(&vector, 23.0);
    }

    #[test]
    fn normalise_about_should_mutate_to_valid_result() {
        // Arrange
        let mut vector: Vector = Vector::create(10.0, 2.0);

        // Act
        vector.normalise();

        // Assert
        assert_xy(&vector, 0.9805806756909202_f64, 0.19611613513818404_f64);
    }

    #[test]
    fn rotate_about_should_mutate_to_valid_result() {
        // Arrange
        let mut vector: Vector = Vector::create(10.0, 2.0);
        let point = Vector::create(2.0, 2.0);
        let angle = -2_f64;

        // Act
        vector.rotate_about(angle, &point);

        // Assert
        assert_xy(&vector, -1.3291746923771393_f64, -5.274379414605454_f64);
    }

    #[test]
    fn rotate_should_mutate_to_valid_result() {
        // Arrange
        let mut vector: Vector = Vector::create(10.0, 2.0);
        let angle = -2_f64;

        // Act
        vector.rotate(angle);

        // Assert
        assert_xy(&vector, -2.3428735118200605_f64, -9.925267941351102_f64);
    }

    #[test]
    fn magnitude_squared_should_return_valid_result() {
        // Arrange
        let mut vector: Vector = Vector::create(10.0, 2.0);

        // Act
        let result = vector.magnitude_squared();

        // Assert
        assert_float(result, 104.0_f64);
    }

    #[test]
    fn magnitude_should_be_able_to_deal_with_zero() {
        // Arrange
        let mut vector: Vector = Vector::create(0.0, 0.0);

        // Act
        let result = vector.magnitude();

        // Assert
        assert_float(result, 0.0_f64);
    }

    #[test]
    fn magnitude_should_return_valid_result() {
        // Arrange
        let mut vector: Vector = Vector::create(5.0, 3.0);

        // Act
        let result = vector.magnitude();

        // Assert
        assert_float(result, 5.830951894845301);
    }
}
