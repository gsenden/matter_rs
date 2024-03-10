pub struct Vector {
  pub x: f64,
  pub y: f64
}

impl Vector {
  pub fn create(x: f64, y: f64) -> Self {
    Self {
      x: x,
      y: y
    }
  }

  pub fn magnitude(&self) -> f64 {
    self.magnitude_squared().sqrt()
  }
  
  fn magnitude_squared(&self) -> f64 {
    self.x.powi(2) + self.y.powi(2)
  }
  
  fn rotate(&mut self, angle: f64) {
    let cos = angle.cos();
    let sin = angle.sin();
    let x = self.x * cos - self.y * sin;
    self.y = self.x * sin + self.y * cos;
    self.x = x;
  }
}
  

#[cfg(test)]
mod vector_tests {
  use crate::geometry::vector::Vector;
  use float_cmp::{self, ApproxEq};

  #[test]
  fn magnitude_should_return_valid_result() {
    // Arrange
    let subject: Vector = Vector::create(5.0, 3.0);

    // Act
    let result = subject.magnitude();    

    // Assert
    let expected: f64 = 5.830951894845301;
    assert!(result.approx_eq(expected, (0.0, 2)));
  }

  #[test]
  fn magnitude_should_be_able_to_deal_with_zero() {
    // Arrange
    let subject: Vector = Vector::create(0.0, 0.0);

    // Act
    let result = subject.magnitude();    

    // Assert
    let expected = 0.0_f64;
    assert!(result.approx_eq(expected, (0.0, 2)));
  }

  #[test]
  fn magnitude_squared_should_return_valid_result() {
    // Arrange
    let subject: Vector = Vector::create(10.0, 2.0);

    // Act
    let result = subject.magnitude_squared();    

    // Assert
    let expected = 104.0_f64;
    assert!(result.approx_eq(expected, (0.0, 2)));
  }

  #[test]
  fn rotate_should_mutate_to_valid_result() {
    // Arrange
    let mut subject: Vector = Vector::create(10.0, 2.0);
    let angle = -2_f64;

    // Act
    subject.rotate(angle);    

    // Assert
    let expected_x = -2.3428735118200605_f64;
    let expected_y = -9.925267941351102_f64;
    assert!(subject.x.approx_eq(expected_x, (0.0, 2)));
    assert!(subject.y.approx_eq(expected_y, (0.0, 2)));
  }

}
