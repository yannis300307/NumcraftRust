use nalgebra::{Const, OPoint, Perspective3, Point2, Point3, Rotation3, Vector2, Vector3};

use core::{cmp::Ordering, f32, mem::swap};

use crate::{
    camera::Camera,
    eadk::{self, Color, Rect},
};

const SCREEN_WIDTH: f32 = 320.0;
const SCREEN_HEIGHT: f32 = 240.0;
const ASPECT_RATIO: f32 = SCREEN_HEIGHT / SCREEN_WIDTH;
const HALF_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.0;
const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;
const FOV: f32 = f32::consts::PI / 4.0;

const ZNEAR: f32 = 1.0;
const ZFAR: f32 = 1000.0;

const GLOBAL_LIGHT: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

const MAX_TRIANGLES: usize = 100;

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
        p2: Point3::new(0.0, 1.0, 0.0),
        p3: Point3::new(0.0, 1.0, 1.0),
    },
    Triangle3d {
        p1: Point3::new(0.0, 0.0, 1.0),
        p2: Point3::new(0.0, 0.0, 0.0),
        p3: Point3::new(0.0, 1.0, 0.0),
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

#[derive(Clone, Copy, Debug)]
struct Triangle2d {
    p1: Point3<f32>,
    p2: Point3<f32>,
    p3: Point3<f32>,
    color: eadk::Color,
}

#[derive(Clone, Copy)]
struct Triangle3d {
    p1: Point3<f32>,
    p2: Point3<f32>,
    p3: Point3<f32>,
}

impl Triangle3d {
    fn get_normal(&self) -> Vector3<f32> {
        let a = self.p2 - self.p1;
        a.cross(&(self.p3 - self.p1))
    }
}

fn get_color(r: u16, g: u16, b: u16) -> eadk::Color {
    eadk::Color {
        rgb565: r << 11 | g << 6 | b,
    }
}

fn vector_intersect_plane(
    plane_p: &Vector3<f32>,
    plane_n: &Vector3<f32>,
    line_start: &Vector3<f32>,
    line_end: &Vector3<f32>,
) -> Point3<f32> {
    let plane_n = plane_n.normalize();
    let plane_d = -plane_n.dot(plane_p);
    let ad = line_start.dot(&plane_n);
    let bd = line_end.dot(&plane_n);
    let t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end = line_end - line_start;
    let line_to_intersect = line_start_to_end * t;
    let coords = line_start + line_to_intersect;
    Point3::new(coords.x, coords.y, coords.z)
}

fn triangle_clip_against_plane(
    plane_p: &Vector3<f32>,
    plane_n: &Vector3<f32>,
    mut in_tri: &Triangle3d,
) -> (usize, Option<Triangle3d>, Option<Triangle3d>) {
    let plane_n = plane_n.normalize();

    let dist = |p: Point3<f32>| {
        let n = p.coords.normalize();
        plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - plane_n.dot(plane_p)
    };

    let binding = Default::default();
    let mut inside_points: [&Point3<f32>; 3] = [&binding; 3];
    let mut n_inside_point_count = 0;
    let binding = Default::default();
    let mut outside_points: [&Point3<f32>; 3] = [&binding; 3];
    let mut n_outside_point_count = 0;

    let d0 = dist(in_tri.p1);
    let d1 = dist(in_tri.p2);
    let d2 = dist(in_tri.p3);

    if (d0 >= 0.0) {
        inside_points[n_inside_point_count] = &in_tri.p1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &in_tri.p1;
        n_outside_point_count += 1;
    }
    if (d1 >= 0.0) {
        inside_points[n_inside_point_count] = &in_tri.p2;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &in_tri.p2;
        n_outside_point_count += 1;
    }
    if (d2 >= 0.0) {
        inside_points[n_inside_point_count] = &in_tri.p3;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &in_tri.p3;
        n_outside_point_count += 1;
    }

    if (n_inside_point_count == 0) {
        return (0, None, None);
    }

    if (n_inside_point_count == 3) {
        return (1, Some(*in_tri), None);
    }

    if (n_inside_point_count == 1 && n_outside_point_count == 2) {
        let out_tri = Triangle3d {
            p1: *inside_points[0],
            p2: vector_intersect_plane(
                &plane_p,
                &plane_n,
                &inside_points[0].coords,
                &outside_points[0].coords,
            ),
            p3: vector_intersect_plane(
                plane_p,
                &plane_n,
                &inside_points[0].coords,
                &outside_points[1].coords,
            ),
        };

        return (1, Some(out_tri), None);
    }

    if (n_inside_point_count == 2 && n_outside_point_count == 1) {
        let out_tri1 = Triangle3d {
            p1: *inside_points[0],
            p2: *inside_points[1],
            p3: vector_intersect_plane(
                &plane_p,
                &plane_n,
                &inside_points[0].coords,
                &outside_points[0].coords,
            ),
        };

        let out_tri2 = Triangle3d {
            p1: *inside_points[1],
            p2: out_tri1.p3,
            p3: vector_intersect_plane(
                &plane_p,
                &plane_n,
                &inside_points[1].coords,
                &outside_points[0].coords,
            ),
        };
        return (2, Some(out_tri1), Some(out_tri2));
    }
    (0, None, None)
}

fn draw_line(x1: isize, y1: isize, x2: isize, y2: isize, color: eadk::Color) {
    let mut x1 = x1;
    let mut y1 = y1;
    let mut x2 = x2;
    let mut y2 = y2;

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        eadk::display::push_rect_uniform(
            Rect {
                x: x1 as u16,
                y: y1 as u16,
                width: 1,
                height: 1,
            },
            color,
        );

        if x1 == x2 && y1 == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x1 += sx;
        }
        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }
}

fn fill_triangle(t0: Vector2<f32>, t1: Vector2<f32>, t2: Vector2<f32>, color: eadk::Color) {
    let mut t0 = t0;
    let mut t1 = t1;
    let mut t2 = t2;
    if t0.y == t1.y && t0.y == t2.y {
        return;
    }; // I dont care about degenerate triangles
       // sort the vertices, t0, t1, t2 lower−to−upper (bubblesort yay!)
    if t0.y > t1.y {
        swap(&mut t0, &mut t1)
    };
    if t0.y > t2.y {
        swap(&mut t0, &mut t2)
    };
    if t1.y > t2.y {
        swap(&mut t1, &mut t2)
    };
    let total_height = t2.y - t0.y;
    for i in 0..(total_height as isize) {
        let second_half = i > ((t1.y - t0.y) as isize) || t1.y == t0.y;
        let segment_height = if second_half {
            t2.y - t1.y
        } else {
            t1.y - t0.y
        };
        let alpha = (i as f32) / total_height;
        let beta = (i as f32 - (if second_half { t1.y - t0.y } else { 0.0 })) / segment_height; // be careful: with above conditions no division by zero here
        let mut a: Vector2<f32> = t0 + (t2 - t0) * alpha;
        let mut b: Vector2<f32> = if second_half {
            t1 + (t2 - t1) * beta
        } else {
            t0 + (t1 - t0) * beta
        };
        if a.x > b.x {
            swap(&mut a, &mut b)
        };
        for j in (a.x as isize)..(b.x as isize) {
            eadk::display::push_rect_uniform(
                Rect {
                    x: j as u16,
                    y: ((t0.y as isize) + i) as u16,
                    width: 1,
                    height: 1,
                },
                color,
            )
        }
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
    triangles_to_render: heapless::Vec<Triangle2d, MAX_TRIANGLES>,
}

impl Renderer {
    pub fn new() -> Self {
        let renderer: Renderer = Renderer {
            camera: Camera::new(),
            math_tools: MathTools::new(),
            triangles_to_render: heapless::Vec::new(),
        };

        renderer
    }

    fn project_point(&self, point: OPoint<f32, Const<3>>) -> Point3<f32> {
        self.math_tools.projection_matrix.unproject_point(&point)
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

    fn draw_2d_triangle(tri: &Triangle2d) {
        /*draw_line(
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
        );*/

        fill_triangle(
            Vector2::new(
                (tri.p1.x + 1.0) * HALF_SCREEN_WIDTH,
                (tri.p1.y + 1.0) * HALF_SCREEN_HEIGHT,
            ),
            Vector2::new(
                (tri.p2.x + 1.0) * HALF_SCREEN_WIDTH,
                (tri.p2.y + 1.0) * HALF_SCREEN_HEIGHT,
            ),
            Vector2::new(
                (tri.p3.x + 1.0) * HALF_SCREEN_WIDTH,
                (tri.p3.y + 1.0) * HALF_SCREEN_HEIGHT,
            ),
            tri.color,
        );
    }

    fn add_3d_triangle_to_render(&mut self, tri: Triangle3d) {
        let rotation: Rotation3<f32> = Rotation3::new(*self.camera.get_rotation());

        let transformed = Triangle3d {
            p1: rotation.inverse_transform_point(&(tri.p1 - self.camera.get_pos())),
            p2: rotation.inverse_transform_point(&(tri.p2 - self.camera.get_pos())),
            p3: rotation.inverse_transform_point(&(tri.p3 - self.camera.get_pos())),
        };

        if self
            .camera
            .get_pos()
            .normalize()
            .dot(&transformed.get_normal().normalize())
            > 0.0
        {
            let light = GLOBAL_LIGHT
                .normalize()
                .dot(&tri.get_normal().normalize())
                .max(0.2);

            let clipped: [Triangle3d; 2];
            let n_clipped_triangles = triangle_clip_against_plane(
                &Vector3::new(0.0, 0.0, 0.1),
                &Vector3::new(0.0, 0.0, 1.0),
                &transformed,
            );

            let projected_triangle = Triangle2d {
                p1: self.project_point(transformed.p1),
                p2: self.project_point(transformed.p2),
                p3: self.project_point(transformed.p3),
                color: get_color(
                    ((0b11111 as f32) * light) as u16,
                    ((0b111111 as f32) * light) as u16,
                    ((0b11111 as f32) * light) as u16,
                ),
            };

            self.triangles_to_render.push(projected_triangle).unwrap();
        }
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

    fn draw_triangles(&mut self) {
        self.triangles_to_render
            .sort_by(|tri1: &Triangle2d, tri2: &Triangle2d| -> Ordering {
                let z1 = (tri1.p1.z + tri1.p2.z + tri1.p3.z) / 3.0;
                let z2 = (tri2.p1.z + tri2.p2.z + tri2.p3.z) / 3.0;

                z1.partial_cmp(&z2).unwrap()
            });

        for tri in &self.triangles_to_render {
            Renderer::draw_2d_triangle(tri);
        }
    }

    pub fn update(&mut self) {
        Renderer::clear_screen(get_color(0, 0, 0));

        self.triangles_to_render.clear();
        for tri in TEST_CUBE_MESH {
            self.add_3d_triangle_to_render(tri);
        }

        self.draw_triangles();

        //self.camera.rotate(Vector3::new(0.0, 0.1, 0.0));
    }
}
