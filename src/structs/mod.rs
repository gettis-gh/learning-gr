// structs/mod.rs

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub pos_x: f32,
    pub pos_y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub point_a: Point,
    pub point_b: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub line_ab: Line,
    pub line_bc: Line,
    pub line_ca: Line,
}