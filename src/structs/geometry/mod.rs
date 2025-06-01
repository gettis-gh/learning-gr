use crate::structs::math::Transform;
use crate::structs::rasterization::Color;
use nalgebra::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub pos_x: f32,
    pub pos_y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub point_a: Point,
    pub point_b: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle3 {
    pub point_a: Point3,
    pub point_b: Point3,
    pub point_c: Point3,
    pub color: Color,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vector3<f32>>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn to_triangles(&self, transform: &Transform, triangle_colors: &[Color]) -> Vec<Triangle3> {
        let mut triangles = Vec::new();
        for (i, chunk) in self.indices.chunks(3).enumerate() {
            if chunk.len() == 3 {
                let vertex_a = transform.apply(&self.vertices[chunk[0] as usize]);
                let vertex_b = transform.apply(&self.vertices[chunk[1] as usize]);
                let vertex_c = transform.apply(&self.vertices[chunk[2] as usize]);

                let point_a = Point3 { pos_x: vertex_a.x, pos_y: vertex_a.y, pos_z: vertex_a.z };
                let point_b = Point3 { pos_x: vertex_b.x, pos_y: vertex_b.y, pos_z: vertex_b.z };
                let point_c = Point3 { pos_x: vertex_c.x, pos_y: vertex_c.y, pos_z: vertex_c.z };

                let color = triangle_colors.get(i).copied().unwrap_or(Color {
                    red: 255, green: 255, blue: 255, alpha: 255,
                });
                

                triangles.push(Triangle3 {
                    point_a,
                    point_b,
                    point_c,
                    color,
                }); 
            }
        }
        triangles
    }
}