use float_cmp::ApproxEq;
use std::cmp::Ordering;
use std::vec;
use uuid::Uuid;

use crate::body_mod::body::Body;
use crate::core::xy::XY;

use super::super::core::common;
use super::vector::{self, Vector};
use regex::Regex;

#[derive(Clone)]
pub struct Vertex {
    body: Option<Body>,
    x: f64,
    y: f64,
    index: usize,
    is_internal: bool,
}

impl XY for Vertex {
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

impl Vertex {
    pub fn from_vector(body: Option<Body>, vector: &Vector, index: usize, is_internal: bool) -> Self {
        Vertex::new(body, vector.get_x(), vector.get_y(), index, is_internal)
    }

    pub fn new(body: Option<Body>, x: f64, y: f64, index: usize, is_internal: bool) -> Self {
        Vertex {
            body: body,
            x: x,
            y: y,
            index: index,
            is_internal: is_internal,
        }
    }

    pub fn get_body(&self) -> Option<Body> {
        self.body.clone()
    }

    pub fn set_body(&mut self, body: Body) {
        self.body = Some(body);
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_is_internal(&self) -> bool {
        self.is_internal
    }

    pub fn set_is_interal(&mut self, is_internal: bool) {
        self.is_internal = is_internal
    }
}

pub fn create(points: Vec<Vector>, body: Option<Body>) -> Vec<Vertex> {
    let mut vertices: Vec<Vertex> = Vec::new();
    for (index, vector) in points.iter().enumerate() {
        vertices.push(Vertex::from_vector(body.clone(), vector, index, false));
    }
    vertices
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromPathError {
    RegexError(String),
    ParseFloatError(String),
}

pub fn set_body(vertices: &mut Vec<Vertex>, body: &Body) {
    for vertex in vertices.iter_mut() {
        vertex.body = Some(body.clone());
    }
}

pub fn from_path(path: &str, body: Option<Body>) -> Result<Vec<Vertex>, FromPathError> {
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

    let mut index = 0;
    let vertices: Result<Vec<Vertex>, FromPathError> = regex
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
            index += 1;
            Ok(Vertex::new(body.clone(), x, y, index - 1, false))
        })
        .collect();

    match vertices {
        Ok(points) => Ok(points),
        Err(e) => Err(e),
    }
}

pub fn area(vertices: &Vec<Vertex>, signed: bool) -> f64 {
    let mut index2 = vertices.len() - 1;
    let mut area = 0.0_f64;

    for (index, vertex) in vertices.iter().enumerate() {
        area += (vertices[index2].x - vertex.x) * (vertices[index2].y + vertex.y);
        index2 = index;
    }

    if signed {
        area / 2.0
    } else {
        area.abs() / 2.0
    }
}

pub fn centre(vertices: &Vec<Vertex>) -> Vector {
    let area = area(&vertices, true);
    let mut centre = vector::create(0.0, 0.0);
    for (index, vertex) in vertices.iter().enumerate() {
        let index2 = (index + 1) % vertices.len();
        let cross = vector::cross(vertex, &vertices[index2]);
        let temp = vector::mult(&vector::add(vertex, &vertices[index2]), cross);
        centre = vector::add(&centre, &temp);
    }
    vector::div(&centre, 6.0 * area)
}

pub fn mean(vertices: &Vec<Vertex>) -> Vector {
    let average = vertices
        .iter()
        .fold(vector::create(0., 0.), |mut cur, acc| {
            cur.set_x(cur.get_x() + acc.get_x());
            cur.set_y(cur.get_y() + acc.get_y());
            cur
        });
    let scalar = vertices.len() as f64;
    vector::div(&average, scalar)
}

pub fn innertia(vertices: &Vec<Vertex>, mass: f64) -> f64 {
    let mut numerator: f64 = 0.0;
    let mut denominator: f64 = 0.0;

    for (index, vertex) in vertices.iter().enumerate() {
        let index2 = (index + 1) % vertices.len();
        let vertex2 = vertices[index2].clone();
        let cross = f64::abs(vector::cross(&vertex2, vertex));
        numerator += cross
            * (vector::dot(&vertex2, &vertex2)
                + vector::dot(&vertex2, vertex)
                + vector::dot(vertex, vertex));
        denominator += cross;
    }
    (mass / 6.0) * (numerator / denominator)
}

pub fn translate(vertices: &mut Vec<Vertex>, vector: &impl XY, scalar: Option<f64>) {
    let scalar = if scalar.is_some() {
        scalar.unwrap()
    } else {
        1.
    };

    let translate_x = vector.get_x() * scalar;
    let translate_y = vector.get_y() * scalar;

    for vector in vertices {
        vector.x += translate_x;
        vector.y += translate_y;
    }
}

pub fn contains(vertices: &Vec<Vertex>, point: &impl XY) -> bool {
    let mut previous_vector = &vertices[vertices.len() - 1];
    for vertex in vertices {
        if (point.get_x() - previous_vector.x) * (vertex.y - previous_vector.y)
            + (point.get_y() - previous_vector.y) * (previous_vector.x - vertex.x)
            > 0.0
        {
            return false;
        }
        previous_vector = vertex
    }
    true
}

pub fn scale(vertices: &mut Vec<Vertex>, scale_x: f64, scale_y: f64, point: Option<&Vector>) {
    let point = match point {
        Some(point) => *point,
        None => centre(vertices),
    };

    if scale_x == 1.0 && scale_y == 1.0 {
        return;
    }

    for vertex in vertices {
        let delta = vector::sub(vertex, &point);
        vertex.x = point.get_x() + delta.get_x() * scale_x;
        vertex.y = point.get_y() + delta.get_y() * scale_y;
    }
}

pub fn chamfer(
    vertices: &Vec<Vertex>,
    radius: Option<Vec<f64>>,
    quality: Option<f64>,
    quality_min: Option<f64>,
    quality_max: Option<f64>,
) -> Vec<Vertex> {
    let default_quality = -1.0;
    let radius = radius.unwrap_or(vec![8.0_f64]);
    let quality = quality.unwrap_or(default_quality);
    let quality_min = quality_min.unwrap_or(2.0_f64);
    let quality_max = quality_max.unwrap_or(104.0_f64);

    let mut new_vertices: Vec<Vertex> = Vec::new();
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

        let prev_normal = vector::normalise(&vector::create(
            vertex.y - prev_vertex.y,
            prev_vertex.x - vertex.x,
        ));
        let next_normal = vector::normalise(&vector::create(
            next_vertex.y - vertex.y,
            vertex.x - next_vertex.x,
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

        let mut index = 0_usize;
        while (index as f64) < precision {
            let vector = vector::add(
                &vector::rotate(&radius_vector, theta * index as f64),
                &scaled_vertex,
            );
            new_vertices.push(Vertex::from_vector(
                vertex.body.clone(),
                &vector,
                index,
                vertex.is_internal,
            ));
            index += 1;
        }
    }

    new_vertices
}

pub fn clockwise_sort(vertices: &mut Vec<Vertex>) {
    let centre = mean(vertices);

    vertices.sort_by(|vector_a: &Vertex, vector_b: &Vertex| {
        if vector::angle(&centre, vector_a).approx_eq(vector::angle(&centre, vector_b), (0.0, 2)) {
            Ordering::Equal
        } else if vector::angle(&centre, vector_a) - vector::angle(&centre, vector_b) < 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

pub fn hull(vertices: &mut Vec<Vertex>) {
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

    let mut lower: Vec<Vertex> = Vec::new();

    let mut index = 0;
    while index < vertices.len() {
        let vertex = &vertices[index];

        while lower.len() >= 2
            && vector::cross3(&lower[lower.len() - 2], &lower[lower.len() - 1], vertex) <= 0.0
        {
            lower.pop();
        }
        lower.push(vertex.clone());
        index += 1;
    }

    let mut upper: Vec<Vertex> = Vec::new();
    let mut index = (vertices.len() - 1) as i32;

    while index >= 0 {
        let vertex = &vertices[index as usize];

        while upper.len() >= 2
            && vector::cross3(&upper[upper.len() - 2], &upper[upper.len() - 1], vertex) <= 0.0
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

pub fn is_convex(vertices: &Vec<Vertex>) -> Option<bool> {
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
    use crate::{
        geometry::vector,
        test_utils::{
            common_test_utils::assert_float,
            geometry_test_utils::{
                assert_xy, test_shape_convex, test_shape_non_convex, test_square_with_decimals,
                test_square_with_decimals_signed, vec_vector_to_vec_vertex,
            },
        },
    };

    use super::*;

    #[test]
    fn is_convex_should_return_none_for_vertices_with_z_0() {
        //Arrange
        let point_a = vector::create(0.0, 0.0);
        let point_b = vector::create(0.0, 0.0);
        let point_c = vector::create(0.0, 0.0);
        let points = vec![point_a, point_b, point_c];
        let vertices = create(points, None);

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
        let vertices = create(points, None);

        // Act
        let result = is_convex(&vertices);

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn is_convex_should_return_false_for_non_convex_vertices() {
        //Arrange
        let points = test_shape_non_convex();
        let vertices = create(points, None);

        // Act
        let result = is_convex(&vertices);

        // Assert
        assert_eq!(result, Some(false));
    }

    #[test]
    fn is_convex_should_return_true_for_convex_vertices() {
        //Arrange
        let points = test_shape_convex();
        let vertices = create(points, None);

        // Act
        let result = is_convex(&vertices);

        // Assert
        assert_eq!(result, Some(true));
    }

    #[test]
    fn hull_should_mutate_the_vertices_to_a_valid_vec() {
        //Arrange
        let points = test_square_with_decimals();
        let mut vertices = create(points, None);

        // Act
        hull(&mut vertices);

        // Assert
        assert_xy(&vertices[0], 40.1, 40.1);
        assert_xy(&vertices[1], 0.0, 40.1);
        assert_xy(&vertices[2], 0.0, 0.0);
        assert_xy(&vertices[3], 40.1, 0.0);
    }

    #[test]
    fn clockwise_sort_should_mutate_the_vertices_to_a_valid_vec() {
        //Arrange
        let point_d = vector::create(0.0, 0.0);
        let point_c = vector::create(40.1, 0.0);
        let point_b = vector::create(40.1, 40.1);
        let point_a = vector::create(0.0, 40.1);

        let points = vec![point_a, point_b, point_c, point_d];

        let mut vertices = create(points, None);

        // Act
        clockwise_sort(&mut vertices);

        // Assert

        assert_xy(&vertices[0], 0.0, 0.0);
        assert_xy(&vertices[1], 40.1, 0.0);
        assert_xy(&vertices[2], 40.1, 40.1);
        assert_xy(&vertices[3], 0.0, 40.1);
    }

    #[test]
    fn chamfer_should_create_valid_vertices_when_using_default_parameters() {
        //Arrange
        let points = test_square_with_decimals();
        let vertices = create(points, None);
        let radius = None;
        let quality = None;
        let quality_min = None;
        let quality_max = None;

        // Act
        let result = chamfer(&vertices, radius, quality, quality_min, quality_max);

        // Assert
        assert_eq!(result.len(), 16);
        assert_xy(&result[0], 0.0, 8.0);
        assert_xy(&result[1], 0.8366176963389194, 4.438265314261145);
        assert_xy(&result[2], 3.171488492898817, 1.6214831954606046);
        assert_xy(&result[3], 6.516263832912953, 0.13879608542827349);
        assert_xy(&result[4], 32.1, 0.0);
        assert_xy(&result[5], 35.66173468573886, 0.8366176963389194);
        assert_xy(&result[6], 38.478516804539396, 3.171488492898817);
        assert_xy(&result[7], 39.961203914571726, 6.516263832912953);
        assert_xy(&result[8], 40.1, 32.1);
        assert_xy(&result[9], 39.26338230366108, 35.66173468573886);
        assert_xy(&result[10], 36.928511507101184, 38.478516804539396);
        assert_xy(&result[11], 33.58373616708705, 39.961203914571726);
        assert_xy(&result[12], 8.0, 40.1);
        assert_xy(&result[13], 4.438265314261145, 39.26338230366108);
        assert_xy(&result[14], 1.6214831954606046, 36.928511507101184);
        assert_xy(&result[15], 0.13879608542827349, 33.58373616708705);
    }

    #[test]
    fn chamfer_should_create_valid_vertices_when_not_using_default_parameters() {
        //Arrange
        let points = test_square_with_decimals();
        let vertices = create(points, None);
        let radius = Some(vec![2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64]);
        let quality = Some(-3.0_f64);
        let quality_min = Some(8.0_f64);
        let quality_max = Some(12.0_f64);

        // Act
        let result = chamfer(&vertices, radius, quality, quality_min, quality_max);

        // Assert
        assert_xy(&result[0], 0.0, 2.0);
        assert_xy(&result[1], 0.03842943919353914, 1.6098193559677436);
        assert_xy(&result[2], 0.15224093497742652, 1.2346331352698203);
        assert_xy(&result[3], 0.33706077539490953, 0.8888595339607956);
        assert_xy(&result[4], 0.5857864376269049, 0.5857864376269051);
        assert_xy(&result[5], 0.8888595339607954, 0.33706077539490953);
        assert_xy(&result[6], 1.2346331352698203, 0.15224093497742652);
        assert_xy(&result[7], 1.6098193559677434, 0.03842943919353914);
        assert_xy(&result[8], 37.1, -4.440892098500626e-16);
        assert_xy(&result[9], 37.685270966048385, 0.057644158790308264);
        assert_xy(&result[10], 38.24805029709527, 0.22836140246613956);
        assert_xy(&result[11], 38.766710699058805, 0.505591163092364);
        assert_xy(&result[12], 39.22132034355965, 0.8786796564403567);
        assert_xy(&result[13], 39.59440883690764, 1.3332893009411926);
        assert_xy(&result[14], 39.87163859753386, 1.85194970290473);
        assert_xy(&result[15], 40.04235584120969, 2.4147290339516143);
        assert_xy(&result[16], 40.1, 36.1);
        assert_xy(&result[17], 40.02314112161292, 36.88036128806451);
        assert_xy(&result[18], 39.795518130045146, 37.63073372946036);
        assert_xy(&result[19], 39.42587844921018, 38.322280932078414);
        assert_xy(&result[20], 38.92842712474619, 38.92842712474619);
        assert_xy(&result[21], 38.322280932078414, 39.42587844921018);
        assert_xy(&result[22], 37.63073372946036, 39.795518130045146);
        assert_xy(&result[23], 36.88036128806451, 40.02314112161292);
        assert_xy(&result[24], 5.0, 40.1);
        assert_xy(&result[25], 4.024548389919358, 40.003926402016155);
        assert_xy(&result[26], 3.086582838174551, 39.71939766255643);
        assert_xy(&result[27], 2.222148834901989, 39.25734806151273);
        assert_xy(&result[28], 1.4644660940672627, 38.63553390593274);
        assert_xy(&result[29], 0.8426519384872737, 37.877851165098015);
        assert_xy(&result[30], 0.3806023374435661, 37.01341716182545);
        assert_xy(&result[31], 0.09607359798384785, 36.07545161008064);
    }

    #[test]
    fn scale_should_mutate_the_vertices_to_valid_values() {
        //Arrange
        let points = test_square_with_decimals();
        let mut vertices = create(points, None);
        let scale_x = 5.0_f64;
        let scale_y = 8.0_f64;
        let point = vector::create(0.0, 0.0);

        // Act
        scale(&mut vertices, scale_x, scale_y, Some(&point));

        // Assert
        assert_xy(&vertices[0], 0.0, 0.0);
        assert_xy(&vertices[1], 200.5, 0.0);
        assert_xy(&vertices[2], 200.5, 320.8);
        assert_xy(&vertices[3], 0.0, 320.8);
    }

    #[test]
    fn contains_should_respond_false_when_the_vector_is_outside() {
        // Arrange
        let points = test_square_with_decimals();
        let vertices = create(points, None);

        let vector = vector::create(-1.0, 0.0);

        // Act
        let result = contains(&vertices, &vector);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn contains_should_respond_true_when_the_vector_is_on_the_edge() {
        // Arrange
        let points = test_square_with_decimals();
        let vertices = create(points, None);

        let vector = vector::create(0.0, 0.0);

        // Act
        let result = contains(&vertices, &vector);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn contains_should_respond_true_when_the_vector_is_in_the_middle() {
        // Arrange
        let points = test_square_with_decimals();
        let vertices = create(points, None);

        let vector = vector::create(20.5, 20.5);

        // Act
        let result = contains(&vertices, &vector);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn translate_should_mutate_vertices_in_a_valid_way() {
        // Arrange
        let points = test_square_with_decimals();
        let mut vertices = create(points, None);

        let vector = vector::create(5.0, 6.0);
        let scalar = Some(3.0_f64);

        // Act
        translate(&mut vertices, &vector, scalar);

        // Assert
        assert_xy(&vertices[0], 15.0, 18.0);
        assert_xy(&vertices[1], 55.1, 18.0);
        assert_xy(&vertices[2], 55.1, 58.1);
        assert_xy(&vertices[3], 15.0, 58.1);
    }

    #[test]
    fn innertia_should_calculate_a_valid_value() {
        // Arrange
        let points = vec_vector_to_vec_vertex(test_square_with_decimals());
        let mass = 10.0;

        // Act
        let result = innertia(&points, mass);

        // Assert
        assert_float(result, 10720.06666666667);
    }

    #[test]
    fn mean_should_return_a_valid_vector() {
        // Arrange
        let points = vec_vector_to_vec_vertex(test_square_with_decimals());

        // Act
        let result: Vector = mean(&points);

        // Assert
        assert_xy(&result, 20.05, 20.05);
    }

    #[test]
    fn centre_should_return_a_valid_vector() {
        // Arrange
        let points = vec_vector_to_vec_vertex(test_square_with_decimals());

        // Act
        let result: Vector = centre(&points);

        // Assert
        assert_xy(&result, 20.05, 20.05);
    }

    #[test]
    fn area_should_calculate_a_valid_value_with_signed_false() {
        // Arrange
        let points = vec_vector_to_vec_vertex(test_square_with_decimals());
        //let vertices = create(points);
        let signed = false;

        // Act
        let result: f64 = area(&points, signed);

        // Assert
        assert_float(result, 1608.0100000000002_f64);
    }

    #[test]
    fn area_should_calculate_a_valid_value_with_signed_true() {
        // Arrange
        let points = vec_vector_to_vec_vertex(test_square_with_decimals_signed());
        let signed = true;

        // Act
        let result: f64 = area(&points, signed);

        // Assert
        assert_float(result, 1608.0100000000002_f64);
    }

    #[test]
    fn from_path_should_produce_a_valid_list_of_vectors() {
        // Arrange
        let path = "1 2 L 3, 4 L 5 6";

        // Act
        let result = from_path(path, None).unwrap();

        // Assert
        assert_xy(&result[0], 1.0, 2.0);
        assert_xy(&result[1], 3.0, 4.0);
        assert_xy(&result[2], 5.0, 6.0);

        assert_eq!(result.len(), 3_usize);
    }

    #[test]
    fn create_should_produce_a_valid_list_of_vectors() {
        // Arrange
        let vector_a = vector::create(1.0, 2.0);
        let vector_b = vector::create(3.0, 4.0);
        let vector_c = vector::create(5.0, 6.0);
        let vector_list = vec![vector_a, vector_b, vector_c];

        // Act
        let result = create(vector_list, None);

        // Assert
        assert_xy(&result[0], 1.0, 2.0);
        assert_xy(&result[1], 3.0, 4.0);
        assert_xy(&result[2], 5.0, 6.0);
        assert_eq!(result.len(), 3);
    }
}
