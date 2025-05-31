use crate::structs::{Point, Line};

pub fn create_line(point_a: Point, point_b: Point) -> Line {
    Line {
        point_a: point_a, 
        point_b: point_b,
    }
}