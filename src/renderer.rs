use core::{convert::TryInto, mem::swap};

use nalgebra::{Matrix3, Matrix3x1, Vector2, Vector3};

use crate::{camera::Camera, eadk::{self, Rect}};

const SCREEN_WIDTH: usize = 340;
const SCREEN_HEIGHT: usize = 240;
const HALF_SCREEN_WIDTH: usize = SCREEN_WIDTH/2;
const HALF_SCREEN_HEIGHT: usize = SCREEN_HEIGHT/2;

struct Triangle2d {
    p1: Vector2<f32>,
    p2: Vector2<f32>,
    p3: Vector2<f32>,
}
struct Triangle3d {
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    p3: Vector3<f32>
}

fn get_color(r: u16, g: u16, b: u16) -> eadk::Color {
    eadk::Color { rgb565: r<<11 | g << 6 | b }
}

fn draw_line(x1: usize, y1: usize, x2: usize, y2: usize, color: eadk::Color) {
    let mut x1 = x1;
    let mut x2 = x2;
    let mut y1:isize = y1.try_into().unwrap();
    let mut y2:isize = y2.try_into().unwrap();
    if x1 > x2 {
        swap(&mut x1, &mut x2);
        swap(&mut y1, &mut y2);
    }
    let width = x2-x1;
    let height = y2-y1;
    let error = if height > width.try_into().unwrap() { (height as usize)/width } else {1};
    for i in 0..width{
        let coef = (i as f32) / (width as f32);
        eadk::display::push_rect_uniform(Rect{x: (x1+i) as u16, y: (y1+((height as f32)*coef ) as isize) as u16, width: 1, height: error as u16}, color);
    }
}

pub struct Renderer {
    camera: Camera,
}

impl Renderer {
    pub fn new() -> Self {
        let renderer: Renderer = Renderer {camera: Camera::new()};

        renderer
    }
    
    fn compute_transform(&self, point: Vector3<f32>) -> Vector2<f32> {
        let rotation_matrix: Matrix3<f32> = self.camera.get_x_rotation_matrix()
            * self.camera.get_y_rotation_matrix()
            * self.camera.get_z_rotation_matrix();
        let translation_matrix: Matrix3x1<f32> = point - self.camera.get_pos();
        let transformed_point: Matrix3x1<f32> = rotation_matrix * translation_matrix;
        let display_surface_position: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
        let projected: Matrix3x1<f32> = Matrix3::new(1.0, 0.0, display_surface_position.x/display_surface_position.z,
                                                     0.0, 1.0, display_surface_position.y/display_surface_position.z,
                                                     0.0, 0.0, 1.0/display_surface_position.z)
                                                    *transformed_point;
        
        let projected_x = projected.x * projected.z;
        let projected_y = projected.y * projected.z;

        Vector2::new(projected_x, projected_y)
    }
    fn clear_screen(color: eadk::Color) {
        eadk::display::push_rect_uniform(eadk::Rect{x: 0, y: 0, width: 340, height: 240}, color);
    }

    fn draw_2d_triangle(tri: Triangle2d) {
        draw_line(tri.p1.x as usize + HALF_SCREEN_WIDTH, tri.p1.y as usize + HALF_SCREEN_HEIGHT, tri.p2.x as usize + HALF_SCREEN_WIDTH, tri.p2.y as usize + HALF_SCREEN_HEIGHT, get_color(0b11111, 0b0, 0b0));
        draw_line(tri.p2.x as usize + HALF_SCREEN_WIDTH, tri.p2.y as usize + HALF_SCREEN_HEIGHT, tri.p3.x as usize + HALF_SCREEN_WIDTH, tri.p3.y as usize + HALF_SCREEN_HEIGHT, get_color(0b11111, 0b0, 0b0));
        draw_line(tri.p3.x as usize + HALF_SCREEN_WIDTH, tri.p3.y as usize + HALF_SCREEN_HEIGHT, tri.p1.x as usize + HALF_SCREEN_WIDTH, tri.p1.y as usize + HALF_SCREEN_HEIGHT, get_color(0b11111, 0b0, 0b0));
    }

    fn draw_3d_triangle(&self, tri: Triangle3d) {
        let projected_triangle = Triangle2d{
            p1: self.compute_transform(tri.p1),
            p2: self.compute_transform(tri.p2),
            p3: self.compute_transform(tri.p3),
        };

        Renderer::draw_2d_triangle(projected_triangle);
    }

    pub fn update(&self) {
        Renderer::clear_screen(get_color(0b01111, 0b011111, 0b11111));

        eadk::display::draw_string("Hello", eadk::Point{x: 10, y : 10}, false, get_color(0b11111, 0, 0), get_color(0b11111, 0b111111, 0b11111));
        
        //self.draw_3d_triangle(Triangle3d { p1: Vector3::new(1.0, 1.0, -3.0), p2: Vector3::new(0.0, 1.0, -3.0), p3: Vector3::new(0.0, 0.0, -3.0) });
        
        /*Renderer::draw_2d_triangle(Triangle2d {
            p1: Vector2::new(30.0, 20.0),
            p2: Vector2::new(40.0, 40.0),
            p3: Vector2::new(20.0, 60.0),
        });

        let my_point:Vector3<f32> = Vector3::new(0.0, -3.0, 1.0);
        let projected = self.compute_transform(my_point);
        eadk::display::push_rect_uniform(eadk::Rect{x: (projected.0 as usize + HALF_SCREEN_WIDTH) as u16, y: (projected.1 as usize + HALF_SCREEN_HEIGHT) as u16, width: 2, height: 2}, get_color(0, 0, 0) );*/
    }
}
