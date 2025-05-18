use alloc::vec::Vec;
use cbitmap::bitmap::{self, Bitmap, BitsManage};
use nalgebra::{Matrix4, Perspective3, Vector2, Vector3, Vector4};

use core::{cmp::Ordering, f32, mem::swap};

use crate::{
    camera::Camera,
    constants::rendering::*,
    eadk::{self, Color, Rect},
    mesh::{Quad, Triangle},
};

// Screen size related constants

const SCREEN_WIDTHF: f32 = SCREEN_WIDTH as f32;
const SCREEN_HEIGHTF: f32 = SCREEN_HEIGHT as f32;

// Screen tiling constants
const SCREEN_TILE_WIDTH: usize = SCREEN_WIDTH.div_ceil(SCREEN_TILE_SUBDIVISION);
const SCREEN_TILE_HEIGHT: usize = SCREEN_HEIGHT.div_ceil(SCREEN_TILE_SUBDIVISION);

const HALF_SCREEN_TILE_WIDTH: f32 = SCREEN_WIDTH as f32 / 2.0;
const HALF_SCREEN_TILE_HEIGHT: f32 = SCREEN_HEIGHT as f32 / 2.2;

// z_buffer constants
const SCREEN_PIXELS_COUNT: usize = SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT;
const Z_BUFFER_SIZE: usize = SCREEN_PIXELS_COUNT.div_ceil(8);

// Projection parameters
const ASPECT_RATIO: f32 = SCREEN_WIDTHF / SCREEN_HEIGHTF;

const ZNEAR: f32 = 1.0;
const ZFAR: f32 = 1000.0;

// Other
const GLOBAL_LIGHT: Vector3<f32> = Vector3::new(0.5, 0.0, -1.0);

static FONT_DATA: &[u8] = include_bytes!("../target/font.bin");
const FONT_WIDTH: usize = 1045;
const FONT_HEIGHT: usize = 15;
const FONT_CHAR_WIDTH: usize = 11;
static FONT_ORDER: &str = "!\" $%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^+`abcdefghijklmnopqrstuvwxyz{|}~€";

#[inline]
fn get_color(r: u16, g: u16, b: u16) -> eadk::Color {
    eadk::Color {
        rgb565: r << 11 | g << 5 | b,
    }
}

fn rgb565_from_888(r: u16, g: u16, b: u16) -> Color {
    Color {
        rgb565: ((r & 0b11111000) << 8) | ((g & 0b11111100) << 3) | (b >> 3),
    }
}

fn fill_triangle(
    mut t0: Vector2<isize>,
    mut t1: Vector2<isize>,
    mut t2: Vector2<isize>,
    frame_buffer: &mut [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    color: Color,
) {
    if t0.y > t1.y {
        swap(&mut t0, &mut t1);
    }
    if t0.y > t2.y {
        swap(&mut t0, &mut t2);
    }
    if t1.y > t2.y {
        swap(&mut t1, &mut t2);
    }

    let triangle_height = t2.y - t0.y;

    'height_iter: for i in 0..triangle_height as isize {
        let second_half = i > (t1.y - t0.y) as isize || (t1.y == t0.y);
        let segment_height = if second_half {
            t2.y - t1.y
        } else {
            t1.y - t0.y
        };

        let alpha = i as f32 / triangle_height as f32;
        let beta = if second_half {
            (i as f32 - (t1.y - t0.y) as f32) / segment_height as f32
        } else {
            i as f32 / segment_height as f32
        };

        let mut a = t0.x as f32 + ((t2 - t0).x as f32 * alpha);
        let mut b = if second_half {
            t1.x as f32 + ((t2 - t1).x as f32 * beta)
        } else {
            t0.x as f32 + ((t1 - t0).x as f32 * beta)
        };

        if a > b {
            swap(&mut a, &mut b);
        }


        let y = t0.y + i;
        if y < 0 {
            continue 'height_iter;
        }
        if y >= SCREEN_TILE_HEIGHT as isize {
            break 'height_iter;
        }

        if (b as usize) < 1 { // prevent line bug
            continue;
        }

        for j in (a as usize)..=(b as usize) {
            if j >= SCREEN_TILE_WIDTH {continue 'height_iter;}
            frame_buffer[(j + y as usize * SCREEN_TILE_WIDTH) as usize] = color;
        }
    }
}

fn draw_2d_triangle(
    tri: &Triangle,
    frame_buffer: &mut [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
) {
    fill_triangle(
        Vector2::new(tri.p1.x as isize, tri.p1.y as isize),
        Vector2::new(tri.p2.x as isize, tri.p2.y as isize),
        Vector2::new(tri.p3.x as isize, tri.p3.y as isize),
        frame_buffer,
        tri.color,
    );
}

fn matrix_point_at(pos: &Vector3<f32>, target: &Vector3<f32>, up: &Vector3<f32>) -> Matrix4<f32> {
    let new_forward = (target - pos).normalize();

    let new_up = (up - new_forward * up.dot(&new_forward)).normalize();
    let new_right = new_up.cross(&new_forward);

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
    in_tri: &Triangle,
) -> (Option<Triangle>, Option<Triangle>) {
    let plane_n = plane_n.normalize();

    let dist = |p: Vector3<f32>| {
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
        let out_tri = Triangle {
            p1: *inside_points[0],
            p2: vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[0]),
            p3: vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[1]),
            color: in_tri.color,
        };

        return (Some(out_tri), None);
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        let out_tri1 = Triangle {
            p1: *inside_points[0],
            p2: *inside_points[1],
            p3: vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[0]),
            color: in_tri.color,
        };

        let out_tri2 = Triangle {
            p1: *inside_points[1],
            p2: out_tri1.p3,
            p3: vector_intersect_plane(plane_p, &plane_n, inside_points[1], outside_points[0]),
            color: in_tri.color,
        };
        return (Some(out_tri1), Some(out_tri2));
    }
    (None, None)
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
    triangles_to_render: heapless::Vec<Triangle, MAX_TRIANGLES>,
    tile_frame_buffer: [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
}

impl Renderer {
    pub fn new() -> Self {
        let renderer: Renderer = Renderer {
            camera: Camera::new(),
            math_tools: MathTools::new(),
            triangles_to_render: heapless::Vec::new(),
            tile_frame_buffer: [Color { rgb565: 0 }; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
        };

        renderer
    }

    fn project_point(&self, point: Vector3<f32>) -> Vector3<f32> {
        self.math_tools.projection_matrix.project_vector(&point) * -1.0
    }

    fn clear_screen(&mut self, color: eadk::Color) {
        self.tile_frame_buffer.fill(color);
    }

    fn add_3d_triangle_to_render(&mut self, tri: Triangle) {
        let mut tri = tri;
        let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
        let target: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
        let look_dir = self.camera.get_rotation_matrix() * target.to_homogeneous();
        let target = self.camera.get_pos() + look_dir.xyz();

        let mat_camera = matrix_point_at(self.camera.get_pos(), &target, &up);

        let mat_view = mat_camera.try_inverse().unwrap();

        let camera_ray = tri.p1 - self.camera.get_pos();

        if tri.get_normal().dot(&camera_ray) < 0.0 {
            let light = GLOBAL_LIGHT
                .normalize()
                .dot(&tri.get_normal().normalize())
                .max(0.2);

            tri.p1 = (mat_view * Vector4::new(tri.p1.x, tri.p1.y, tri.p1.z, 1.0)).xyz(); // try to_homogenous here
            tri.p2 = (mat_view * Vector4::new(tri.p2.x, tri.p2.y, tri.p2.z, 1.0)).xyz();
            tri.p3 = (mat_view * Vector4::new(tri.p3.x, tri.p3.y, tri.p3.z, 1.0)).xyz();

            let clipped_triangles = triangle_clip_against_plane(
                &Vector3::new(0.0, 0.0, 0.1),
                &Vector3::new(0.0, 0.0, 1.0),
                &tri,
            );

            let mut project_and_add = |to_project: Triangle| {
                let mut projected_triangle = Triangle {
                    p1: self.project_point(to_project.p1),
                    p2: self.project_point(to_project.p2),
                    p3: self.project_point(to_project.p3),
                    color: get_color(
                        ((0b11111 as f32) * light) as u16,
                        ((0b111111 as f32) * light) as u16, // Why is my white green?
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
                projected_triangle.p1.x *= HALF_SCREEN_TILE_WIDTH;
                projected_triangle.p2.x *= HALF_SCREEN_TILE_WIDTH;
                projected_triangle.p3.x *= HALF_SCREEN_TILE_WIDTH;

                projected_triangle.p1.y *= HALF_SCREEN_TILE_HEIGHT;
                projected_triangle.p2.y *= HALF_SCREEN_TILE_HEIGHT;
                projected_triangle.p3.y *= HALF_SCREEN_TILE_HEIGHT;

                let _ = self.triangles_to_render.push(projected_triangle); // Do nothing if overflow
            };

            if let Some(clipped) = clipped_triangles.0 {
                project_and_add(clipped)
            }
            if let Some(clipped) = clipped_triangles.1 {
                project_and_add(clipped)
            }
        }
    }

    fn draw_triangles(&mut self, tile_x: usize, tile_y: usize) {
        self.triangles_to_render
            .sort_by(|tri1: &Triangle, tri2: &Triangle| -> Ordering {
                let z1 = (tri1.p1.z + tri1.p2.z + tri1.p3.z) / 3.0;
                let z2 = (tri2.p1.z + tri2.p2.z + tri2.p3.z) / 3.0;

                z1.partial_cmp(&z2).unwrap()
            });

        for tri in self.triangles_to_render.iter_mut() {
            let mut clip_buffer: heapless::Deque<Triangle, 16> = heapless::Deque::new(); // 2^4

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
                let mut tri_to_draw = clip_buffer.pop_front().unwrap();

                tri_to_draw.p1.x -= (SCREEN_TILE_WIDTH * tile_x) as f32;
                tri_to_draw.p1.y -= (SCREEN_TILE_HEIGHT * tile_y) as f32;

                tri_to_draw.p2.x -= (SCREEN_TILE_WIDTH * tile_x) as f32;
                tri_to_draw.p2.y -= (SCREEN_TILE_HEIGHT * tile_y) as f32;

                tri_to_draw.p3.x -= (SCREEN_TILE_WIDTH * tile_x) as f32;
                tri_to_draw.p3.y -= (SCREEN_TILE_HEIGHT * tile_y) as f32;

                draw_2d_triangle(
                    &tri_to_draw,
                    &mut self.tile_frame_buffer,
                );
            }
        }
    }

    fn draw_string(&mut self, text: &str, pos: &Vector2<usize>) {
        let mut text_cursor: usize = 0;
        for char in text.chars() {
            let font_index = FONT_ORDER.chars().position(|c| c == char).unwrap();

            let font_pixel_index = font_index * FONT_CHAR_WIDTH;

            for x in 0..FONT_CHAR_WIDTH {
                for y in 0..FONT_HEIGHT {
                    let pixel_value = FONT_DATA[(font_pixel_index + x) + y * FONT_WIDTH];

                    let rgb565 =
                        rgb565_from_888(pixel_value as u16, pixel_value as u16, pixel_value as u16);

                    let pix_x = pos.x + x + text_cursor;

                    if pix_x > SCREEN_TILE_WIDTH {
                        continue;
                    }

                    self.tile_frame_buffer[pix_x + (pos.y + y) * SCREEN_TILE_WIDTH] = rgb565;
                }
            }
            text_cursor += FONT_CHAR_WIDTH;
        }
    }

    fn add_quad_to_render(&mut self, quad: &Quad) {
        let quad_triangles = quad.get_triangles();
        self.add_3d_triangle_to_render(quad_triangles.0);
        self.add_3d_triangle_to_render(quad_triangles.1);
    }

    pub fn update(&mut self, world_mesh: &Vec<&Vec<Quad>>, fps_count: f32) {
        self.triangles_to_render.clear();

        let mut quad_count: usize = 0;

        for chunk_mech in world_mesh.iter() {
            for quad in chunk_mech.iter() {
                self.add_quad_to_render(quad);
                quad_count += 1;
            }
        }

        for x in 0..SCREEN_TILE_SUBDIVISION {
            for y in 0..SCREEN_TILE_SUBDIVISION {
                self.clear_screen(get_color(0b01110, 0b110110, 0b11111));
                self.draw_triangles(x, y);

                if x == 0 && y == 0 {
                    self.draw_string(
                        alloc::format!("FPS:{fps_count:.2}").as_str(),
                        &Vector2::new(10, 10),
                    );

                    self.draw_string(
                        alloc::format!("Quads:{quad_count}").as_str(),
                        &Vector2::new(10, 30),
                    );
                }

                eadk::display::push_rect(
                    Rect {
                        x: (SCREEN_TILE_WIDTH * x) as u16,
                        y: (SCREEN_TILE_HEIGHT * y) as u16,
                        width: SCREEN_TILE_WIDTH as u16,
                        height: SCREEN_TILE_HEIGHT as u16,
                    },
                    &self.tile_frame_buffer,
                );
            }
        }
    }
}
