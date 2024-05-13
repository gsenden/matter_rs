use float_cmp::ApproxEq;
use std::cmp::Ordering;
use std::collections::btree_map::Values;
use std::rc::Weak;
use std::slice::Iter;
use std::slice::IterMut;
use std::vec;
use uuid::Uuid;

use crate::body::body::Body;
use crate::core::xy::{XYNew, XY};

use super::super::core::common;
use super::vector::{self, Vector};
use super::vertex::Vertex;
use regex::Regex;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromPathError {
    RegexError(String),
    ParseFloatError(String),
}

#[derive(Clone)]
pub struct Vertices {
    value: Vec<Vertex>,
}

impl Index<usize> for Vertices {
    type Output = Vertex;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for Vertices {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        &mut self.value[index]
    }
}

impl Vertices {
    pub fn iter(&self) -> Iter<Vertex> {
        self.value.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Vertex> {
        self.value.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn get_value(&self) -> Vec<Vertex> {
        self.value.clone()
    }

    pub fn append(&mut self, values: &Vertices) {
        let mut values = values.clone();
        self.value.append(&mut values.value);
    }

    pub fn new(points: Vec<Vector>, body: Option<Body>) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();

        for (index, vector) in points.iter().enumerate() {
            vertices.push(Vertex::from_xy(body.clone(), vector, index, false));
        }
        Vertices { value: vertices }
    }

    pub fn create(points: Vec<Vector>, body: Option<Body>) -> Self {
        Vertices::new(points, body)
    }

    pub fn set_body(&mut self, body: &Body) {
        for vertex in self.value.iter_mut() {
            vertex.set_body(body.clone());
        }
    }

    pub fn from_path(path: &str, body: Option<Body>) -> Result<Vertices, FromPathError> {
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
            Ok(points) => Ok(Vertices { value: points }),
            Err(e) => Err(e),
        }
    }

    pub fn area(&self, signed: Option<bool>) -> f64 {
        let signed = signed.unwrap_or(false);
        let mut index2 = self.value.len() - 1;
        let mut area = 0.0_f64;

        for (index, vertex) in self.value.iter().enumerate() {
            area += (self.value[index2].get_x() - vertex.get_x())
                * (self.value[index2].get_y() + vertex.get_x());
            index2 = index;
        }

        if signed {
            area / 2.0
        } else {
            area.abs() / 2.0
        }
    }

    pub fn centre(&self) -> Vector {
        let area = self.area(Some(true));
        let mut centre = Vector::new(0., 0.);
        for (index, vertex) in self.value.iter().enumerate() {
            let index2 = (index + 1) % self.value.len();

            let cross = Vector::cross(vertex, &self.value[index2]);
            let mut temp = Vector::add(vertex, &self.value[index2]);
            temp.mult(cross);

            centre = Vector::add(&centre, &temp);
        }
        centre.div(6. * area);
        centre
    }

    pub fn mean(&self) -> Vector {
        let mut average = self.value.iter().fold(Vector::new(0., 0.), |mut cur, acc| {
            cur.set_x(cur.get_x() + acc.get_x());
            cur.set_y(cur.get_y() + acc.get_y());
            cur
        });
        let scalar = self.value.len() as f64;
        average.div(scalar);
        average
    }

    pub fn innertia(&self, mass: f64) -> f64 {
        let mut numerator: f64 = 0.;
        let mut denominator: f64 = 0.;

        for (index, vertex) in self.value.iter().enumerate() {
            let index2 = (index + 1) % self.value.len();
            let vertex2 = self.value[index2].clone();
            let cross = f64::abs(Vector::cross(&vertex2, vertex));

            numerator += cross * (vertex2.dot(&vertex2) + vertex2.dot(vertex) + vertex.dot(vertex));
            denominator += cross;
        }
        (mass / 6.0) * (numerator / denominator)
    }

    pub fn translate(&mut self, point: &impl XY, scalar: Option<f64>) {
        let scalar = scalar.unwrap_or(1.);
        let mut translate = Vector::new_from(point);
        translate.mult(scalar);

        for vertex in self.value.iter_mut() {
            vertex.add_xy(&translate);
        }
    }

    pub fn contains(&self, point: &impl XY) -> bool {
        let mut previous_vector = &self.value[self.value.len() - 1];

        for vertex in self.value.iter() {
            if (point.get_x() - previous_vector.get_x())
                * (vertex.get_y() - previous_vector.get_y())
                + (point.get_y() - previous_vector.get_y())
                    * (previous_vector.get_x() - vertex.get_x())
                > 0.0
            {
                return false;
            }
            previous_vector = vertex
        }
        true
    }

    pub fn scale(&mut self, scale_x: f64, scale_y: f64, point: Option<&impl XY>) {
        let point = if let Some(point) = point {
            Vector::new_from(point)
        } else {
            self.centre()
        };

        if scale_x == 1.0 && scale_y == 1.0 {
            return;
        }

        for vertex in self.value.iter_mut() {
            let mut delta = vertex.clone();
            delta.sub(&point);

            vertex.set_x(point.get_x() + delta.get_x() * scale_x);
            vertex.set_y(point.get_y() + delta.get_y() * scale_y);
        }
    }

    pub fn chamfer(
        &mut self,
        radius: Option<Vec<f64>>,
        quality: Option<f64>,
        quality_min: Option<f64>,
        quality_max: Option<f64>,
    ) {
        let default_quality = -1.0;
        let radius = radius.unwrap_or(vec![8.0_f64]);
        let quality = quality.unwrap_or(default_quality);
        let quality_min = quality_min.unwrap_or(2.0_f64);
        let quality_max = quality_max.unwrap_or(104.0_f64);

        let mut new_vertices: Vec<Vertex> = Vec::new();
        let value = self.value.clone();
        for (index, vertex) in self.value.iter_mut().enumerate() {
            let prev_vertex = &value[if index > 0 {
                index - 1
            } else {
                value.len() - 1
            }];
            let next_vertex = &value[(index + 1) % value.len()];
            let current_radius = radius[if index < radius.len() {
                index
            } else {
                radius.len() - 1
            }];

            if current_radius == 0.0 {
                new_vertices.push(vertex.clone());
                continue;
            }

            let mut prev_normal = Vector::new(
                vertex.get_y() - prev_vertex.get_y(),
                prev_vertex.get_x() - vertex.get_x(),
            );
            prev_normal.normalise();

            let mut next_normal = Vector::new(
                next_vertex.get_y() - vertex.get_y(),
                vertex.get_x() - next_vertex.get_x(),
            );
            next_normal.normalise();

            let diagonal_radius = f64::sqrt(2.0 * f64::powf(current_radius, 2.0));
            let mut radius_vector = prev_normal.clone();
            radius_vector.mult(current_radius);

            let mut mid_normal = Vector::add(&prev_normal, &next_normal);
            mid_normal.mult(0.5);
            mid_normal.normalise();

            let mut scaled_vertex = vertex.clone();
            let mut mult = mid_normal.clone();
            mult.mult(diagonal_radius);
            scaled_vertex.sub(&mult);

            let mut precision = quality;
            if quality == default_quality {
                precision = f64::powf(current_radius, 0.32) * 1.75;
            }

            precision = common::clamp(precision, quality_min, quality_max);
            if precision % 2.0 == 1.0 {
                precision += 1.0;
            }

            let alpha = prev_normal.clone();
            let alpha = f64::acos(alpha.dot(&next_normal));
            let theta = alpha / precision;

            let mut index = 0_usize;
            while (index as f64) < precision {
                let mut rotated = radius_vector.clone();
                rotated.rotate(theta * index as f64);
                rotated.add_xy(&scaled_vertex);
                new_vertices.push(Vertex::from_xy(
                    vertex.get_body().clone(),
                    &rotated,
                    index,
                    vertex.get_is_internal(),
                ));
                index += 1;
            }
        }
        self.value = new_vertices;
    }

    pub fn clockwise_sort(&mut self) {
        let centre = self.mean();

        self.value.sort_by(|vector_a: &Vertex, vector_b: &Vertex| {
            let angle_a = Vector::angle(&centre, vector_a);
            let angle_b = Vector::angle(&centre, vector_b);

            if angle_a.approx_eq(angle_b, (0.0, 2)) {
                Ordering::Equal
            } else if angle_a - angle_b < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    pub fn hull(&mut self) {
        self.value.sort_by(|vector_a, vector_b| {
            let delta_x = vector_a.get_x() - vector_b.get_x();
            let compare_value = if delta_x.approx_eq(0.0, (0.0, 2)) {
                vector_a.get_y() - vector_b.get_y()
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
        while index < self.value.len() {
            let vertex = &self.value[index];

            while lower.len() >= 2
                && Vector::cross3(&lower[lower.len() - 2], &lower[lower.len() - 1], vertex) <= 0.0
            {
                lower.pop();
            }
            lower.push(vertex.clone());
            index += 1;
        }

        let mut upper: Vec<Vertex> = Vec::new();
        let mut index = (self.value.len() - 1) as i32;

        while index >= 0 {
            let vertex = &self.value[index as usize];

            while upper.len() >= 2
                && Vector::cross3(&upper[upper.len() - 2], &upper[upper.len() - 1], vertex) <= 0.0
            {
                upper.pop();
            }
            upper.push(vertex.clone());

            index -= 1;
        }
        upper.pop();
        lower.pop();

        upper.append(&mut lower);
        self.value = upper;
    }

    pub fn is_convex(&self) -> Option<bool> {
        let vertices_len = self.value.len();

        if vertices_len < 3 {
            return None;
        }

        let mut flag = 0;
        let mut index = 0_usize;
        while index < vertices_len {
            let j = (index + 1) % vertices_len;
            let k = (index + 2) % vertices_len;
            let mut z = (self.value[j].get_x() - self.value[index].get_x())
                * (self.value[k].get_y() - self.value[j].get_y());
            z -= (self.value[j].get_y() - self.value[index].get_y())
                * (self.value[k].get_x() - self.value[j].get_x());
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

    pub fn rotate(&mut self, angle: f64, point: &impl XY) {
        if angle == 0. {
            return;
        }

        let cos = f64::cos(angle);
        let sin = f64::sin(angle);

        for vertex in self.value.iter_mut() {
            let dx = vertex.get_x() - point.get_x();
            let dy = vertex.get_y() - point.get_y();
            vertex.set_x(point.get_x() + (dx * cos - dy * sin));
            vertex.set_y(point.get_y() + (dx * sin + dy * cos));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::vector,
        test_utils::{
            common_test_utils::assert_float,
            geometry_test_utils::{
                assert_xy, test_shape_convex, test_shape_non_convex, test_square,
                test_square_with_decimals, test_square_with_decimals_signed,
                vec_vector_to_vec_vertex,
            },
        },
    };

    use super::*;

    #[test]
    fn rotate_should_rotate_the_vertices_in_place() {
        // Arrange
        let mut vertices = Vertices {
            value: vec_vector_to_vec_vertex(test_square()),
        };
        let angle = 37.;
        let point = Vector::create(42., 42.);

        // Act
        vertices.rotate(angle, &point);

        // Assert
        assert_xy(&vertices[0], -15.767039597396057, 37.0030873378779);
        assert_xy(&vertices[1], -14.236211493505373, 35.716011071163905);
        assert_xy(&vertices[2], -12.94913522679137, 37.24683917505459);
        assert_xy(&vertices[3], -14.479963330682054, 38.533915441768585);
    }

    #[test]
    fn is_convex_should_return_none_for_vertices_with_z_0() {
        //Arrange
        let point_a = Vector::create(0.0, 0.0);
        let point_b = Vector::create(0.0, 0.0);
        let point_c = Vector::create(0.0, 0.0);
        let points = vec![point_a, point_b, point_c];
        let vertices = Vertices::create(points, None);

        // Act
        let result = vertices.is_convex();

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn is_convex_should_return_none_for_vertices_with_less_then_3_vectors() {
        //Arrange
        let point_a = Vector::create(0.0, 1.0);
        let point_b = Vector::create(1.0, 1.0);
        let points = vec![point_a, point_b];
        let vertices = Vertices::create(points, None);

        // Act
        let result = vertices.is_convex();

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn is_convex_should_return_false_for_non_convex_vertices() {
        //Arrange
        let points = test_shape_non_convex();
        let vertices = Vertices::create(points, None);

        // Act
        let result = vertices.is_convex();

        // Assert
        assert_eq!(result, Some(false));
    }

    #[test]
    fn is_convex_should_return_true_for_convex_vertices() {
        //Arrange
        let points = test_shape_convex();
        let vertices = Vertices::create(points, None);

        // Act
        let result = vertices.is_convex();

        // Assert
        assert_eq!(result, Some(true));
    }

    #[test]
    fn hull_should_mutate_the_vertices_to_a_valid_vec() {
        //Arrange
        let points = test_square_with_decimals();
        let mut vertices = Vertices::create(points, None);

        // Act
        vertices.hull();

        // Assert
        assert_xy(&vertices[0], 40.1, 40.1);
        assert_xy(&vertices[1], 0.0, 40.1);
        assert_xy(&vertices[2], 0.0, 0.0);
        assert_xy(&vertices[3], 40.1, 0.0);
    }

    #[test]
    fn clockwise_sort_should_mutate_the_vertices_to_a_valid_vec() {
        //Arrange
        let point_d = Vector::create(0.0, 0.0);
        let point_c = Vector::create(40.1, 0.0);
        let point_b = Vector::create(40.1, 40.1);
        let point_a = Vector::create(0.0, 40.1);

        let points = vec![point_a, point_b, point_c, point_d];

        let mut vertices = Vertices::create(points, None);

        // Act
        vertices.clockwise_sort();

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
        let mut vertices = Vertices::create(points, None);
        let radius = None;
        let quality = None;
        let quality_min = None;
        let quality_max = None;

        // Act
        vertices.chamfer(radius, quality, quality_min, quality_max);

        // Assert
        assert_eq!(vertices.len(), 16);
        assert_xy(&vertices[0], 0.0, 8.0);
        assert_xy(&vertices[1], 0.8366176963389194, 4.438265314261145);
        assert_xy(&vertices[2], 3.171488492898817, 1.6214831954606046);
        assert_xy(&vertices[3], 6.516263832912953, 0.13879608542827349);
        assert_xy(&vertices[4], 32.1, 0.0);
        assert_xy(&vertices[5], 35.66173468573886, 0.8366176963389194);
        assert_xy(&vertices[6], 38.478516804539396, 3.171488492898817);
        assert_xy(&vertices[7], 39.961203914571726, 6.516263832912953);
        assert_xy(&vertices[8], 40.1, 32.1);
        assert_xy(&vertices[9], 39.26338230366108, 35.66173468573886);
        assert_xy(&vertices[10], 36.928511507101184, 38.478516804539396);
        assert_xy(&vertices[11], 33.58373616708705, 39.961203914571726);
        assert_xy(&vertices[12], 8.0, 40.1);
        assert_xy(&vertices[13], 4.438265314261145, 39.26338230366108);
        assert_xy(&vertices[14], 1.6214831954606046, 36.928511507101184);
        assert_xy(&vertices[15], 0.13879608542827349, 33.58373616708705);
    }

    #[test]
    fn chamfer_should_create_valid_vertices_when_not_using_default_parameters() {
        //Arrange
        let points = test_square_with_decimals();
        let mut vertices = Vertices::create(points, None);
        let radius = Some(vec![2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64]);
        let quality = Some(-3.0_f64);
        let quality_min = Some(8.0_f64);
        let quality_max = Some(12.0_f64);

        // Act
        vertices.chamfer(radius, quality, quality_min, quality_max);

        // Assert
        assert_xy(&vertices[0], 0.0, 2.0);
        assert_xy(&vertices[1], 0.03842943919353914, 1.6098193559677436);
        assert_xy(&vertices[2], 0.15224093497742652, 1.2346331352698203);
        assert_xy(&vertices[3], 0.33706077539490953, 0.8888595339607956);
        assert_xy(&vertices[4], 0.5857864376269049, 0.5857864376269051);
        assert_xy(&vertices[5], 0.8888595339607954, 0.33706077539490953);
        assert_xy(&vertices[6], 1.2346331352698203, 0.15224093497742652);
        assert_xy(&vertices[7], 1.6098193559677434, 0.03842943919353914);
        assert_xy(&vertices[8], 37.1, -4.440892098500626e-16);
        assert_xy(&vertices[9], 37.685270966048385, 0.057644158790308264);
        assert_xy(&vertices[10], 38.24805029709527, 0.22836140246613956);
        assert_xy(&vertices[11], 38.766710699058805, 0.505591163092364);
        assert_xy(&vertices[12], 39.22132034355965, 0.8786796564403567);
        assert_xy(&vertices[13], 39.59440883690764, 1.3332893009411926);
        assert_xy(&vertices[14], 39.87163859753386, 1.85194970290473);
        assert_xy(&vertices[15], 40.04235584120969, 2.4147290339516143);
        assert_xy(&vertices[16], 40.1, 36.1);
        assert_xy(&vertices[17], 40.02314112161292, 36.88036128806451);
        assert_xy(&vertices[18], 39.795518130045146, 37.63073372946036);
        assert_xy(&vertices[19], 39.42587844921018, 38.322280932078414);
        assert_xy(&vertices[20], 38.92842712474619, 38.92842712474619);
        assert_xy(&vertices[21], 38.322280932078414, 39.42587844921018);
        assert_xy(&vertices[22], 37.63073372946036, 39.795518130045146);
        assert_xy(&vertices[23], 36.88036128806451, 40.02314112161292);
        assert_xy(&vertices[24], 5.0, 40.1);
        assert_xy(&vertices[25], 4.024548389919358, 40.003926402016155);
        assert_xy(&vertices[26], 3.086582838174551, 39.71939766255643);
        assert_xy(&vertices[27], 2.222148834901989, 39.25734806151273);
        assert_xy(&vertices[28], 1.4644660940672627, 38.63553390593274);
        assert_xy(&vertices[29], 0.8426519384872737, 37.877851165098015);
        assert_xy(&vertices[30], 0.3806023374435661, 37.01341716182545);
        assert_xy(&vertices[31], 0.09607359798384785, 36.07545161008064);
    }

    #[test]
    fn scale_should_mutate_the_vertices_to_valid_values() {
        //Arrange
        let points = test_square_with_decimals();
        let mut vertices = Vertices::create(points, None);
        let scale_x = 5.0_f64;
        let scale_y = 8.0_f64;
        let point = Vector::create(0.0, 0.0);

        // Act
        vertices.scale(scale_x, scale_y, Some(&point));

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
        let vertices = Vertices::create(points, None);

        let vector = Vector::create(-1.0, 0.0);

        // Act
        let result = vertices.contains(&vector);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn contains_should_respond_true_when_the_vector_is_on_the_edge() {
        // Arrange
        let points = test_square_with_decimals();
        let vertices = Vertices::create(points, None);

        let vector = Vector::create(0.0, 0.0);

        // Act
        let result = vertices.contains(&vector);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn contains_should_respond_true_when_the_vector_is_in_the_middle() {
        // Arrange
        let points = test_square_with_decimals();
        let vertices = Vertices::create(points, None);

        let vector = Vector::create(20.5, 20.5);

        // Act
        let result = vertices.contains(&vector);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn translate_should_mutate_vertices_in_a_valid_way() {
        // Arrange
        let points = test_square_with_decimals();
        let mut vertices = Vertices::create(points, None);

        let vector = Vector::create(5.0, 6.0);
        let scalar = Some(3.0_f64);

        // Act
        vertices.translate(&vector, scalar);

        // Assert
        assert_xy(&vertices[0], 15.0, 18.0);
        assert_xy(&vertices[1], 55.1, 18.0);
        assert_xy(&vertices[2], 55.1, 58.1);
        assert_xy(&vertices[3], 15.0, 58.1);
    }

    #[test]
    fn innertia_should_calculate_a_valid_value() {
        // Arrange
        let points = Vertices::create(test_square_with_decimals(), None);
        let mass = 10.0;

        // Act
        let result = points.innertia(mass);

        // Assert
        assert_float(result, 10720.06666666667);
    }

    #[test]
    fn mean_should_return_a_valid_vector() {
        // Arrange
        let points = Vertices::create(test_square_with_decimals(), None);

        // Act
        let result: Vector = points.mean();

        // Assert
        assert_xy(&result, 20.05, 20.05);
    }

    #[test]
    fn centre_should_return_a_valid_vector() {
        // Arrange
        let points = Vertices::create(test_square_with_decimals(), None);

        // Act
        let result: Vector = points.centre();

        // Assert
        assert_xy(&result, 20.05, 20.05);
    }

    #[test]
    fn area_should_calculate_a_valid_value_with_signed_false() {
        // Arrange
        let points = Vertices::create(test_square_with_decimals(), None);
        //let vertices = create(points);
        let signed = None;

        // Act
        let result: f64 = points.area(signed);

        // Assert
        assert_float(result, 1608.0100000000002_f64);
    }

    #[test]
    fn area_should_calculate_a_valid_value_with_signed_true() {
        // Arrange
        let points = Vertices::create(test_square_with_decimals(), None);
        let signed = Some(true);

        // Act
        let result: f64 = points.area(signed);

        // Assert
        assert_float(result, 1608.0100000000002_f64);
    }

    #[test]
    fn from_path_should_produce_a_valid_list_of_vectors() {
        // Arrange
        let path = "1 2 L 3, 4 L 5 6";

        // Act
        let result = Vertices::from_path(path, None).unwrap();

        // Assert
        assert_xy(&result[0], 1.0, 2.0);
        assert_xy(&result[1], 3.0, 4.0);
        assert_xy(&result[2], 5.0, 6.0);

        assert_eq!(result.len(), 3_usize);
    }

    #[test]
    fn create_should_produce_a_valid_list_of_vectors() {
        // Arrange
        let vector_a = Vector::create(1.0, 2.0);
        let vector_b = Vector::create(3.0, 4.0);
        let vector_c = Vector::create(5.0, 6.0);
        let vector_list = vec![vector_a, vector_b, vector_c];

        // Act
        let result = Vertices::create(vector_list, None);

        // Assert
        assert_xy(&result[0], 1.0, 2.0);
        assert_xy(&result[1], 3.0, 4.0);
        assert_xy(&result[2], 5.0, 6.0);
        assert_eq!(result.len(), 3);
    }
}
