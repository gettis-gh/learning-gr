use crate::structs::{
    Pixel,
    Color,
    Context
};

pub fn draw_pixel(context: &mut Context, pixel: Pixel, color: Color) {
    if pixel.pos_x >= context.width || pixel.pos_y >= context.height {
        return;
    }

    let i = (pixel.pos_y * context.width + pixel.pos_x) * 4;
    if i + 3 < context.frame.len() {
        context.frame[i] = color.red;
        context.frame[i + 1] = color.green;
        context.frame[i + 2] = color.blue;
        context.frame[i + 3] = color.alpha;
    }
}