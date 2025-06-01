use crate::structs::geometry::Mesh;
use nalgebra::{Vector3};

pub fn create_cube_mesh(size: f32) -> Mesh {
    let half = size / 2.0;

    let vertices = vec![
        // Frente
        Vector3::new(-half, -half,  half), // 0
        Vector3::new( half, -half,  half), // 1
        Vector3::new( half,  half,  half), // 2
        Vector3::new(-half,  half,  half), // 3
        // Atrás
        Vector3::new(-half, -half, -half), // 4
        Vector3::new( half, -half, -half), // 5
        Vector3::new( half,  half, -half), // 6
        Vector3::new(-half,  half, -half), // 7
    ];

    let indices = vec![
        // Frente
        0, 1, 2,  0, 2, 3,
        // Derecha
        1, 5, 6,  1, 6, 2,
        // Atrás
        5, 4, 7,  5, 7, 6,
        // Izquierda
        4, 0, 3,  4, 3, 7,
        // Arriba
        3, 2, 6,  3, 6, 7,
        // Abajo
        4, 5, 1,  4, 1, 0,
    ];

    Mesh { vertices, indices }
}