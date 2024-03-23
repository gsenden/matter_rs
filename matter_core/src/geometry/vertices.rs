use float_cmp::ApproxEq;
use std::cmp::Ordering;
use std::vec;

use super::super::core::common;
use super::vector::{self, Vector};
use regex::Regex;

pub fn create(points: Vec<Vector>) -> Vec<Vector> {
    let mut vertices: Vec<Vector> = Vec::new();
    for (index, vector) in points.iter().enumerate() {
        vertices.push(Vector {
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

pub fn from_path(path: &str) -> Result<Vec<Vector>, FromPathError> {
    let regex = match Regex::new(r"([\d.e]+)[\s,]*([-\d.e]+)") {
        Ok(reg) => reg,
        Err(error) => {
            let regex_error = format!(
                "Regex error while parsing {} to Vectores.\nError:{}",
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

pub fn translate(vertices: &mut Vec<Vector>, vector: &Vector, scalar: f64) {
    let translate_x = vector.x * scalar;
    let translate_y = vector.y * scalar;

    for vector in vertices {
        vector.x += translate_x;
        vector.y += translate_y;
    }
}

pub fn contains(vertices: &Vec<Vector>, point: &Vector) -> bool {
    let mut previous_vector = &vertices[vertices.len() - 1];
    for vector in vertices {
        if (point.x - previous_vector.x) * (vector.y - previous_vector.y)
            + (point.y - previous_vector.y) * (previous_vector.x - vector.x)
            > 0.0
        {
            return false;
        }
        previous_vector = vector
    }
    true
}

pub fn scale_centre(vertices: &mut Vec<Vector>, scale_x: f64, scale_y: f64) {
    let centre = centre(vertices);
    scale(vertices, scale_x, scale_y, &centre)
}

pub fn scale(vertices: &mut Vec<Vector>, scale_x: f64, scale_y: f64, point: &Vector) {
    if scale_x == 1.0 && scale_y == 1.0 {
        return;
    }

    for vector in vertices {
        let delta = vector::sub(vector, point);
        vector.x = point.x + delta.x * scale_x;
        vector.y = point.y + delta.y * scale_y;
    }
}

pub fn chamfer(
    vertices: &Vec<Vector>,
    radius: Option<Vec<f64>>,
    quality: Option<f64>,
    quality_min: Option<f64>,
    quality_max: Option<f64>,
) -> Vec<Vector> {
    let default_quality = -1.0;
    let radius = radius.unwrap_or(vec![8.0_f64]);
    let quality = quality.unwrap_or(default_quality);
    let quality_min = quality_min.unwrap_or(2.0_f64);
    let quality_max = quality_max.unwrap_or(104.0_f64);

    let mut new_vertices: Vec<Vector> = Vec::new();
    for (index, vertex) in vertices.iter().enumerate() {
        let prev_vertex = &vertices[if index > 0 {
            index - 1
        } else {
            vertices.len() - 1
        }];
        let next_vertex = &vertices[(index + 1) % vertices.len()];
        let current_radius = radius[if index < radius.len() {
            index
        } else {
            radius.len() - 1
        }];

        if current_radius == 0.0 {
            new_vertices.push(vertex.clone());
            continue;
        }

        let prev_normal = vector::normalise(&vector::create_vertex(
            vertex.y - prev_vertex.y,
            prev_vertex.x - vertex.x,
            index,
        ));
        let next_normal = vector::normalise(&vector::create_vertex(
            next_vertex.y - vertex.y,
            vertex.x - next_vertex.x,
            index,
        ));
        let diagonal_radius = f64::sqrt(2.0 * f64::powf(current_radius, 2.0));
        let radius_vector = vector::mult(&prev_normal, current_radius);
        let mid_normal = vector::mult(&vector::add(&prev_normal, &next_normal), 0.5);
        let mid_normal = vector::normalise(&mid_normal);
        let scaled_vertex = vector::sub(vertex, &vector::mult(&mid_normal, diagonal_radius));

        let mut precision = quality;
        if quality == default_quality {
            precision = f64::powf(current_radius, 0.32) * 1.75;
        }

        precision = common::clamp(precision, quality_min, quality_max);
        if precision % 2.0 == 1.0 {
            precision += 1.0;
        }

        let alpha = f64::acos(vector::dot(&prev_normal, &next_normal));
        let theta = alpha / precision;

        let mut index = 0.0_f64;
        while index < precision {
            new_vertices.push(vector::add(
                &vector::rotate(&radius_vector, theta * index),
                &scaled_vertex,
            ));
            index += 1.0;
        }
    }

    new_vertices
}

pub fn clockwise_sort(vertices: &mut Vec<Vector>) {
    let centre = mean(vertices);

    vertices.sort_by(|vector_a, vector_b: &Vector| {
        if vector::angle(&centre, vector_a).approx_eq(vector::angle(&centre, vector_b), (0.0, 2)) {
            Ordering::Equal
        } else if vector::angle(&centre, vector_a) - vector::angle(&centre, vector_b) < 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

pub fn hull(vertices: &mut Vec<Vector>) {
    vertices.sort_by(|vector_a, vector_b| {
        let delta_x = vector_a.x - vector_b.x;
        let compare_value = if delta_x.approx_eq(0.0, (0.0, 2)) {
            vector_a.y - vector_b.y
        } else {
            delta_x
        };
        if compare_value < 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut lower: Vec<Vector> = Vec::new();

    let mut index = 0;
    while index < vertices.len() {
        let vertex = &vertices[index];

        while lower.len() >= 2
            && vector::cross3(&lower[lower.len() - 2], &lower[lower.len() - 1], &vertex) <= 0.0
        {
            lower.pop();
        }
        lower.push(vertex.clone());
        index += 1;
    }

    let mut upper: Vec<Vector> = Vec::new();
    let mut index = (vertices.len() - 1) as i32;

    while index >= 0 {
        let vertex = &vertices[index as usize];

        while upper.len() >= 2
            && vector::cross3(&upper[upper.len() - 2], &upper[upper.len() - 1], &vertex) <= 0.0
        {
            upper.pop();
        }
        upper.push(vertex.clone());

        index -= 1;
    }
    upper.pop();
    lower.pop();

    upper.append(&mut lower);
    vertices.clear();

    vertices.append(&mut upper)
}

pub fn is_convex(vertices: &Vec<Vector>) -> Option<bool> {
    let vertices_len = vertices.len();

    if vertices_len < 3 {
        return None;
    }

    let mut flag = 0;
    let mut index = 0_usize;
    while index < vertices_len {
        let j = (index + 1) % vertices_len;
        let k = (index + 2) % vertices_len;
        let mut z = (vertices[j].x - vertices[index].x) * (vertices[k].y - vertices[j].y);
        z -= (vertices[j].y - vertices[index].y) * (vertices[k].x - vertices[j].x);
        index += 1;

        if z < 0.0 {
            flag = flag | 1;
        } else if z > 0.0 {
            flag = flag | 2;
        }

        if flag == 3 {
            return Some(false);
        }
    }

    if flag != 0 {
        Some(true)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::ApproxEq;

    use crate::geometry::vector;

    use super::*;

    #[test]
    fn is_convex_should_return_none_for_vertices_with_z_0() {
        //Arrange
        let point_a = vector::create(0.0, 0.0);
        let point_b = vector::create(0.0, 0.0);
        let point_c = vector::create(0.0, 0.0);
        let points = vec![point_a, point_b, point_c];
        let vertices = create(points);

        // Act
        let result = is_convex(&vertices);

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn is_convex_should_return_none_for_vertices_with_less_then_3_vectors() {
        //Arrange
        let point_a = vector::create(0.0, 1.0);
        let point_b = vector::create(1.0, 1.0);
        let points = vec![point_a, point_b];
        let vertices = create(points);

        // Act
        let result = is_convex(&vertices);

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn is_convex_should_return_false_for_non_convex_vertices() {
        //Arrange
        let points = test_shape_non_convex();
        let vertices = create(points);

        // Act
        let result = is_convex(&vertices);

        // Assert
        assert_eq!(result, Some(false));
    }

    #[test]
    fn is_convex_should_return_true_for_convex_vertices() {
        //Arrange
        let points = test_shape_convex();
        let vertices = create(points);

        // Act
        let result = is_convex(&vertices);

        // Assert
        assert_eq!(result, Some(true));
    }

    #[test]
    fn hull_should_mutate_the_vertices_to_a_valid_vec() {
        //Arrange
        let points = test_square();
        let mut vertices = create(points);

        // Act
        hull(&mut vertices);

        // Assert
        assert_vector(&vertices[0], 40.1, 40.1);
        assert_vector(&vertices[1], 0.0, 40.1);
        assert_vector(&vertices[2], 0.0, 0.0);
        assert_vector(&vertices[3], 40.1, 0.0);
    }

    #[test]
    fn clockwise_sort_should_mutate_the_vertices_to_a_valid_vec() {
        //Arrange
        let point_d = vector::create(0.0, 0.0);
        let point_c = vector::create(40.1, 0.0);
        let point_b = vector::create(40.1, 40.1);
        let point_a = vector::create(0.0, 40.1);

        let points = vec![point_a, point_b, point_c, point_d];

        let mut vertices = create(points);

        // Act
        clockwise_sort(&mut vertices);

        // Assert

        assert_vector(&vertices[0], 0.0, 0.0);
        assert_vector(&vertices[1], 40.1, 0.0);
        assert_vector(&vertices[2], 40.1, 40.1);
        assert_vector(&vertices[3], 0.0, 40.1);
    }

    #[test]
    fn chamfer_should_create_valid_vertices_when_using_default_parameters() {
        //Arrange
        let points = test_square();
        let vertices = create(points);
        let radius = None;
        let quality = None;
        let quality_min = None;
        let quality_max = None;

        // Act
        let result = chamfer(&vertices, radius, quality, quality_min, quality_max);

        // Assert
        assert_eq!(result.len(), 16);
        assert_vector(&result[0], 0.0, 8.0);
        assert_vector(&result[1], 0.8366176963389194, 4.438265314261145);
        assert_vector(&result[2], 3.171488492898817, 1.6214831954606046);
        assert_vector(&result[3], 6.516263832912953, 0.13879608542827349);
        assert_vector(&result[4], 32.1, 0.0);
        assert_vector(&result[5], 35.66173468573886, 0.8366176963389194);
        assert_vector(&result[6], 38.478516804539396, 3.171488492898817);
        assert_vector(&result[7], 39.961203914571726, 6.516263832912953);
        assert_vector(&result[8], 40.1, 32.1);
        assert_vector(&result[9], 39.26338230366108, 35.66173468573886);
        assert_vector(&result[10], 36.928511507101184, 38.478516804539396);
        assert_vector(&result[11], 33.58373616708705, 39.961203914571726);
        assert_vector(&result[12], 8.0, 40.1);
        assert_vector(&result[13], 4.438265314261145, 39.26338230366108);
        assert_vector(&result[14], 1.6214831954606046, 36.928511507101184);
        assert_vector(&result[15], 0.13879608542827349, 33.58373616708705);
    }

    #[test]
    fn chamfer_should_create_valid_vertices_when_not_using_default_parameters() {
        //Arrange
        let points = test_square();
        let vertices = create(points);
        let radius = Some(vec![2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64]);
        let quality = Some(-3.0_f64);
        let quality_min = Some(8.0_f64);
        let quality_max = Some(12.0_f64);

        // Act
        let result = chamfer(&vertices, radius, quality, quality_min, quality_max);

        // Assert
        assert_vector(&result[0], 0.0, 2.0);
        assert_vector(&result[1], 0.03842943919353914, 1.6098193559677436);
        assert_vector(&result[2], 0.15224093497742652, 1.2346331352698203);
        assert_vector(&result[3], 0.33706077539490953, 0.8888595339607956);
        assert_vector(&result[4], 0.5857864376269049, 0.5857864376269051);
        assert_vector(&result[5], 0.8888595339607954, 0.33706077539490953);
        assert_vector(&result[6], 1.2346331352698203, 0.15224093497742652);
        assert_vector(&result[7], 1.6098193559677434, 0.03842943919353914);
        assert_vector(&result[8], 37.1, -4.440892098500626e-16);
        assert_vector(&result[9], 37.685270966048385, 0.057644158790308264);
        assert_vector(&result[10], 38.24805029709527, 0.22836140246613956);
        assert_vector(&result[11], 38.766710699058805, 0.505591163092364);
        assert_vector(&result[12], 39.22132034355965, 0.8786796564403567);
        assert_vector(&result[13], 39.59440883690764, 1.3332893009411926);
        assert_vector(&result[14], 39.87163859753386, 1.85194970290473);
        assert_vector(&result[15], 40.04235584120969, 2.4147290339516143);
        assert_vector(&result[16], 40.1, 36.1);
        assert_vector(&result[17], 40.02314112161292, 36.88036128806451);
        assert_vector(&result[18], 39.795518130045146, 37.63073372946036);
        assert_vector(&result[19], 39.42587844921018, 38.322280932078414);
        assert_vector(&result[20], 38.92842712474619, 38.92842712474619);
        assert_vector(&result[21], 38.322280932078414, 39.42587844921018);
        assert_vector(&result[22], 37.63073372946036, 39.795518130045146);
        assert_vector(&result[23], 36.88036128806451, 40.02314112161292);
        assert_vector(&result[24], 5.0, 40.1);
        assert_vector(&result[25], 4.024548389919358, 40.003926402016155);
        assert_vector(&result[26], 3.086582838174551, 39.71939766255643);
        assert_vector(&result[27], 2.222148834901989, 39.25734806151273);
        assert_vector(&result[28], 1.4644660940672627, 38.63553390593274);
        assert_vector(&result[29], 0.8426519384872737, 37.877851165098015);
        assert_vector(&result[30], 0.3806023374435661, 37.01341716182545);
        assert_vector(&result[31], 0.09607359798384785, 36.07545161008064);
    }

    #[test]
    fn scale_centre_should_mutate_the_vertices_to_valid_values() {
        //Arrange
        let points = test_square();
        let mut vertices = create(points);
        let scale_x = 5.0_f64;
        let scale_y = 8.0_f64;

        // Act
        scale_centre(&mut vertices, scale_x, scale_y);

        // Assert
        assert_vector(&vertices[0], -80.2, -140.35);
        assert_vector(&vertices[1], 120.3, -140.35);
        assert_vector(&vertices[2], 120.3, 180.45000000000002);
        assert_vector(&vertices[3], -80.2, 180.45000000000002);
    }

    #[test]
    fn scale_should_mutate_the_vertices_to_valid_values() {
        //Arrange
        let points = test_square();
        let mut vertices = create(points);
        let scale_x = 5.0_f64;
        let scale_y = 8.0_f64;
        let point = vector::create(0.0, 0.0);

        // Act
        scale(&mut vertices, scale_x, scale_y, &point);

        // Assert
        assert_vector(&vertices[0], 0.0, 0.0);
        assert_vector(&vertices[1], 200.5, 0.0);
        assert_vector(&vertices[2], 200.5, 320.8);
        assert_vector(&vertices[3], 0.0, 320.8);
    }

    #[test]
    fn contains_should_respond_false_when_the_vector_is_outside() {
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
    fn contains_should_respond_true_when_the_vector_is_on_the_edge() {
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
    fn contains_should_respond_true_when_the_vector_is_in_the_middle() {
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
        assert_vector(&vertices[0], 15.0, 18.0);
        assert_vector(&vertices[1], 55.1, 18.0);
        assert_vector(&vertices[2], 55.1, 58.1);
        assert_vector(&vertices[3], 15.0, 58.1);
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
        assert_vector(&result, 20.05, 20.05);
    }

    #[test]
    fn centre_should_return_a_valid_vector() {
        // Arrange
        let points = test_square();

        // Act
        let result: Vector = centre(&points);

        // Assert
        assert_vector(&result, 20.05, 20.05);
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
    fn from_path_should_produce_a_valid_list_of_vectors() {
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
    fn create_should_produce_a_valid_list_of_vectors() {
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

    fn assert_vector(result: &Vector, expected_x: f64, expected_y: f64) {
        assert_float(result.x, expected_x);
        assert_float(result.y, expected_y);
    }

    fn assert_float(result: f64, expected: f64) {
        assert!(
            result.approx_eq(expected, (0.0, 2)),
            "result: {} did not match expected: {}",
            result,
            expected
        );
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

    fn test_shape_convex() -> Vec<Vector> {
        let point_a = vector::create(40.1, 40.1);
        let point_b = vector::create(0.0, 40.1);
        let point_c = vector::create(0.0, 0.0);
        let point_d = vector::create(40.1, 0.0);
        vec![point_a, point_b, point_c, point_d]
    }

    fn test_shape_non_convex() -> Vec<Vector> {
        let point_a = vector::create(1.0, 1.0);
        let point_b = vector::create(5.0, 1.0);
        let point_c = vector::create(5.0, 3.0);
        let point_d = vector::create(4.0, 4.0);
        let point_e = vector::create(3.0, 3.0);
        let point_f = vector::create(2.0, 4.0);
        let point_g = vector::create(1.0, 3.0);

        vec![
            point_a, point_b, point_c, point_d, point_e, point_f, point_g,
        ]
    }
}
