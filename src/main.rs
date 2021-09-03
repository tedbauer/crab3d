extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

struct Point2d {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for Point2d {
    fn from((x, y): (f64, f64)) -> Self {
        Point2d { x, y }
    }
}

impl From<&Point2d> for Point {
    fn from(point: &Point2d) -> Self {
        Point::from((point.x as i32, point.y as i32))
    }
}

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vec3 {
    fn multiply_by_scalar(&self, scalar: f64) -> Self {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Vec4 {
    fn multiply_by_scalar(&self, scalar: f64) -> Self {
        Vec4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    fn multiply_by_matrix(&self, matrix: &Matrix4x4) -> Self {
        let x = self.x * matrix[0][0]
            + self.y * matrix[1][0]
            + self.z * matrix[2][0]
            + self.w * matrix[3][0];
        let y = self.x * matrix[0][1]
            + self.y * matrix[1][1]
            + self.z * matrix[2][1]
            + self.w * matrix[3][1];
        let z = self.x * matrix[0][2]
            + self.y * matrix[1][2]
            + self.z * matrix[2][2]
            + self.w * matrix[3][2];
        let w = self.x * matrix[0][3]
            + self.y * matrix[1][3]
            + self.z * matrix[2][3]
            + self.w * matrix[3][3];
        Vec4 { x, y, z, w }
    }
}

fn project_vertex(vec: &Vec3, projection_matrix: &Matrix4x4) -> Vec4 {
    let projected = Vec4 {
        x: vec.x,
        y: vec.y,
        z: vec.z,
        w: 1.0,
    }
    .multiply_by_matrix(projection_matrix);
    let scalar = if projected.w != 0.0 { projected.w } else { 1.0 };
    projected.multiply_by_scalar(1.0 / scalar)
}

struct Triangle(Vec3, Vec3, Vec3);

type Matrix4x4 = [[f64; 4]; 4];

fn draw_triangle(a: Point2d, b: Point2d, c: Point2d, canvas: &mut Canvas<Window>) -> () {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_line(Point::from(&a), Point::from(&b));
    canvas.draw_line(Point::from(&b), Point::from(&c));
    canvas.draw_line(Point::from(&c), Point::from(&a));
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let (window_height, window_width) = window.size();
    let aspect_ratio: f64 = window_height as f64 / window_width as f64;

    let theta = 90.0;
    let field_of_view = 1.0 / (theta * 0.5 / 180.0 * 3.14159);

    let z_far = 0.1;
    let z_near = 1000.0;

    let q = z_far / (z_far - z_near);

    let projection_matrix: Matrix4x4 = [
        [aspect_ratio * field_of_view, 0.0, 0.0, 0.0],
        [0.0, field_of_view, 0.0, 0.0],
        [0.0, 0.0, q, 1.0],
        [0.0, 0.0, -1.0 * z_near * q, 0.0],
    ];

    let cube_mesh = vec![
        // Front
        Triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
        ),
        Triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        ),
        // Right side
        Triangle(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        ),
        Triangle(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        ),
        // Left side
        Triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            },
        ),
        Triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        ),
        // Back
        Triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        ),
        Triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        ),
        // Top
        Triangle(
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        ),
        Triangle(
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
        ),
        // Bottom
        Triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        ),
        Triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        ),
    ];

    let mut canvas = window.into_canvas().build().unwrap();

    let mut i = 3.0;
    let mut f_theta: f64 = 0.0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        //i += 0.2;
        f_theta += 0.03;
        
        let rotation_z: Matrix4x4 = [
            [f_theta.cos(), -f_theta.sin(), 0.0, 0.0],
            [f_theta.sin(), f_theta.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let rotation_x: Matrix4x4 = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, f_theta.cos(), -f_theta.sin(), 0.0],
            [0.0, f_theta.sin(), f_theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        for tri in &cube_mesh {
            let Triangle(vertex_a_1, vertex_b_1, vertex_c_1) = tri;

            let vertex_a = Vec4 { x: vertex_a_1.x, y: vertex_a_1.y, z: vertex_a_1.z, w: 1.0 }
                .multiply_by_matrix(&rotation_z)
                .multiply_by_matrix(&rotation_x);
            let vertex_b = Vec4 { x: vertex_b_1.x, y: vertex_b_1.y, z: vertex_b_1.z, w: 1.0 }
                .multiply_by_matrix(&rotation_z)
                .multiply_by_matrix(&rotation_x);
            let vertex_c = Vec4 { x: vertex_c_1.x, y: vertex_c_1.y, z: vertex_c_1.z, w: 1.0 }
                .multiply_by_matrix(&rotation_z)
                .multiply_by_matrix(&rotation_x);

            let a = Vec3 { x: vertex_a.x, y: vertex_a.y, z: vertex_a.z+i };
            let b = Vec3 { x: vertex_b.x, y: vertex_b.y, z: vertex_b.z+i };
            let c = Vec3 { x: vertex_c.x, y: vertex_c.y, z: vertex_c.z+i };
            let vertex_a_projected = project_vertex(&a, &projection_matrix);
            let vertex_b_projected = project_vertex(&b, &projection_matrix);
            let vertex_c_projected = project_vertex(&c, &projection_matrix);

            let vertex_a_view = Vec3 {
                x: vertex_a_projected.x + 1.0,
                y: vertex_a_projected.y + 1.0,
                z: 1.0,
            }
            .multiply_by_scalar(0.5)
            .multiply_by_scalar(window_width as f64);
            let vertex_b_view = Vec3 {
                x: vertex_b_projected.x + 1.0,
                y: vertex_b_projected.y + 1.0,
                z: 1.0,
            }
            .multiply_by_scalar(0.5)
            .multiply_by_scalar(window_width as f64);
            let vertex_c_view = Vec3 {
                x: vertex_c_projected.x + 1.0,
                y: vertex_c_projected.y + 1.0,
                z: 1.0,
            }
            .multiply_by_scalar(0.5)
            .multiply_by_scalar(window_width as f64);

            draw_triangle(
                (vertex_a_view.x, vertex_a_view.y).into(),
                (vertex_b_view.x, vertex_b_view.y).into(),
                (vertex_c_view.x, vertex_c_view.y).into(),
                &mut canvas,
            );
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
