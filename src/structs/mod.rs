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

pub struct Context<'a> {
    pub frame: &'a mut [u8],
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub alpha: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub pos_x: usize,
    pub pos_y: usize,
}