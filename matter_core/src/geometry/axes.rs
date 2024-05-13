use crate::core::common::OrderedHashMap;
use crate::core::xy::XY;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::Iter;
use std::slice::IterMut;

use super::vector::Vector;
use super::{vector, vertex::Vertex};

pub struct Axes {
    value: Vec<Vertex>,
}

impl Index<usize> for Axes {
    type Output = Vertex;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for Axes {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        &mut self.value[index]
    }
}

impl Axes {
    pub fn iter(&self) -> Iter<Vertex> {
        self.value.iter()
    }

    pub fn iter_mut(&self) -> IterMut<Vertex> {
        self.value.iter_mut()
    }

    pub fn from_vertices(vertices: &Vec<Vertex>) -> Self {
        let mut axes: OrderedHashMap<Vertex> = OrderedHashMap::new();
        let vertices_len = vertices.len();

        let mut index = 0;
        while index < vertices_len {
            let next_index = (index + 1) % vertices_len;

            let x = vertices[next_index].get_y() - vertices[index].get_y();
            let y = vertices[index].get_x() - vertices[next_index].get_x();
            let mut normal = Vector::create(x, y);
            normal.normalise();

            let vertex = vertices[index];
            let normal = Vertex::new(
                vertex.get_body(),
                normal.get_x(),
                normal.get_y(),
                index,
                vertex.get_is_internal(),
            );

            let gradient = if normal.get_y() == 0.0 {
                f64::INFINITY
            } else {
                let result = normal.get_x() / normal.get_y();
                if result == -0.0 {
                    0.0
                } else {
                    result
                }
            };
            let gradient = format!("{0:.3}", gradient);

            axes.insert(gradient, normal);

            index += 1;
        }
        Axes { value: axes.values }
    }

    pub fn rotate(&mut self, angle: f64) {
        let cos = angle.cos();
        let sin = angle.sin();

        for axis in self.value.iter_mut() {
            let xx = axis.get_x() * cos - axis.get_y() * sin;
            axis.set_y(axis.get_x() * sin + axis.get_y() * cos);
            axis.set_x(xx);
        }
    }
}

#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use crate::geometry::vertices::{self, Vertices};
    use crate::test_utils::geometry_test_utils::{assert_xy, test_square};

    use super::*;

    #[test]
    fn rotate_should_mutate_the_vertices_in_valid_way() {
        //Arrange
        let points = test_square();
        let vertices = Vertices::create(points, None);
        let mut axes = Axes::from_vertices(&vertices);
        let angle = 90.0_f64;

        // Act
        axes.rotate(angle);

        // Assert
        assert_xy(&axes[0], -0.8939966636005579, -0.4480736161291702);
        assert_xy(&axes[1], 0.4480736161291702, -0.8939966636005579);
    }

    #[test]
    fn from_vertices_should_return_valid_vectors_as_axes() {
        //Arrange
        let points = test_square();
        let vertices = Vertices::create(points, None);

        // Act
        let result = Axes::from_vertices(&vertices);

        // Assert
        assert_xy(&result[0], 0.0, 1.0);
        assert_xy(&result[1], -1.0, 0.0);
    }
}
