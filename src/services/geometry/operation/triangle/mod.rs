use crate::structs::{Point, Triangle};

fn edge_function(p: &Point, a: &Point, b: &Point) -> f32 {
    // Devuelve el valor del producto cruzado entre los vectores ab y ap.
    // El signo indica en qué lado del borde está el punto.
    (p.pos_x - b.pos_x) * (a.pos_y - b.pos_y) - (a.pos_x - b.pos_x) * (p.pos_y - b.pos_y)
}

pub fn inside_of_triangle(point: Point, triangle: &Triangle) -> bool {
    let vertex_a = triangle.line_ab.point_a;
    let vertex_b = triangle.line_bc.point_a;
    let vertex_c = triangle.line_ca.point_a;

    let on_same_side_ab = edge_function(&point, &vertex_a, &vertex_b);
    let on_same_side_bc = edge_function(&point, &vertex_b, &vertex_c);
    let on_same_side_ca = edge_function(&point, &vertex_c, &vertex_a);

    let is_all_positive = on_same_side_ab > 0.0 && on_same_side_bc > 0.0 && on_same_side_ca > 0.0;
    let is_all_negative = on_same_side_ab < 0.0 && on_same_side_bc < 0.0 && on_same_side_ca < 0.0;

    is_all_positive || is_all_negative
}