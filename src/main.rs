pub mod services;
pub mod structs;
pub mod template;
pub mod traits;
use hecs::{World};
use nalgebra::{Vector3};
use nalgebra::{UnitQuaternion, Unit};
use rand::Rng;

use services::{
    geometry::{
        operation::triangle::{
            inside_of_triangle,
            spatial_grid::create_triangle_spatial_grid
        },
        operation::mesh::load_meshes_from_gltf
    },
    rasterizer::{
        clear_frame,
        draw_pixels
    }
};

use structs::{
    geometry::{
        Point,
        Point3,
        Triangle3,
        Mesh
    },
    rasterization::{
        Context,
        Color
    },
    math::{
        Transform,
        AngularVelocity
    }
};

fn barycentric_coords(p: &Point, a: &Point3, b: &Point3, c: &Point3) -> Option<(f32, f32, f32)> {
    let denom = (b.pos_y - c.pos_y) * (a.pos_x - c.pos_x) + (c.pos_x - b.pos_x) * (a.pos_y - c.pos_y);
    if denom == 0.0 {
        return None; // triángulo degenerado
    }
    let alpha = ((b.pos_y - c.pos_y) * (p.pos_x - c.pos_x) + (c.pos_x - b.pos_x) * (p.pos_y - c.pos_y)) / denom;
    let beta = ((c.pos_y - a.pos_y) * (p.pos_x - c.pos_x) + (a.pos_x - c.pos_x) * (p.pos_y - c.pos_y)) / denom;
    let gamma = 1.0 - alpha - beta;

    // Opcional: chequea que el punto esté dentro del triángulo
    if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0 {
        Some((alpha, beta, gamma))
    } else {
        None
    }
}

fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();
    let window_service = services::window::WindowService::new(&event_loop);

    let mut world = World::new();

    let mut rng = rand::thread_rng();
    
    use std::path::Path;
    use std::fs;

    let asset_dir = "./assets";
    println!("Listing assets directory:");
    for entry in fs::read_dir(asset_dir).expect("Failed to read assets dir") {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();
        println!(" - {}", path.display());
    }

    let path = "./assets/car_pack/scene.gltf";
    let meshes = load_meshes_from_gltf(&path);

    for mesh in meshes[0..5].iter().cloned() {
        let position = Vector3::new(
            rng.gen_range(300.0..500.0),
            rng.gen_range(200.0..400.0),
            0.0,
        );

        let transform = Transform {
            position,
            rotation: UnitQuaternion::identity(),
            scale: Vector3::new(0.05, 0.05, 0.05),
        };

        let axis = Unit::new_normalize(Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ));

        let speed = 1.0;

        let angular_velocity = AngularVelocity { axis, speed };

        let num_triangles = mesh.indices.len() / 3;

        let color = Color {
            red: rng.gen_range(0..=255),
            green: rng.gen_range(0..=255),
            blue: rng.gen_range(0..=255),
            alpha: 0xff
        };

        let triangle_colors: Vec<Color> = (0..num_triangles)
            .map(|_| {
                let red = (color.red as i16 + rng.gen_range(-30..30)).clamp(0, 255) as u8;
                let green = (color.green as i16 + rng.gen_range(-30..30)).clamp(0, 255) as u8;
                let blue = (color.blue as i16 + rng.gen_range(-30..30)).clamp(0, 255) as u8;

                Color { red, green, blue, alpha: 0xff }
            })
            .collect();
    

        world.spawn((mesh, transform, angular_velocity, triangle_colors));
    }

    window_service.run(event_loop, move |frame, width, height| {
        let mut context = Context {
            frame,
            width,
            height,
        };

        for (_entity, (_mesh, transform, angular_velocity)) in world.query_mut::<(&Mesh, &mut Transform, &AngularVelocity)>() {
            let delta_rotation = UnitQuaternion::from_axis_angle(&angular_velocity.axis, angular_velocity.speed);
            transform.rotation = delta_rotation * transform.rotation;
        }
        
        let triangles: Vec<Triangle3> = world.query::<(&Mesh, &Transform, &Vec<Color>)>()
            .iter()
            .flat_map(|(_entity, (mesh, transform, triangle_colors))| {
                mesh.to_triangles(transform, triangle_colors)
            })
            .collect();

        let grid = create_triangle_spatial_grid(&triangles, width, height, 64, 64);

        let color_shader = move |_x: usize, _y: usize, point: &Point, triangles: &[Triangle3]| {
            let mut top_color = Color { red: 0, green: 0, blue: 0, alpha: 0 };
            let mut top_depth = f32::NEG_INFINITY;
        
            for triangle in triangles {
                if let Some((alpha, beta, gamma)) = barycentric_coords(point, &triangle.point_a, &triangle.point_b, &triangle.point_c) {
                    let depth = alpha * triangle.point_a.pos_z + beta * triangle.point_b.pos_z + gamma * triangle.point_c.pos_z;
                    if depth > top_depth {
                        top_depth = depth;
                        top_color = triangle.color;
                    }
                }
            }
            top_color
        };
        let condition = |point: &Point, triangles: &[Triangle3]| {
            triangles.iter().any(|t| inside_of_triangle(*point, t))
        };

        clear_frame(&mut context, Color {red: 0, green: 0, blue: 0, alpha: 0xff});
        draw_pixels(&mut context, grid, color_shader, condition);
    });
}