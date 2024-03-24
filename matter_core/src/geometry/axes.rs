use std::collections::HashMap;

use super::vector::{self, Vector};

pub fn from_vertices(vertices: &Vec<Vector>) -> Vec<Vector> {
    let mut axes: HashMap<String, Vector> = HashMap::new();
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
            normal.x / normal.y
        };
        let gradient = format!("{0:.3}", gradient);
        axes.insert(gradient, normal);

        index += 1;
    }

    axes.values().cloned().collect()
}

#[cfg(test)]
mod tests {

    use crate::geometry::vertices;
    use crate::test_utils::geometry_test_utils::{assert_vector, test_square};

    use super::*;

    #[test]
    fn from_vertices_should_return_valid_vectors_as_axes() {
        //Arrange
        let points = test_square();
        let vertices = vertices::create(points);

        // Act
        let result = from_vertices(&vertices);

        // Assert
        assert_vector(&result[0], 0.0, 1.0);
        //assert_vector(&result[1], -1.0, 0.0);
    }
}
