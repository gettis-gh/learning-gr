pub mod helper;

use helper::{
    bounding_box,
    cell_range_for_bbox,
    cell_dimensions
};

use crate::structs::{
    geometry::Triangle3,
    rasterization::TriangleGrid
};
use std::sync::Arc;

pub fn create_triangle_spatial_grid(
    triangles: &[Triangle3],
    screen_width: usize,
    screen_height: usize,
    num_cells_horizontal: usize,
    num_cells_vertical: usize,
) -> TriangleGrid {
    let (cell_width, cell_height) = 
        cell_dimensions(
            screen_width, 
            screen_height, 
            num_cells_horizontal, 
            num_cells_vertical
        );
    let mut grid: Vec<Vec<Triangle3>> = vec![vec![]; num_cells_horizontal * num_cells_vertical];

    for triangle in triangles {
        let bbox = bounding_box(triangle);

        let (start_col, end_col, start_row, end_row) = cell_range_for_bbox(
            bbox,
            cell_width,
            cell_height,
            num_cells_horizontal,
            num_cells_vertical,
        );

        for row in start_row..=end_row {
            for col in start_col..=end_col {
                let index = row * num_cells_horizontal + col;
                grid[index].push(triangle.clone());
            }
        }
    }

    TriangleGrid {
        grid: Arc::new(grid),
        cell_width,
        cell_height,
        cols: num_cells_horizontal,
        rows: num_cells_vertical
    }
}