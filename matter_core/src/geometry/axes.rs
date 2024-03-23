// #[cfg(test)]
// mod tests {
//     use float_cmp::ApproxEq;

//     use crate::geometry::vector;
//     use crate::geometry::vector::Vector;
//     use crate::geometry::vertices;

//     use super::*;

//     #[test]
//     fn from_vertices_should_return_valid_vectors_as_axes() {
//         //Arrange
//         let points = test_square();
//         let vertices = vertices::create(points);

//         // Act
//         let result = from_vertices(&vertices);

//         // Assert
//         assert_vector(&result[0], 0.0, 1.0);
//         assert_vector(&result[1], -1.0, 0.0);
//     }

//     fn assert_vector(result: &Vector, expected_x: f64, expected_y: f64) {
//         assert_float(result.x, expected_x);
//         assert_float(result.y, expected_y);
//     }

//     fn assert_float(result: f64, expected: f64) {
//         assert!(
//             result.approx_eq(expected, (0.0, 2)),
//             "result: {} did not match expected: {}",
//             result,
//             expected
//         );
//     }

//     fn test_square() -> Vec<Vector> {
//         let point_a = vector::create(1.0, 1.0);
//         let point_b = vector::create(3.0, 1.0);
//         let point_c = vector::create(3.0, 3.0);
//         let point_d = vector::create(1.0, 3.0);
//         vec![point_a, point_b, point_c, point_d]
//     }
// }
