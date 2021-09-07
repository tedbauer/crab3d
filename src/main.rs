extern crate sdl2;

mod geometry;

use crate::sdl2::gfx::primitives::DrawRenderer;
use geometry::{rotate_x, rotate_z, Matrix4x4, Triangle, Vec3, Vec4, cube};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::{self, BufRead};
use std::time::Duration;
use std::cmp::Ordering;

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

fn draw_triangle(
    a: Point2d,
    b: Point2d,
    c: Point2d,
    canvas: &mut Canvas<Window>,
    color: Color,
) -> () {
    //canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.filled_trigon(
        a.x as i16, a.y as i16, b.x as i16, b.y as i16, c.x as i16, c.y as i16, color,
    );
    //canvas.draw_line(Point::from(&a), Point::from(&b));
    //canvas.draw_line(Point::from(&b), Point::from(&c));
    //canvas.draw_line(Point::from(&c), Point::from(&a));
}

fn gen_mesh(obj_file: Lines<BufReader<File>>) -> Vec<Triangle> {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    for line in obj_file {
        let line = line.unwrap();
        let line_input_type = line
            .split(' ')
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .clone();

        if line_input_type.eq(&"v".to_string()) {
            let vertex_line = line.split(' ').skip(1).collect::<Vec<&str>>();
            let a = vertex_line.get(0).unwrap().parse::<f64>().unwrap();
            let b = vertex_line.get(1).unwrap().parse::<f64>().unwrap();
            let c = vertex_line.get(2).unwrap().parse::<f64>().unwrap();

            vertices.push(Vec3::from((a, b, c)));
        } else if line_input_type.eq(&"f".to_string()) {
            let triangle_line = line.split(' ').skip(1).collect::<Vec<&str>>();
            let a_index = triangle_line.get(0).unwrap().parse::<usize>().unwrap();
            let b_index = triangle_line.get(1).unwrap().parse::<usize>().unwrap();
            let c_index = triangle_line.get(2).unwrap().parse::<usize>().unwrap();

            triangles.push(Triangle(
                vertices.get(a_index - 1).unwrap().clone(),
                vertices.get(b_index - 1).unwrap().clone(),
                vertices.get(c_index - 1).unwrap().clone(),
            ));
        }
    }

    triangles
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

    // BLAH
    let lines = io::BufReader::new(File::open("models/teapot.obj").unwrap()).lines();
    let mut mesh = gen_mesh(lines);
    mesh.sort_by(|triangle_a, triangle_b| {
        let z_midpoint = |triangle: &Triangle| {
            (triangle.0.z + triangle.1.z + triangle.2.z) / 3.0
        };
        if z_midpoint(triangle_a) < z_midpoint(triangle_b) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    //let mesh = cube();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut i = 10.0;
    let mut f_theta: f64 = 0.0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let camera = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut light_x = 0.0;

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

        for triangle in &mesh {
            let rotated_translated_triangle = triangle.transform(|vertex: &Vec3| {
                let rotated_triangle = Vec4::from((vertex.x, vertex.y, vertex.z, 1.0))
                    .multiply_by_matrix(&rotate_z(f_theta))
                    .multiply_by_matrix(&rotate_x(f_theta));

                Vec3::from((rotated_triangle.x, rotated_triangle.y, rotated_triangle.z))
                    .add(&(0.0, 0.0, i).into())
            });

            //todo: can we do this without a clone()?
            let Triangle(vertex_a, vertex_b, vertex_c) = rotated_translated_triangle.clone();

            let triangle_normal = vertex_a
                .minus(&vertex_b)
                .cross(&vertex_a.minus(&vertex_c))
                .normalized();

            if triangle_normal.dot(&vertex_a.minus(&camera)) < 0.0 {
                let light_direction = Vec3 {
                    x: light_x,
                    y: 0.0,
                    z: -1.0,
                }
                .normalized();
                let dot_prod = triangle_normal.dot(&light_direction);

                let Triangle(view_a, view_b, view_c) =
                    rotated_translated_triangle.transform(|vertex| {
                        let projected: Vec4 = Vec4::from((vertex.x, vertex.y, vertex.z, 1.0))
                            .multiply_by_matrix(&projection_matrix);
                        let projected: Vec3 = if projected.w != 0.0 {
                            (
                                projected.x / projected.w,
                                projected.y / projected.w,
                                projected.z / projected.w,
                            )
                                .into()
                        } else {
                            projected.into()
                        };

                        projected
                            .add(&(1.0, 1.0, 1.0).into())
                            .multiply_by_scalar(0.5)
                            .multiply_by_scalar(window_width as f64)
                    });

                draw_triangle(
                    (view_a.x, view_a.y).into(),
                    (view_b.x, view_b.y).into(),
                    (view_c.x, view_c.y).into(),
                    &mut canvas,
                    Color::RGB(
                        (dot_prod * 255.0) as u8,
                        (dot_prod * 255.0) as u8,
                        (dot_prod * 255.0) as u8,
                    ),
                );
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
