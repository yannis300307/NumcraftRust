use core::mem::swap;

use alloc::vec::Vec;
use libm::tanf;
use nalgebra::{
    Isometry3, Matrix3, Matrix3x1, Matrix4, Perspective3, Point, Point2, Point3, Projective3,
    Rotation3, Vector2, Vector3, Vector4,
};

use crate::{
    camera::Camera,
    eadk::{self, debug, Rect},
};

const PI: f32 = 3.14159265358979323846264338327950288419716939937510582;

const SCREEN_WIDTH: f32 = 320.0;
const SCREEN_HEIGHT: f32 = 240.0;
const ASPECT_RATIO: f32 = SCREEN_HEIGHT / SCREEN_WIDTH;
const HALF_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.0;
const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;
const FOV: f32 = 90.0;

const ZNEAR: f32 = 0.1;
const ZFAR: f32 = 1000.0;

const TEST_CUBE_MESH: [Triangle3d; 12] = [
    Triangle3d {
        p1: Point3::new(0.0, 0.0, 0.0),
        p2: Point3::new(0.0, 1.0, 0.0),
        p3: Point3::new(1.0, 1.0, 0.0),
    },
    Triangle3d {
        p1: Point3::new(0.0, 0.0, 0.0),
        p2: Point3::new(1.0, 1.0, 0.0),
        p3: Point3::new(1.0, 0.0, 0.0),
    },
    // EAST
    Triangle3d {
        p1: Point3::new(1.0, 0.0, 0.0),
        p2: Point3::new(1.0, 1.0, 0.0),
        p3: Point3::new(1.0, 1.0, 1.0),
    },
    Triangle3d {
        p1: Point3::new(1.0, 0.0, 0.0),
        p2: Point3::new(1.0, 1.0, 1.0),
        p3: Point3::new(1.0, 0.0, 1.0),
    },
    // NORTH
    Triangle3d {
        p1: Point3::new(1.0, 0.0, 1.0),
        p2: Point3::new(1.0, 1.0, 1.0),
        p3: Point3::new(0.0, 1.0, 1.0),
    },
    Triangle3d {
        p1: Point3::new(1.0, 0.0, 1.0),
        p2: Point3::new(0.0, 1.0, 1.0),
        p3: Point3::new(0.0, 0.0, 1.0),
    },
    // WEST
    Triangle3d {
        p1: Point3::new(0.0, 0.0, 1.0),
        p2: Point3::new(0.0, 1.0, 1.0),
        p3: Point3::new(0.0, 1.0, 0.0),
    },
    Triangle3d {
        p1: Point3::new(0.0, 0.0, 1.0),
        p2: Point3::new(0.0, 1.0, 0.0),
        p3: Point3::new(0.0, 0.0, 0.0),
    },
    // TOP
    Triangle3d {
        p1: Point3::new(0.0, 1.0, 0.0),
        p2: Point3::new(0.0, 1.0, 1.0),
        p3: Point3::new(1.0, 1.0, 1.0),
    },
    Triangle3d {
        p1: Point3::new(0.0, 1.0, 0.0),
        p2: Point3::new(1.0, 1.0, 1.0),
        p3: Point3::new(1.0, 1.0, 0.0),
    },
    // BOTTOM
    Triangle3d {
        p1: Point3::new(1.0, 0.0, 1.0),
        p2: Point3::new(0.0, 0.0, 1.0),
        p3: Point3::new(0.0, 0.0, 0.0),
    },
    Triangle3d {
        p1: Point3::new(1.0, 0.0, 1.0),
        p2: Point3::new(0.0, 0.0, 0.0),
        p3: Point3::new(1.0, 0.0, 0.0),
    },
];

#[derive(Clone, Copy)]
struct Triangle2d {
    p1: Point2<f32>,
    p2: Point2<f32>,
    p3: Point2<f32>,
}

#[derive(Clone, Copy)]
struct Triangle3d {
    p1: Point3<f32>,
    p2: Point3<f32>,
    p3: Point3<f32>,
}

fn get_color(r: u16, g: u16, b: u16) -> eadk::Color {
    eadk::Color {
        rgb565: r << 11 | g << 6 | b,
    }
}

fn draw_line(x1: isize, y1: isize, x2: isize, y2: isize, color: eadk::Color) {
    let mut x1 = x1;
    let mut x2 = x2;
    let mut y1: isize = y1;
    let mut y2: isize = y2;
    if x1 > x2 {
        swap(&mut x1, &mut x2);
        swap(&mut y1, &mut y2);
    }
    let mut width = x2 - x1;
    if width == 0 {
        width = 1
    }

    let mut height = y2 - y1;

    if height == 0 {
        height = 1
    }
    let error = if height > width { height / width } else { 1 };
    for i in 0..width {
        let coef = (i as f32) / (width as f32);
        eadk::display::push_rect_uniform(
            Rect {
                x: (x1 + i) as u16,
                y: (y1 + ((height as f32) * coef) as isize) as u16,
                width: 1,
                height: error as u16,
            },
            color,
        );
    }
}

struct MathTools {
    projection_matrix: Perspective3<f32>,
}

impl MathTools {
    pub fn new() -> Self {
        let fov_rad = 1.0 / tanf(FOV * 0.5 / 180.0 * PI);
        MathTools {
            projection_matrix: Perspective3::new(ASPECT_RATIO, PI / 4.0, 1.0, 1000.0),
            /* Matrix4::new(
                ASPECT_RATIO * fov_rad, 0.0,     0.0,                              0.0,
                0.0,                    fov_rad, 0.0,                              0.0,
                0.0,                    0.0,     ZFAR / (ZFAR - ZNEAR),            1.0,
                0.0,                    0.0,     (-ZFAR * ZNEAR) / (ZFAR - ZNEAR), 0.0,
            ),*/
        }
    }
}

pub struct Renderer {
    pub camera: Camera,
    math_tools: MathTools,
}

impl Renderer {
    pub fn new() -> Self {
        let renderer: Renderer = Renderer {
            camera: Camera::new(),
            math_tools: MathTools::new(),
        };

        renderer
    }

    fn compute_transform(&self, point: Point3<f32>) -> Point2<f32> {
        /*let rotation_matrix: Matrix3<f32> = self.camera.get_x_rotation_matrix()
            * self.camera.get_y_rotation_matrix()
            * self.camera.get_z_rotation_matrix();
        let translation_matrix: Matrix3x1<f32> = point - self.camera.get_pos();
        let transformed_point: Matrix3x1<f32> = rotation_matrix * translation_matrix;
        let display_surface_position: Vector3<f32> = Vector3::new(-1.0, 1.0, 1.0);
        let projected: Matrix3x1<f32> = Matrix3::new(
            1.0,
            0.0,
            display_surface_position.x / display_surface_position.z,
            0.0,
            1.0,
            display_surface_position.y / display_surface_position.z,
            0.0,
            0.0,
            1.0 / display_surface_position.z,
        ) * transformed_point;

        let projected_x = projected.x / projected.z;;
        let projected_y = projected.y / projected.z;*/

        let iso: Isometry3<f32> =
            Isometry3::new(*self.camera.get_pos(), *self.camera.get_rotation());

        let transformed = iso * (point - self.camera.get_pos());

        let projected = self
            .math_tools
            .projection_matrix
            .unproject_point(&transformed);

        //debug(&projected);

        Point2::new(projected.x, projected.y)
    }

    fn clear_screen(color: eadk::Color) {
        eadk::display::push_rect_uniform(
            eadk::Rect {
                x: 0,
                y: 0,
                width: 320,
                height: 240,
            },
            color,
        );
    }

    fn draw_2d_triangle(tri: Triangle2d) {
        draw_line(
            ((tri.p1.x + 1.0) * HALF_SCREEN_WIDTH) as isize,
            ((tri.p1.y + 1.0) * HALF_SCREEN_HEIGHT) as isize,
            ((tri.p2.x + 1.0) * HALF_SCREEN_WIDTH) as isize,
            ((tri.p2.y + 1.0) * HALF_SCREEN_HEIGHT) as isize,
            get_color(0b11111, 0b0, 0b0),
        );
        draw_line(
            ((tri.p2.x + 1.0) * HALF_SCREEN_WIDTH) as isize,
            ((tri.p2.y + 1.0) * HALF_SCREEN_HEIGHT) as isize,
            ((tri.p3.x + 1.0) * HALF_SCREEN_WIDTH) as isize,
            ((tri.p3.y + 1.0) * HALF_SCREEN_HEIGHT) as isize,
            get_color(0b11111, 0b0, 0b0),
        );
        draw_line(
            ((tri.p3.x + 1.0) * HALF_SCREEN_WIDTH) as isize,
            ((tri.p3.y + 1.0) * HALF_SCREEN_HEIGHT) as isize,
            ((tri.p1.x + 1.0) * HALF_SCREEN_WIDTH) as isize,
            ((tri.p1.y + 1.0) * HALF_SCREEN_HEIGHT) as isize,
            get_color(0b11111, 0b0, 0b0),
        );
    }

    fn draw_3d_triangle(&self, tri: Triangle3d) {
        let projected_triangle = Triangle2d {
            p1: self.compute_transform(tri.p1),
            p2: self.compute_transform(tri.p2),
            p3: self.compute_transform(tri.p3),
        };

        Renderer::draw_2d_triangle(projected_triangle);
    }

    pub fn draw_debug_float(value: f32) {
        let mut buf = [0u8; 64];
        let s = format_no_std::show(&mut buf, format_args!("{}", value)).unwrap();

        eadk::display::draw_string(
            s,
            eadk::Point { x: 10, y: 10 },
            false,
            get_color(0b11111, 0, 0),
            get_color(0b11111, 0b111111, 0b11111),
        );
    }

    pub fn update(&mut self) {
        //Renderer::clear_screen(get_color(0, 0, 0));

        for mut tri in TEST_CUBE_MESH {
            tri.p1.z += 3.0;
            tri.p2.z += 3.0;
            tri.p3.z += 3.0;

            self.draw_3d_triangle(tri);
        }

        self.camera.rotate(Vector3::new(0.0, 0.02, 0.0));

        //eadk::display::push_rect_uniform(eadk::Rect{ x: 10, y: 10, width: 50, height: 50 }, get_color(255, 0, 0));

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
