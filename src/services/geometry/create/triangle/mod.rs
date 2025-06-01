use crate::structs::geometry::{Point, Triangle};
use crate::structs::rasterization::{Color};

use rand::Rng;

pub fn generate_random_triangles(
    count: usize,
    scale_range: (f32, f32),
    area_width: f32,
    area_height: f32,
) -> Vec<Triangle> {
    let mut rng = rand::thread_rng();
    let mut triangles = Vec::with_capacity(count);

    for _ in 0..count {
        // Posición base aleatoria
        let base_x = rng.gen_range(0.0..area_width);
        let base_y = rng.gen_range(0.0..area_height);

        // Tamaño aleatorio dentro del rango
        let scale = rng.gen_range(scale_range.0..scale_range.1);

        // Construimos un triángulo isósceles básico escalado y desplazado
        let point_a = Point { pos_x: base_x,         pos_y: base_y };
        let point_b = Point { pos_x: base_x + scale, pos_y: base_y };
        let point_c = Point { pos_x: base_x,         pos_y: base_y + scale };

        let color = Color {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 0xff,
        };

        let triangle = create_triangle(point_a, point_b, point_c, color);
        triangles.push(triangle);
    }

    triangles
}

pub fn create_triangle(point_a: Point, point_b: Point, point_c: Point, color: Color) -> Triangle {
    Triangle {
        point_a,
        point_b,
        point_c,
        color,
    }
}