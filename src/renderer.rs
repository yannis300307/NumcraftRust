use cbitmap::{
    bitmap::{self, Bitmap, BitsManage},
    newmap,
};
use heapless::Vec;
use nalgebra::{
    Const, Matrix4, OPoint, Perspective3, Point2, Point3, Rotation3, Vector2, Vector3, Vector4,
};

use core::{cmp::Ordering, f32, mem::swap};

use crate::{
    camera::{self, Camera},
    eadk::{self, Color, Rect},
};

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;
const SCREEN_WIDTHF: f32 = SCREEN_WIDTH as f32;
const SCREEN_HEIGHTF: f32 = SCREEN_HEIGHT as f32;

const SCREEN_PIXELS_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
const Z_BUFFER_SIZE: usize = SCREEN_PIXELS_COUNT.div_ceil(8);
const ASPECT_RATIO: f32 = SCREEN_WIDTHF / SCREEN_HEIGHTF;
const HALF_SCREEN_WIDTH: f32 = SCREEN_WIDTHF / 2.0;
const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHTF / 2.0;
const FOV: f32 = f32::consts::PI / 4.0;

const ZNEAR: f32 = 1.0;
const ZFAR: f32 = 1000.0;

const GLOBAL_LIGHT: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

const MAX_TRIANGLES: usize = 100;

const DEFAULT_DEBUG_COLOR: Color = Color {
    rgb565: 0b1111100000000000,
};

const TEST_CUBE_MESH: [Triangle3d; 12] = [
    Triangle3d {
        p1: Vector3::new(0.0, 0.0, 0.0),
        p2: Vector3::new(0.0, 1.0, 0.0),
        p3: Vector3::new(1.0, 1.0, 0.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    Triangle3d {
        p1: Vector3::new(0.0, 0.0, 0.0),
        p2: Vector3::new(1.0, 1.0, 0.0),
        p3: Vector3::new(1.0, 0.0, 0.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    // EAST
    Triangle3d {
        p1: Vector3::new(1.0, 0.0, 0.0),
        p2: Vector3::new(1.0, 1.0, 0.0),
        p3: Vector3::new(1.0, 1.0, 1.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    Triangle3d {
        p1: Vector3::new(1.0, 0.0, 0.0),
        p2: Vector3::new(1.0, 1.0, 1.0),
        p3: Vector3::new(1.0, 0.0, 1.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    // NORTH
    Triangle3d {
        p1: Vector3::new(1.0, 0.0, 1.0),
        p2: Vector3::new(1.0, 1.0, 1.0),
        p3: Vector3::new(0.0, 1.0, 1.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    Triangle3d {
        p1: Vector3::new(1.0, 0.0, 1.0),
        p2: Vector3::new(0.0, 1.0, 1.0),
        p3: Vector3::new(0.0, 0.0, 1.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    // WEST
    Triangle3d {
        p1: Vector3::new(0.0, 0.0, 1.0),
        p3: Vector3::new(0.0, 1.0, 0.0),
        p2: Vector3::new(0.0, 1.0, 1.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    Triangle3d {
        p1: Vector3::new(0.0, 0.0, 1.0),
        p3: Vector3::new(0.0, 0.0, 0.0),
        p2: Vector3::new(0.0, 1.0, 0.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    // TOP
    Triangle3d {
        p1: Vector3::new(0.0, 1.0, 0.0),
        p2: Vector3::new(0.0, 1.0, 1.0),
        p3: Vector3::new(1.0, 1.0, 1.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    Triangle3d {
        p1: Vector3::new(0.0, 1.0, 0.0),
        p2: Vector3::new(1.0, 1.0, 1.0),
        p3: Vector3::new(1.0, 1.0, 0.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    // BOTTOM
    Triangle3d {
        p1: Vector3::new(1.0, 0.0, 1.0),
        p2: Vector3::new(0.0, 0.0, 1.0),
        p3: Vector3::new(0.0, 0.0, 0.0),
        color: DEFAULT_DEBUG_COLOR,
    },
    Triangle3d {
        p1: Vector3::new(1.0, 0.0, 1.0),
        p2: Vector3::new(0.0, 0.0, 0.0),
        p3: Vector3::new(1.0, 0.0, 0.0),
        color: DEFAULT_DEBUG_COLOR,
    },
];

#[derive(Clone, Copy, Debug)]
struct Triangle2d {
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    p3: Vector3<f32>,
    color: eadk::Color,
}

#[derive(Clone, Copy, Debug)]
struct Triangle3d {
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    p3: Vector3<f32>,
    color: eadk::Color,
}

impl Triangle3d {
    fn get_normal(&self) -> Vector3<f32> {
        let a = self.p2 - self.p1;
        let b = self.p3 - self.p1;
        a.cross(&b).normalize()
    }
}

fn get_color(r: u16, g: u16, b: u16) -> eadk::Color {
    eadk::Color {
        rgb565: r << 11 | g << 6 | b,
    }
}

fn fill_triangle(
    t0: Vector2<f32>,
    t1: Vector2<f32>,
    t2: Vector2<f32>,
    color: eadk::Color,
    z_buffer: &mut Bitmap<Z_BUFFER_SIZE>
) {
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
            let y = (t0.y as isize) + i;

            if j < 0 || j > SCREEN_WIDTH as isize || y < 0 || y > SCREEN_WIDTH as isize {
                continue;
            }

            let pix_index = (j + y * (SCREEN_WIDTH as isize)) as usize;

            if !z_buffer.get_bool(pix_index) {
                
                z_buffer.set(pix_index);

                eadk::display::push_rect_uniform(
                    Rect {
                        x: j as u16,
                        y: y as u16,
                        width: 1,
                        height: 1,
                    },
                    color,
                )
            }
        }
    }
}

fn draw_2d_triangle(tri: &Triangle3d, z_buffer: &mut Bitmap<Z_BUFFER_SIZE>) {
    fill_triangle(
        tri.p1.xy(),
        tri.p2.xy(),
        tri.p3.xy(),
        tri.color,
        z_buffer
    );

    draw_line(
        tri.p1.x as isize,
        tri.p1.y as isize,
        tri.p2.x as isize,
        tri.p2.y as isize,
        get_color(0b11111, 0b0, 0b0),
    );
    draw_line(
        tri.p2.x as isize,
        tri.p2.y as isize,
        tri.p3.x as isize,
        tri.p3.y as isize,
        get_color(0b11111, 0b0, 0b0),
    );
    draw_line(
        tri.p3.x as isize,
        tri.p3.y as isize,
        tri.p1.x as isize,
        tri.p1.y as isize,
        get_color(0b11111, 0b0, 0b0),
    );
}

fn matrix_point_at(pos: &Vector3<f32>, target: &Vector3<f32>, up: &Vector3<f32>) -> Matrix4<f32> {
    let new_forward = (target - pos).normalize();

    let new_up = (up - new_forward * up.dot(&new_forward)).normalize();
    let new_right = new_up.cross(&new_forward);

    /*Matrix4::new(
        new_right.x, new_right.y, new_right.z, 0.0,
        new_up.x, new_up.y, new_up.z, 0.0,
        new_forward.x, new_forward.y, new_forward.z, 0.0,
        pos.x, pos.y, pos.z, 1.0,
    )*/
    Matrix4::new(
        new_right.x,
        new_up.x,
        new_forward.x,
        pos.x,
        new_right.y,
        new_up.y,
        new_forward.y,
        pos.y,
        new_right.z,
        new_up.z,
        new_forward.z,
        pos.z,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

fn vector_intersect_plane(
    plane_p: &Vector3<f32>,
    plane_n: &Vector3<f32>,
    line_start: &Vector3<f32>,
    line_end: &Vector3<f32>,
) -> Vector3<f32> {
    let plane_n = plane_n.normalize();
    let plane_d = -plane_n.dot(plane_p);
    let ad = line_start.dot(&plane_n);
    let bd = line_end.dot(&plane_n);
    let t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end = line_end - line_start;
    let line_to_intersect = line_start_to_end * t;
    let coords = line_start + line_to_intersect;
    Vector3::new(coords.x, coords.y, coords.z)
}

fn triangle_clip_against_plane(
    plane_p: &Vector3<f32>,
    plane_n: &Vector3<f32>,
    in_tri: &Triangle3d,
) -> (Option<Triangle3d>, Option<Triangle3d>) {
    let plane_n = plane_n.normalize();

    let dist = |p: Vector3<f32>| {
        let n = p.normalize();
        plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - plane_n.dot(plane_p)
    };

    let binding = Default::default();
    let mut inside_points: [&Vector3<f32>; 3] = [&binding; 3];
    let mut n_inside_point_count = 0;
    let binding = Default::default();
    let mut outside_points: [&Vector3<f32>; 3] = [&binding; 3];
    let mut n_outside_point_count = 0;

    let d0 = dist(in_tri.p1);
    let d1 = dist(in_tri.p2);
    let d2 = dist(in_tri.p3);

    //println!("{}, {}, {}", d0, d1, d2);

    if d0 >= 0.0 {
        inside_points[n_inside_point_count] = &in_tri.p1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &in_tri.p1;
        n_outside_point_count += 1;
    }
    if d1 >= 0.0 {
        inside_points[n_inside_point_count] = &in_tri.p2;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &in_tri.p2;
        n_outside_point_count += 1;
    }
    if d2 >= 0.0 {
        inside_points[n_inside_point_count] = &in_tri.p3;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &in_tri.p3;
        n_outside_point_count += 1;
    }

    if n_inside_point_count == 0 {
        return (None, None);
    }

    if n_inside_point_count == 3 {
        return (Some(*in_tri), None);
    }

    if n_inside_point_count == 1 && n_outside_point_count == 2 {
        let out_tri = Triangle3d {
            p1: *inside_points[0],
            p2: vector_intersect_plane(plane_p, &plane_n, &inside_points[0], &outside_points[0]),
            p3: vector_intersect_plane(plane_p, &plane_n, &inside_points[0], &outside_points[1]),
            color: in_tri.color,
        };

        return (Some(out_tri), None);
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        let out_tri1 = Triangle3d {
            p1: *inside_points[0],
            p2: *inside_points[1],
            p3: vector_intersect_plane(&plane_p, &plane_n, &inside_points[0], &outside_points[0]),
            color: in_tri.color,
        };

        let out_tri2 = Triangle3d {
            p1: *inside_points[1],
            p2: out_tri1.p3,
            p3: vector_intersect_plane(&plane_p, &plane_n, &inside_points[1], &outside_points[0]),
            color: in_tri.color,
        };
        return (Some(out_tri1), Some(out_tri2));
    }
    (None, None)
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
    triangles_to_render: heapless::Vec<Triangle3d, MAX_TRIANGLES>,
    z_buffer: bitmap::Bitmap<Z_BUFFER_SIZE>,
}

impl Renderer {
    pub fn new() -> Self {
        let renderer: Renderer = Renderer {
            camera: Camera::new(),
            math_tools: MathTools::new(),
            triangles_to_render: heapless::Vec::new(),
            z_buffer: bitmap::Bitmap::new(),
        };

        renderer
    }

    fn project_point(&self, point: Vector3<f32>) -> Vector3<f32> {
        self.math_tools.projection_matrix.project_vector(&point) * -1.0
    }

    fn clear_screen(&mut self, color: eadk::Color) {
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                if !self.z_buffer.get_bool(x+y*SCREEN_WIDTH) {
                    eadk::display::push_rect_uniform(
                        eadk::Rect {
                            x: x as u16,
                            y: y as u16,
                            width: 1,
                            height: 1,
                        },
                        color,
                    );
                }
            }
        }
        self.z_buffer.reset_all();
        /*eadk::display::push_rect_uniform(
            eadk::Rect {
                x: 0,
                y: 0,
                width: 320,
                height: 240,
            },
            color,
        );*/
    }

    fn add_3d_triangle_to_render(&mut self, tri: Triangle3d) {
        let tri = tri;
        let rotation: Rotation3<f32> = Rotation3::new(*self.camera.get_rotation());

        let mut transformed = Triangle3d {
            p1: tri.p1,
            p2: tri.p2,
            p3: tri.p3,
            color: tri.color,
        };

        let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
        let target: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
        let look_dir = rotation.transform_vector(&target);
        let target = self.camera.get_pos() + look_dir;

        let mat_camera = matrix_point_at(self.camera.get_pos(), &target, &up)
            .try_inverse()
            .unwrap();

        let mat_view = mat_camera;

        let camera_ray = transformed.p1 - self.camera.get_pos();

        if transformed.get_normal().dot(&camera_ray) < 0.0 {
            let light = GLOBAL_LIGHT
                .normalize()
                .dot(&tri.get_normal().normalize())
                .max(0.2);

            transformed.p1 = (mat_view
                * Vector4::new(transformed.p1.x, transformed.p1.y, transformed.p1.z, 1.0))
            .xyz(); // try to_homogenous here
            transformed.p2 = (mat_view
                * Vector4::new(transformed.p2.x, transformed.p2.y, transformed.p2.z, 1.0))
            .xyz();
            transformed.p3 = (mat_view
                * Vector4::new(transformed.p3.x, transformed.p3.y, transformed.p3.z, 1.0))
            .xyz();

            let clipped_triangles = triangle_clip_against_plane(
                &Vector3::new(0.0, 0.0, 0.1),
                &Vector3::new(0.0, 0.0, 1.0),
                &transformed,
            );

            let mut project_and_add = |to_project: Triangle3d| {
                let mut projected_triangle = Triangle3d {
                    p1: self.project_point(to_project.p1),
                    p2: self.project_point(to_project.p2),
                    p3: self.project_point(to_project.p3),
                    color: get_color(
                        ((0b11111 as f32) * light) as u16,
                        ((0b111111 as f32) * light) as u16,
                        ((0b11111 as f32) * light) as u16,
                    ),
                };

                // Center
                projected_triangle.p1.x += 1.0;
                projected_triangle.p2.x += 1.0;
                projected_triangle.p3.x += 1.0;

                projected_triangle.p1.y += 1.0;
                projected_triangle.p2.y += 1.0;
                projected_triangle.p3.y += 1.0;

                // Multiply by size on screen
                projected_triangle.p1.x *= HALF_SCREEN_WIDTH;
                projected_triangle.p2.x *= HALF_SCREEN_WIDTH;
                projected_triangle.p3.x *= HALF_SCREEN_WIDTH;

                projected_triangle.p1.y *= HALF_SCREEN_HEIGHT;
                projected_triangle.p2.y *= HALF_SCREEN_HEIGHT;
                projected_triangle.p3.y *= HALF_SCREEN_HEIGHT;

                self.triangles_to_render.push(projected_triangle).unwrap();
            };

            if let Some(clipped) = clipped_triangles.0 {
                project_and_add(clipped)
            }
            if let Some(clipped) = clipped_triangles.1 {
                project_and_add(clipped)
            }
        }
    }

    #[allow(unused)]
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
            .sort_by(|tri1: &Triangle3d, tri2: &Triangle3d| -> Ordering {
                let z1 = (tri1.p1.z + tri1.p2.z + tri1.p3.z) / 3.0;
                let z2 = (tri2.p1.z + tri2.p2.z + tri2.p3.z) / 3.0;

                z1.partial_cmp(&z2).unwrap()
            });

        for tri in self.triangles_to_render.iter() {
            let mut clip_buffer: heapless::Deque<Triangle3d, 16> = heapless::Deque::new(); // 2^4

            clip_buffer.push_back(*tri).unwrap();
            let mut new_tris = 1;

            let mut clip_triangle = |plane_p, plane_n| {
                while new_tris > 0 {
                    let test = clip_buffer.pop_front().unwrap();
                    new_tris -= 1;

                    let clipped = triangle_clip_against_plane(&plane_p, &plane_n, &test);

                    if let Some(clipped_tri) = clipped.0 {
                        clip_buffer.push_back(clipped_tri).unwrap();
                    }
                    if let Some(clipped_tri) = clipped.1 {
                        clip_buffer.push_back(clipped_tri).unwrap();
                    }
                }
                new_tris = clip_buffer.len();
            };

            clip_triangle(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
            clip_triangle(
                Vector3::new(0.0, SCREEN_HEIGHTF - 1.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0),
            );
            clip_triangle(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
            clip_triangle(
                Vector3::new(SCREEN_WIDTHF - 1.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
            );
            
            while !clip_buffer.is_empty() {
                let tri_to_draw = clip_buffer.pop_front().unwrap();
                draw_2d_triangle(&tri_to_draw, &mut self.z_buffer);
            }
        }
    }

    pub fn update(&mut self) {
        

        self.triangles_to_render.clear();
        for tri in TEST_CUBE_MESH {
            self.add_3d_triangle_to_render(tri);
        }

        self.draw_triangles();

        self.clear_screen(get_color(0, 0, 0));

        //self.camera.rotate(Vector3::new(0.0, 0.01, 0.0));
        //self.camera.translate(Vector3::new(0.01, 0.0, 0.0));
    }
}
