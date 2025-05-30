// services/mod.rs

use crate::structs::{Point, Line, Triangle};
use rand::Rng;

pub fn at_left_of_line(point: Point, line: Line) -> bool {
    let start = line.point_a;
    let end = line.point_b;

    let vector_ab_x = end.pos_x - start.pos_x;
    let vector_ab_y = end.pos_y - start.pos_y;
    let vector_ap_x = point.pos_x - start.pos_x;
    let vector_ap_y = point.pos_y - start.pos_y;

    let cross_product = vector_ab_x * vector_ap_y - vector_ab_y * vector_ap_x;

    cross_product >= 0.0
}

pub fn inside_triangle(point: Point, triangle: &Triangle) -> bool {
    let lines = [
        ("line_ab", triangle.line_ab),
        ("line_bc", triangle.line_bc),
        ("line_ca", triangle.line_ca),
    ];

    for (_, line) in &lines {
        if at_left_of_line(point, *line) {
            return false;
        }
    }
    true
}

pub fn create_triangle(a: Point, b: Point, c: Point) -> Triangle {
    Triangle {
        line_ab: Line { point_a: a, point_b: b },
        line_bc: Line { point_a: b, point_b: c },
        line_ca: Line { point_a: c, point_b: a },
    }
}

pub fn create_equilateral_triangle(width: u32, height: u32, size_ratio: f32) -> Triangle {
    let base = size_ratio * width.min(height) as f32;
    let height_triangle = (base * (3.0f32).sqrt()) / 2.0;

    // Coordenadas centradas
    let center_x = 0.0;
    let center_y = 0.0;

    let half_base = base / 2.0;
    let half_height = height_triangle / 2.0;

    let a = Point {
        pos_x: center_x - half_base,
        pos_y: center_y + half_height,
    };

    let b = Point {
        pos_x: center_x + half_base,
        pos_y: center_y + half_height,
    };

    let c = Point {
        pos_x: center_x,
        pos_y: center_y - half_height,
    };

    Triangle {
        line_ab: Line { point_a: a, point_b: b },
        line_bc: Line { point_a: b, point_b: c },
        line_ca: Line { point_a: c, point_b: a },
    }
}

pub fn create_equilateral_triangle_at(center: Point, size: f32) -> Triangle {
    let base = size;
    let height_triangle = (base * (3.0f32).sqrt()) / 2.0;

    let half_base = base / 2.0;
    let half_height = height_triangle / 2.0;

    let a = Point {
        pos_x: center.pos_x - half_base,
        pos_y: center.pos_y + half_height,
    };

    let b = Point {
        pos_x: center.pos_x + half_base,
        pos_y: center.pos_y + half_height,
    };

    let c = Point {
        pos_x: center.pos_x,
        pos_y: center.pos_y - half_height,
    };

    Triangle {
        line_ab: Line { point_a: a, point_b: b },
        line_bc: Line { point_a: b, point_b: c },
        line_ca: Line { point_a: c, point_b: a },
    }
}

pub fn generate_random_equilateral_triangles(width: u32, height: u32, count: usize) -> Vec<Triangle> {
    let mut rng = rand::thread_rng();
    let mut triangles = Vec::with_capacity(count);

    let min_size = 0.05; // 5%
    let max_size = 0.2;  // 20%
    let min_dim = width.min(height) as f32;

    for _ in 0..count {
        let size_ratio = rng.gen_range(min_size..=max_size);
        let size = size_ratio * min_dim;

        let center_x = rng.gen_range(-(width as f32) / 2.0..=(width as f32) / 2.0);
        let center_y = rng.gen_range(-(height as f32) / 2.0..=(height as f32) / 2.0);

        let center = Point {
            pos_x: center_x,
            pos_y: center_y,
        };

        let triangle = create_equilateral_triangle_at(center, size);
        triangles.push(triangle);
    }

    triangles
}
