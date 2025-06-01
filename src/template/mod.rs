use noise::{Perlin, NoiseFn};
use crate::structs::{
    rasterization::Color,
    geometry::Point
};

pub fn precomputed_perlin_noise(width: usize, height: usize, perlin: &Perlin, scale: f64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let noise_value = perlin.get([x as f64 * scale, y as f64 * scale]);
            let normalized = ((noise_value + 1.0) / 2.0).clamp(0.0, 1.0);
            let color_value = (normalized * 255.0) as u8;
            buffer.push(color_value);
        }
    }
    buffer
}

pub fn perlin_noise_shader_from_buffer(buffer: &[u8], width: usize) -> impl Fn(usize, usize, &Point) -> Color + Sync + Send + '_ {
    move |x, y, _point| {
        let idx = y * width + x;
        let val = buffer[idx];
        Color {
            red: val,
            green: val,
            blue: val,
            alpha: 0xff,
        }
    }
}