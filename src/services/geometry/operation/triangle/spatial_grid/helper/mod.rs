use crate::structs::{
    geometry::{Triangle3},
    rasterization::BoundingBox
};

pub fn bounding_box(triangle: &Triangle3) -> BoundingBox {
    let points = [
        triangle.point_a,
        triangle.point_b,
        triangle.point_c,
    ];

    let left = points.iter()
        .map(|p| p.pos_x)
        .fold(f32::INFINITY, f32::min)
        .floor()
        .max(0.0) as usize;

    let right = points.iter()
        .map(|p| p.pos_x)
        .fold(f32::NEG_INFINITY, f32::max)
        .ceil() as usize;

    let top = points.iter()
        .map(|p| p.pos_y)
        .fold(f32::INFINITY, f32::min)
        .floor()
        .max(0.0) as usize;

    let bottom = points.iter()
        .map(|p| p.pos_y)
        .fold(f32::NEG_INFINITY, f32::max)
        .ceil() as usize;

    BoundingBox { left, right, top, bottom }
}

pub fn cell_range_for_bbox(
    bbox: BoundingBox, 
    cell_width: usize, 
    cell_height: usize, 
    num_cells_horizontal: usize, 
    num_cells_vertical: usize
) -> (usize, usize, usize, usize) {
    let start_col = (bbox.left / cell_width).min(num_cells_horizontal - 1);
    let end_col = (bbox.right / cell_width).min(num_cells_horizontal - 1);
    let start_row = (bbox.top / cell_height).min(num_cells_vertical - 1);
    let end_row = (bbox.bottom / cell_height).min(num_cells_vertical - 1);
    (start_col, end_col, start_row, end_row)
}

pub fn cell_dimensions(
    screen_width: usize,
    screen_height: usize,
    num_cells_horizontal: usize,
    num_cells_vertical: usize,
) -> (usize, usize) {
    let cell_width = (screen_width as f32 / num_cells_horizontal as f32).floor() as usize;
    let cell_height = (screen_height as f32 / num_cells_vertical as f32).floor() as usize;
    (cell_width, cell_height)
}