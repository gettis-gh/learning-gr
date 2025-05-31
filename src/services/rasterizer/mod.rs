pub mod math;

use crate::structs::{
    Color,
    Context,
    Triangle,
    Point,
    TriangleGrid
};

use rayon::prelude::{ParallelSliceMut, ParallelIterator, IndexedParallelIterator};

pub fn clear_frame(context: &mut Context, color: Color) {
    context.frame
        .par_chunks_exact_mut(4)
        .for_each(|pixel| {
            pixel[0] = color.red;
            pixel[1] = color.green;
            pixel[2] = color.blue;
            pixel[3] = color.alpha;
        });
}

pub fn draw_pixels<FColor, FCond>(
    context: &mut Context,
    triangle_grid: TriangleGrid,
    color_fn: FColor,
    condition_fn: FCond,
)
where
    FColor: Fn(usize, usize, &Point) -> Color + Sync + Send,
    FCond: Fn(&Point, &[Triangle]) -> bool + Sync + Send,
{
    let width = context.width;

    context
        .frame
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(index, pixel)| {
            let pixel_x = index % width;
            let pixel_y = index / width;

            let col = (pixel_x / triangle_grid.cell_width).min(triangle_grid.cols - 1);
            let row = (pixel_y / triangle_grid.cell_height).min(triangle_grid.rows - 1);
            let cell = row * triangle_grid.cols + col;

            let point = Point {
                pos_x: pixel_x as f32,
                pos_y: pixel_y as f32,
            };

            let triangles = &triangle_grid.grid[cell];
            if condition_fn(&point, triangles) {
                let color = color_fn(pixel_x, pixel_y, &point);
                pixel[0] = color.red;
                pixel[1] = color.green;
                pixel[2] = color.blue;
                pixel[3] = color.alpha;
            }
        });
}