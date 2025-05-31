pub mod services;
pub mod structs;

use services::{
    geometry::{
        create::triangle::{
            create_triangle,
            generate_random_triangles
        },
        operation::triangle::inside_of_triangle,
    },
    rasterizer::draw_pixel
};

use structs::{
    Point, 
    Context,
    Pixel,
    Color,
    Triangle
};

fn draw_gradient_triangles(context: &mut Context, triangles: &[Triangle]) {
    for pixel in context.frame.chunks_exact_mut(4) {
        pixel[0] = 0; // R
        pixel[1] = 0; // G
        pixel[2] = 0; // B
        pixel[3] = 0xff; // A
    }

    for y in 0..context.height {
        for x in 0..context.width {
            let fx = x as f32 / (context.width - 1) as f32;
            let fy = y as f32 / (context.height - 1) as f32;

            let red = ((1.0 - fx) * (1.0 - fy) * 255.0) as u8;
            let green = (fx * (1.0 - fy) * 255.0) as u8;
            let blue = ((1.0 - fx) * fy * 255.0) as u8;

            let pixel = Point {
                pos_x: x as f32,
                pos_y: y as f32
            };

            for triangle in triangles {
                if inside_of_triangle(pixel, triangle) {
                    let pixel = Pixel {
                        pos_x: x,
                        pos_y: y
                    };
                    let color = Color {
                        red,
                        green,
                        blue,
                        alpha: 0xff
                    };
                    draw_pixel(context, pixel, color);
                }
            }
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();
    let window_service = services::window::WindowService::new(&event_loop);

    window_service.run(event_loop, move |frame, width, height| {
        let triangles = generate_random_triangles(
            10,         // cantidad de triángulos
            (20.0, 100.0), // escala mínima y máxima
            width as f32,
            height as f32,
        );
        let mut context = Context {
            frame,
            width,
            height,
        };
        draw_gradient_triangles(&mut context, &triangles);
    });
}