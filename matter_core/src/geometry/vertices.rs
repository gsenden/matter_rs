use std::vec;

use super::vector::{self, Vector};
use regex::Regex;

pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub index: usize,
    pub is_internal: bool,
}

pub fn create(points: Vec<Vector>) -> Vec<Vertex> {
    let mut vertices: Vec<Vertex> = Vec::new();
    for (index, vector) in points.iter().enumerate() {
        vertices.push(Vertex {
            x: vector.x,
            y: vector.y,
            index: index,
            is_internal: false,
        });
    }
    vertices
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromPathError {
    RegexError(String),
    ParseFloatError(String),
}

pub fn from_path(path: &str) -> Result<Vec<Vertex>, FromPathError> {
    let regex = match Regex::new(r"([\d.e]+)[\s,]*([-\d.e]+)") {
        Ok(reg) => reg,
        Err(error) => {
            let regex_error = format!(
                "Regex error while parsing {} to vertexes.\nError:{}",
                path, error
            );
            return Err(FromPathError::RegexError(regex_error));
        }
    };

    let vectors: Result<Vec<Vector>, FromPathError> = regex
        .captures_iter(path)
        .map(|caps| {
            let (_, [first_number, second_number]) = caps.extract();
            let parse_error = format!("Float parsing error while parsing path {}", path);
            let x = match first_number.parse::<f64>() {
                Ok(n) => n,
                Err(e) => {
                    let parse_error = format!("{}\nError:{}", parse_error, e);
                    return Err(FromPathError::ParseFloatError(parse_error));
                }
            };
            let y = match second_number.parse::<f64>() {
                Ok(n) => n,
                Err(e) => {
                    let parse_error = format!("{}\nError:{}", parse_error, e);
                    return Err(FromPathError::ParseFloatError(parse_error));
                }
            };
            Ok(vector::create(x, y))
        })
        .collect();

    match vectors {
        Ok(points) => Ok(create(points)),
        Err(e) => Err(e),
    }
}

pub fn area(vertices: &Vec<Vector>, signed: bool) -> f64 {
    let mut index2 = vertices.len() - 1;
    let mut area = 0.0_f64;

    for (index, vector) in vertices.iter().enumerate() {
        area += (vertices[index2].x - vector.x) * (vertices[index2].y + vector.y);
        index2 = index;
    }

    if signed {
        area / 2.0
    } else {
        area.abs() / 2.0
    }
}

pub fn centre(vertices: &Vec<Vector>) -> Vector {
    let area = area(&vertices, true);
    let mut centre = vector::create(0.0, 0.0);
    for (index, vector) in vertices.iter().enumerate() {
        let index2 = (index + 1) % vertices.len();
        let cross = vector::cross(vector, &vertices[index2]);
        let temp = vector::mult(&vector::add(vector, &vertices[index2]), cross);
        centre = vector::add(&centre, &temp);
    }
    vector::div(&centre, 6.0 * area)
}

pub fn mean(vertices: &Vec<Vector>) -> Vector {
    let mut average = vector::create(0.0, 0.0);
    for vector in vertices {
        average.x += vector.x;
        average.y += vector.y;
    }
    let scalar = vertices.len() as f64;
    vector::div(&average, scalar)
}

pub fn innertia(vertices: &Vec<Vector>, mass: f64) -> f64 {
    let mut numerator: f64 = 0.0;
    let mut denominator: f64 = 0.0;

    for (index, vector) in vertices.iter().enumerate() {
        let index2 = (index + 1) % vertices.len();
        let vector2 = &vertices[index2];
        let cross = f64::abs(vector::cross(&vector2, vector));
        numerator += cross
            * (vector::dot(&vector2, &vector2)
                + vector::dot(&vector2, vector)
                + vector::dot(vector, vector));
        denominator += cross;
    }
    (mass / 6.0) * (numerator / denominator)
}

pub fn translate(vertices: &mut Vec<Vertex>, vector: &Vector, scalar: f64) {
    let translate_x = vector.x * scalar;
    let translate_y = vector.y * scalar;

    for vertex in vertices {
        vertex.x += translate_x;
        vertex.y += translate_y;
    }
}

pub fn contains(vertices: &Vec<Vertex>, point: &Vector) -> bool {
    let mut previous_vertex = &vertices[vertices.len() - 1];
    for vertex in vertices {
        if (point.x - previous_vertex.x) * (vertex.y - previous_vertex.y)
            + (point.y - previous_vertex.y) * (previous_vertex.x - vertex.x)
            > 0.0
        {
            return false;
        }
        previous_vertex = vertex
    }
    true
}

// pub fn scala_centre (vertices: &mut Vec<Vertex>, scale_x: f64, scale_y: f64) {
//     centre(vertices)
// }

pub fn scale(vertices: &mut Vec<Vertex>, scale_x: f64, scale_y: f64, point: &Vector) {
    if scale_x == 1.0 && scale_y == 1.0 {
        return;
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::ApproxEq;

    use crate::geometry::vector;

    use super::*;

    #[test]
    fn scale_should_mutate_the_vertices_to_valid_values() {
        // Arrange
        // let points = test_square();
        // let mut vertices = create(points);
        // let scale_x = 5.0_f64;
        // let scale_y = 8.0_f64;
        // let point = vector::create(0.0, 0.0);

        // // Act
        // scale(&mut vertices, scale_x, scale_y, &point);

        // // Assert
        // assert_vertex(&vertices[0], 0.0, 0.0);
        // assert_vertex(&vertices[0], 200.5, 0.0);
        // assert_vertex(&vertices[0], 200.5, 320.8);
        // assert_vertex(&vertices[0], 0.0, 320.8);
    }

    #[test]
    fn contains_should_respond_false_when_the_vertex_is_outside() {
        // Arrange
        let points = test_square();
        let vertices = create(points);

        let vector = vector::create(-1.0, 0.0);

        // Act
        let result = contains(&vertices, &vector);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn contains_should_respond_true_when_the_vertex_is_on_the_edge() {
        // Arrange
        let points = test_square();
        let vertices = create(points);

        let vector = vector::create(0.0, 0.0);

        // Act
        let result = contains(&vertices, &vector);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn contains_should_respond_true_when_the_vertex_is_in_the_middle() {
        // Arrange
        let points = test_square();
        let vertices = create(points);

        let vector = vector::create(20.5, 20.5);

        // Act
        let result = contains(&vertices, &vector);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn translate_should_mutate_vertices_in_a_valid_way() {
        // Arrange
        let points = test_square();
        let mut vertices = create(points);

        let vector = vector::create(5.0, 6.0);
        let scalar = 3.0_f64;

        // Act
        translate(&mut vertices, &vector, scalar);

        // Assert
        assert_vertex(&vertices[0], 15.0, 18.0);
        assert_vertex(&vertices[1], 55.1, 18.0);
        assert_vertex(&vertices[2], 55.1, 58.1);
        assert_vertex(&vertices[3], 15.0, 58.1);
    }

    #[test]
    fn innertia_should_calculate_a_valid_value() {
        // Arrange
        let points = test_square();
        let mass = 10.0;

        // Act
        let result = innertia(&points, mass);

        // Assert
        assert_float(result, 10720.06666666667);
    }

    #[test]
    fn mean_should_return_a_valid_vector() {
        // Arrange
        let points = test_square();

        // Act
        let result: Vector = mean(&points);

        // Assert
        assert_vector(result, 20.05, 20.05);
    }

    #[test]
    fn centre_should_return_a_valid_vector() {
        // Arrange
        let points = test_square();

        // Act
        let result: Vector = centre(&points);

        // Assert
        assert_vector(result, 20.05, 20.05);
    }

    #[test]
    fn area_should_calculate_a_valid_value_with_signed_false() {
        // Arrange
        let points = test_square();
        //let vertices = create(points);
        let signed = false;

        // Act
        let result: f64 = area(&points, signed);

        // Assert
        let expected = 1608.0100000000002_f64;

        assert!(result.approx_eq(expected, (0.0, 2)));
    }

    #[test]
    fn area_should_calculate_a_valid_value_with_signed_true() {
        // Arrange
        let points = test_square_signed();
        let signed = true;

        // Act
        let result: f64 = area(&points, signed);

        // Assert
        let expected = 1608.0100000000002_f64;

        assert!(result.approx_eq(expected, (0.0, 2)));
    }

    #[test]
    fn from_path_should_produce_a_valid_list_of_vertexes() {
        // Arrange
        let path = "1 2 L 3, 4 L 5 6";

        // Act
        let result = from_path(path).unwrap();

        // Assert
        let expected_x = 3.0_f64;
        let expected_len = 3_usize;
        let expected_index = 2_usize;

        assert!(result[1].x.approx_eq(expected_x, (0.0, 2)));
        assert_eq!(result.len(), expected_len);
        assert_eq!(result[2].index, expected_index);
    }

    #[test]
    fn create_should_produce_a_valid_list_of_vertexes() {
        // Arrange
        let vector_a = vector::create(1.0, 2.0);
        let vector_b = vector::create(3.0, 4.0);
        let vector_c = vector::create(5.0, 6.0);
        let vector_list = vec![vector_a, vector_b, vector_c];

        // Act
        let result = create(vector_list);

        // Assert
        let expected_x = 3.0_f64;
        let expected_len = 3_usize;
        let expected_index = 2_usize;
        assert!(result[1].x.approx_eq(expected_x, (0.0, 2)));
        assert_eq!(result.len(), expected_len);
        assert_eq!(result[2].index, expected_index);
    }

    fn assert_vertex(result: &Vertex, expected_x: f64, expected_y: f64) {
        assert_float(result.x, expected_x);
        assert_float(result.y, expected_y);
    }

    fn assert_vector(result: Vector, expected_x: f64, expected_y: f64) {
        assert_float(result.x, expected_x);
        assert_float(result.y, expected_y);
    }

    fn assert_float(result: f64, expected: f64) {
        assert!(result.approx_eq(expected, (0.0, 2)));
    }

    fn test_square() -> Vec<Vector> {
        let point_a = vector::create(0.0, 0.0);
        let point_b = vector::create(40.1, 0.0);
        let point_c = vector::create(40.1, 40.1);
        let point_d = vector::create(0.0, 40.1);
        vec![point_a, point_b, point_c, point_d]
    }

    fn test_square_signed() -> Vec<Vector> {
        let point_a = vector::create(0.0, 0.0);
        let point_b = vector::create(-40.1, 0.0);
        let point_c = vector::create(-40.1, -40.1);
        let point_d = vector::create(0.0, -40.1);
        vec![point_a, point_b, point_c, point_d]
    }
}
