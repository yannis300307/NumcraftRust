use core::mem::swap;

use nalgebra::{
    Isometry3, Perspective3, Point2, Point3, Vector3,
};

use core::f32;

use crate::{
    camera::Camera,
    eadk::{self, debug, Rect},
};

const SCREEN_WIDTH: f32 = 320.0;
const SCREEN_HEIGHT: f32 = 240.0;
const ASPECT_RATIO: f32 = SCREEN_HEIGHT / SCREEN_WIDTH;
const HALF_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.0;
const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;
const FOV: f32 = f32::consts::PI / 4.0;

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
        MathTools {
            projection_matrix: Perspective3::new(ASPECT_RATIO, FOV, ZNEAR, ZFAR),
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
        let iso: Isometry3<f32> =
            Isometry3::new(*self.camera.get_pos(), *self.camera.get_rotation());

        let transformed = iso * (point - self.camera.get_pos());

        let projected = self
            .math_tools
            .projection_matrix
            .unproject_point(&transformed);

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
    }
}
