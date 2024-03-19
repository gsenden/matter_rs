pub struct Vector {
    pub x: f64,
    pub y: f64,
}

pub fn create(x: f64, y: f64) -> Vector {
    Vector { x: x, y: y }
}

pub fn magnitude(vector: &Vector) -> f64 {
    magnitude_squared(vector).sqrt()
}

pub fn magnitude_squared(vector: &Vector) -> f64 {
    vector.x.powi(2) + vector.y.powi(2)
}

pub fn rotate(vector: &Vector, angle: f64) -> Vector {
    let point = create(0.0, 0.0);
    rotate_about(vector, angle, &point)
}

pub fn rotate_about(vector: &Vector, angle: f64, point: &Vector) -> Vector {
    let cos = angle.cos();
    let sin = angle.sin();
    let x = point.x + ((vector.x - point.x) * cos - (vector.y - point.y) * sin);
    let y = point.y + ((vector.x - point.x) * sin + (vector.y - point.y) * cos);
    create(x, y)
}

pub fn normalise(vector: &Vector) -> Vector {
    let magnitude = magnitude(vector);
    if magnitude == 0.0 {
        create(0.0_f64, 0.0_f64)
    } else {
        create(vector.x / magnitude, vector.y / magnitude)
    }
}

pub fn dot(vector: &Vector, multiplier: &Vector) -> f64 {
    vector.x * multiplier.x + vector.y * multiplier.y
}

pub fn cross3(vector_a: &Vector, vector_b: &Vector, vector_c: &Vector) -> f64 {
    (vector_b.x - vector_a.x) * (vector_c.y - vector_a.y)
        - (vector_b.y - vector_a.y) * (vector_c.x - vector_a.x)
}

pub fn cross(vector_a: &Vector, vector_b: &Vector) -> f64 {
    (vector_a.x * vector_b.y) - (vector_a.y * vector_b.x)
}

pub fn add(vector: &Vector, vector_b: &Vector) -> Vector {
    create(vector.x + vector_b.x, vector.y + vector_b.y)
}

pub fn sub(vector_a: &Vector, vector_b: &Vector) -> Vector {
    create(vector_a.x - vector_b.x, vector_a.y - vector_b.y)
}

pub fn mult(vector: &Vector, scalar: f64) -> Vector {
    create(vector.x * scalar, vector.y * scalar)
}

pub fn div(vector: &Vector, scalar: f64) -> Vector {
    create(vector.x / scalar, vector.y / scalar)
}

pub fn perp(vector: &Vector, negate: bool) -> Vector {
    let negate_factor = if negate { -1.0 } else { 1.0 };
    let x = negate_factor * (vector.y * -1.0);
    let y = negate_factor * vector.x;
    create(x, y)
}

pub fn neg(vector: &Vector) -> Vector {
    mult(vector, -1.0)
}

pub fn angle(vector_a: &Vector, vector_b: &Vector) -> f64 {
    f64::atan2(vector_b.y - vector_a.y, vector_b.x - vector_a.x)
}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::{self, Vector};
    use float_cmp::ApproxEq;

    #[test]
    fn angle_should_calculate_a_valid_result() {
        // Arrange
        let vector_a: Vector = vector::create(2.0, 4.0);
        let vector_b: Vector = vector::create(5.0, 6.0);

        // Act
        let result = vector::angle(&vector_a, &vector_b);

        // Assert
        let expected = 0.5880026035475675_f64;
        assert!(result.approx_eq(expected, (0.0, 2)));
    }

    #[test]
    fn neg_should_calculate_a_valid_result() {
        // Arrange
        let vector: Vector = vector::create(2.0, 4.0);

        // Act
        let result: Vector = vector::neg(&vector);

        // Assert
        let expected_x = -2_f64;
        let expected_y = -4_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn perp_should_calculate_a_valid_result_for_negate_false() {
        // Arrange
        let vector: Vector = vector::create(2.0, 4.0);
        let negate = false;

        // Act
        let result = vector::perp(&vector, negate);

        // Assert
        let expected_x = -4_f64;
        let expected_y = 2_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn perp_should_calculate_a_valid_result_for_negate_true() {
        // Arrange
        let vector: Vector = vector::create(2.0, 4.0);
        let negate = true;

        // Act
        let result = vector::perp(&vector, negate);

        // Assert
        let expected_x = 4_f64;
        let expected_y = -2_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn div_should_calculate_a_valid_result() {
        // Arrange
        let vector: Vector = vector::create(2.0, 4.0);
        let scalar = 2_f64;

        // Act
        let result = vector::div(&vector, scalar);

        // Assert
        let expected_x = 1_f64;
        let expected_y = 2_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn mult_should_calculate_a_valid_result() {
        // Arrange
        let vector: Vector = vector::create(2.0, 3.0);
        let scalar = 2_f64;

        // Act
        let result = vector::mult(&vector, scalar);

        // Assert
        let expected_x = 4_f64;
        let expected_y = 6_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn sub_should_calculate_a_valid_result() {
        // Arrange
        let vector_a: Vector = vector::create(4.0, 5.0);
        let vector_b: Vector = vector::create(2.0, 3.0);

        // Act
        let result = vector::sub(&vector_a, &vector_b);

        // Assert
        let expected_x = 2_f64;
        let expected_y = 2_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn add_should_calculate_a_valid_result() {
        // Arrange
        let vector_a: Vector = vector::create(2.0, 3.0);
        let vector_b: Vector = vector::create(4.0, 5.0);

        // Act
        let result = vector::add(&vector_a, &vector_b);

        // Assert
        let expected_x = 6_f64;
        let expected_y = 8_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn cross3_should_calculate_a_valid_result() {
        // Arrange
        let vector_a: Vector = vector::create(2.0, 3.0);
        let vector_b: Vector = vector::create(4.0, 5.0);
        let vector_c: Vector = vector::create(6.0, 8.0);

        // Act
        let result = vector::cross3(&vector_a, &vector_b, &vector_c);

        // Assert
        let expected = 2_f64;
        assert!(result.approx_eq(expected, (0.0, 2)));
    }

    #[test]
    fn cross_should_calculate_a_valid_result() {
        // Arrange
        let vector_a: Vector = vector::create(2.0, 3.0);
        let vector_b: Vector = vector::create(4.0, 5.0);

        // Act
        let result = vector::cross(&vector_a, &vector_b);

        // Assert
        let expected = -2_f64;
        assert!(result.approx_eq(expected, (0.0, 2)));
    }

    #[test]
    fn dot_should_calculate_a_valid_result() {
        // Arrange
        let vector: Vector = vector::create(2.0, 3.0);
        let multiplier: Vector = vector::create(4.0, 5.0);

        // Act
        let result = vector::dot(&vector, &multiplier);

        // Assert
        let expected = 23_f64;
        assert!(result.approx_eq(expected, (0.0, 2)));
    }

    #[test]
    fn normalise_about_should_mutate_to_valid_result() {
        // Arrange
        let vector: Vector = vector::create(10.0, 2.0);

        // Act
        let result = vector::normalise(&vector);

        // Assert
        let expected_x = 0.9805806756909202_f64;
        let expected_y = 0.19611613513818404_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn rotate_about_should_mutate_to_valid_result() {
        // Arrange
        let vector: Vector = vector::create(10.0, 2.0);
        let point = vector::create(2.0, 2.0);
        let angle = -2_f64;

        // Act
        let result = vector::rotate_about(&vector, angle, &point);

        // Assert
        let expected_x = -1.3291746923771393_f64;
        let expected_y = -5.274379414605454_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn rotate_should_mutate_to_valid_result() {
        // Arrange
        let vector: Vector = vector::create(10.0, 2.0);
        let angle = -2_f64;

        // Act
        let result = vector::rotate(&vector, angle);

        // Assert
        let expected_x = -2.3428735118200605_f64;
        let expected_y = -9.925267941351102_f64;
        assert!(result.x.approx_eq(expected_x, (0.0, 2)));
        assert!(result.y.approx_eq(expected_y, (0.0, 2)));
    }

    #[test]
    fn magnitude_squared_should_return_valid_result() {
        // Arrange
        let vector: Vector = vector::create(10.0, 2.0);

        // Act
        let result = vector::magnitude_squared(&vector);

        // Assert
        let expected = 104.0_f64;
        assert!(result.approx_eq(expected, (0.0, 2)));
    }

    #[test]
    fn magnitude_should_be_able_to_deal_with_zero() {
        // Arrange
        let vector: Vector = vector::create(0.0, 0.0);

        // Act
        let result = vector::magnitude(&vector);

        // Assert
        let expected = 0.0_f64;
        assert!(result.approx_eq(expected, (0.0, 2)));
    }

    #[test]
    fn magnitude_should_return_valid_result() {
        // Arrange
        let vector: Vector = vector::create(5.0, 3.0);

        // Act
        let result = vector::magnitude(&vector);

        // Assert
        let expected: f64 = 5.830951894845301;
        assert!(result.approx_eq(expected, (0.0, 2)));
    }
}
