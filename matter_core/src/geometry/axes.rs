use crate::core::common::OrderedHashMap;

use super::vector::{self, Vector};

pub fn from_vertices(vertices: &Vec<Vector>) -> Vec<Vector> {
    let mut axes: OrderedHashMap<Vector> = OrderedHashMap::new();
    let vertices_len = vertices.len();

    let mut index = 0;
    while index < vertices_len {
        let next_index = (index + 1) % vertices_len;
        let normal = vector::normalise(&vector::create(
            vertices[next_index].y - vertices[index].y,
            vertices[index].x - vertices[next_index].x,
        ));

        let gradient = if normal.y == 0.0 {
            f64::INFINITY
        } else {
            let result = normal.x / normal.y;
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

fn rotate(axes: &mut Vec<Vector>, angle: f64) {
    let cos = angle.cos();
    let sin = angle.sin();

    for axis in axes {
        let xx = axis.x * cos - axis.y * sin;
        axis.y = axis.x * sin + axis.y * cos;
        axis.x = xx;
    }
}

#[cfg(test)]
mod tests {

    use crate::geometry::vertices;
    use crate::test_utils::geometry_test_utils::{assert_vector, test_square};

    use super::*;

    #[test]
    fn rotate_should_mutate_the_vertices_in_valid_way() {
        //Arrange
        let points = test_square();
        let vertices = vertices::create(points);
        let mut axes = from_vertices(&vertices);
        let angle = 90.0_f64;

        // Act
        rotate(&mut axes, angle);

        // Assert
        assert_vector(&axes[0], -0.8939966636005579, -0.4480736161291702);
        assert_vector(&axes[1], 0.4480736161291702, -0.8939966636005579);
    }

    #[test]
    fn from_vertices_should_return_valid_vectors_as_axes() {
        //Arrange
        let points = test_square();
        let vertices = vertices::create(points);

        // Act
        let result = from_vertices(&vertices);

        // Assert
        assert_vector(&result[0], 0.0, 1.0);
        assert_vector(&result[1], -1.0, 0.0);
    }
}
