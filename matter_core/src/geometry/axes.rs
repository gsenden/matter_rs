use crate::core::common::OrderedHashMap;
use crate::core::xy::{XYGet, XYSet};

use super::{
    vector::{self},
    vertices::Vertex,
};

pub fn from_vertices(vertices: &Vec<Vertex>) -> Vec<Vertex> {
    let mut axes: OrderedHashMap<Vertex> = OrderedHashMap::new();
    let vertices_len = vertices.len();

    let mut index = 0;
    while index < vertices_len {
        let next_index = (index + 1) % vertices_len;

        let normal = vector::normalise(&vector::create(
            vertices[next_index].get_y() - vertices[index].get_y(),
            vertices[index].get_x() - vertices[next_index].get_x(),
        ));
        let normal = Vertex::from_vector(
            vertices[index].get_body_id(),
            &normal,
            index,
            vertices[index].get_is_internal(),
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

    axes.values
}

fn rotate(axes: &mut Vec<Vertex>, angle: f64) {
    let cos = angle.cos();
    let sin = angle.sin();

    for axis in axes {
        let xx = axis.get_x() * cos - axis.get_y() * sin;
        axis.set_y(axis.get_x() * sin + axis.get_y() * cos);
        axis.set_x(xx);
    }
}

#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use crate::geometry::vertices;
    use crate::test_utils::geometry_test_utils::{assert_xy, test_square};

    use super::*;

    #[test]
    fn rotate_should_mutate_the_vertices_in_valid_way() {
        //Arrange
        let points = test_square();
        let vertices = vertices::create(points, Uuid::new_v4());
        let mut axes = from_vertices(&vertices);
        let angle = 90.0_f64;

        // Act
        rotate(&mut axes, angle);

        // Assert
        assert_xy(&axes[0], -0.8939966636005579, -0.4480736161291702);
        assert_xy(&axes[1], 0.4480736161291702, -0.8939966636005579);
    }

    #[test]
    fn from_vertices_should_return_valid_vectors_as_axes() {
        //Arrange
        let points = test_square();
        let vertices = vertices::create(points, Uuid::new_v4());

        // Act
        let result = from_vertices(&vertices);

        // Assert
        assert_xy(&result[0], 0.0, 1.0);
        assert_xy(&result[1], -1.0, 0.0);
    }
}
