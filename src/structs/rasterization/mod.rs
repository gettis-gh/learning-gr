use std::sync::Arc;
use crate::structs::geometry::{
    Triangle3, Point
};

#[derive(Debug, PartialEq)]
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
    pub point: Point,
    pub color: Color
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriangleGrid {
    pub grid: Arc<Vec<Vec<Triangle3>>>,
    pub cell_width: usize,
    pub cell_height: usize,
    pub cols: usize,
    pub rows: usize,
}