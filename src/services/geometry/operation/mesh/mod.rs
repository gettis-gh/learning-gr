use nalgebra::Vector3;

use crate::structs::geometry::Mesh;

pub fn load_meshes_from_gltf(path: &str) -> Vec<Mesh> {
    println!("Inside load_meshes_from_gltf, loading from: {}", path);
    let (gltf, buffers, _) = gltf::import(path).expect("Failed to load GLTF file");

    let mut meshes = Vec::new();

    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            // Leer atributos de posición
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions = reader
                .read_positions()
                .expect("No positions found")
                .map(|p| Vector3::new(p[0], p[1], p[2]))
                .collect::<Vec<_>>();

            // Leer índices
            let indices = reader
                .read_indices()
                .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>())
                .unwrap_or_else(|| (0..positions.len() as u32).collect());

            meshes.push(Mesh {
                vertices: positions,
                indices,
            });
        }
    }

    meshes
}