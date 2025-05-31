use crate::structs::{Point, Line};

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