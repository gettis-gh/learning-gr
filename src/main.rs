// main.rs

pub mod services;
pub mod structs;

use image::{RgbImage, Rgb};

fn main() {
    let image_width = 1920;
    let image_height = 1080;

    let triangles = services::generate_random_equilateral_triangles(image_width, image_height, 50);

    let mut image = RgbImage::new(image_width, image_height);

    for pixel_x in 0..image_width {
        for pixel_y in 0..image_height {
            let pixel = structs::Point {
                pos_x: pixel_x as f32 - (image_width as f32 / 2.0),
                pos_y: pixel_y as f32 - (image_height as f32 / 2.0),
            };

            let horizontal_ratio = pixel_x as f32 / (image_width - 1) as f32;
            let vertical_ratio = pixel_y as f32 / (image_height - 1) as f32;

            let red_intensity = ((1.0 - vertical_ratio) * horizontal_ratio * 255.0) as u8;
            let green_intensity = (vertical_ratio * (1.0 - horizontal_ratio) * 255.0) as u8;
            let blue_intensity = (vertical_ratio * horizontal_ratio * 255.0) as u8;

            let pixel_color = Rgb([red_intensity, green_intensity, blue_intensity]);

            for triangle in &triangles {
                if services::inside_triangle(pixel, triangle) {
                    image.put_pixel(pixel_x, pixel_y, pixel_color);
                }
            }
        }
    }

    image.save("image.bmp").unwrap();
}