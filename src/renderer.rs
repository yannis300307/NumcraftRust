#[cfg(target_os = "none")]
use alloc::format;

#[cfg(target_os = "none")]
use alloc::vec::Vec;

use nalgebra::{Matrix4, Perspective3, Vector2, Vector3, Vector4};

use core::{cmp::Ordering, f32, mem::swap};

use crate::{
    camera::Camera,
    constants::{
        get_quad_color_from_texture_id,
        menu::{
            MENU_BACKGROUND_COLOR, MENU_ELEMENT_BACKGROUND_COLOR,
            MENU_ELEMENT_BACKGROUND_COLOR_HOVER, MENU_OUTLINE_COLOR, MENU_TEXT_COLOR,
        },
        rendering::*,
        world::CHUNK_SIZE,
    },
    eadk::{
        self, Color, Rect,
        display::{push_rect_uniform, wait_for_vblank},
    },
    frustum::Frustum,
    menu::{Menu, MenuElement, TextAnchor},
    mesh::{Quad, SmallTriangle2D, Triangle, Triangle2D},
    player::Player,
    world::World,
};

// Screen size related constants

const SCREEN_WIDTHF: f32 = SCREEN_WIDTH as f32;
const SCREEN_HEIGHTF: f32 = SCREEN_HEIGHT as f32;
const HALF_SCREEN_WIDTHF: f32 = SCREEN_WIDTHF / 2.0;
const HALF_SCREEN_HEIGHTF: f32 = SCREEN_HEIGHTF / 2.0;
const HALF_SCREEN: Vector2<f32> = Vector2::new(HALF_SCREEN_WIDTHF, HALF_SCREEN_HEIGHTF);

// Screen tiling constants
const SCREEN_TILE_WIDTH: usize = SCREEN_WIDTH.div_ceil(SCREEN_TILE_SUBDIVISION);
const SCREEN_TILE_HEIGHT: usize = SCREEN_HEIGHT.div_ceil(SCREEN_TILE_SUBDIVISION);

// Projection parameters
const ASPECT_RATIO: f32 = SCREEN_WIDTHF / SCREEN_HEIGHTF;

const ZNEAR: f32 = 1.0;
const ZFAR: f32 = 1000.0;

// Other
const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

static FONT_DATA: &[u8] = include_bytes!("../target/font.bin");
const FONT_WIDTH: usize = 1045;
const FONT_HEIGHT: usize = 15;

static CROSS_DATA: &[u8] = include_bytes!("../target/cross.bin");
const CROSS_WIDTH: usize = 14;
const CROSS_HEIGHT: usize = 14;

const FONT_CHAR_WIDTH: usize = 11;
static FONT_ORDER: &str = "!\"_$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^+`abcdefghijklmnopqrstuvwxyz{|}~€";

/// Fill a triangle in the frame buffer
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
    let triangle_heightf = triangle_height as f32;

    'height_iter: for i in 0..triangle_height {
        let second_half = i > (t1.y - t0.y) || (t1.y == t0.y);
        let segment_heightf = if second_half {
            (t2.y - t1.y) as f32
        } else {
            (t1.y - t0.y) as f32
        };

        let alpha = i as f32 / triangle_heightf;
        let beta = if second_half {
            (i as f32 - (t1.y - t0.y) as f32) / segment_heightf
        } else {
            i as f32 / segment_heightf
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

        if (b as usize) < 1 {
            // prevent line bug
            continue;
        }

        for j in (a as usize)..=(b as usize) {
            if j >= SCREEN_TILE_WIDTH {
                continue 'height_iter;
            }
            frame_buffer[j + y as usize * SCREEN_TILE_WIDTH] = color;
        }
    }
}

// Draw a line in the frame buffer
fn draw_line(
    pos1: (isize, isize),
    pos2: (isize, isize),
    frame_buffer: &mut [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    color: Color,
) {
    for point in bresenham::Bresenham::new(pos1, pos2) {
        if point.0 >= 0
            && point.0 < SCREEN_TILE_WIDTH as isize
            && point.1 >= 0
            && point.1 < SCREEN_TILE_HEIGHT as isize
        {
            frame_buffer[(point.0 + point.1 * SCREEN_TILE_WIDTH as isize) as usize] = color;
        }
    }
}

// Takes a Triangle2D and draw it as a filled triangle or lines depending of the texture_id
fn draw_2d_triangle(
    tri: &Triangle2D,
    frame_buffer: &mut [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
) {
    if tri.texture_id == 255 {
        // Block marker
        draw_line(
            (tri.p1.x as isize, tri.p1.y as isize),
            (tri.p2.x as isize, tri.p2.y as isize),
            frame_buffer,
            Color::from_components(0b11111, 0b0, 0b0),
        );
        draw_line(
            (tri.p2.x as isize, tri.p2.y as isize),
            (tri.p3.x as isize, tri.p3.y as isize),
            frame_buffer,
            Color::from_components(0b11111, 0b0, 0b0),
        );
    } else {
        // Normal Triangle
        fill_triangle(
            Vector2::new(tri.p1.x as isize, tri.p1.y as isize),
            Vector2::new(tri.p2.x as isize, tri.p2.y as isize),
            Vector2::new(tri.p3.x as isize, tri.p3.y as isize),
            frame_buffer,
            get_quad_color_from_texture_id(tri.texture_id).apply_light(tri.light * 17),
        );
    }
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
    line_start + line_to_intersect
}

fn vector_intersect_line(
    line_p: &Vector2<f32>,
    line_n: &Vector2<f32>,
    line_start: &Vector2<f32>,
    line_end: &Vector2<f32>,
) -> Vector2<i16> {
    let line_n = line_n.normalize();
    let line_d = -line_n.dot(line_p);
    let ad = line_start.dot(&line_n);
    let bd = line_end.dot(&line_n);
    let t = (-line_d - ad) / (bd - ad);
    let line_start_to_end = line_end - line_start;
    let line_to_intersect = line_start_to_end * t;
    let coords = line_start + line_to_intersect;
    coords.map(|x| x as i16)
}

fn triangle_clip_against_line(
    line_p: &Vector2<f32>,
    line_n: &Vector2<f32>,
    in_tri: &Triangle2D,
) -> (Option<Triangle2D>, Option<Triangle2D>) {
    let line_n = line_n.normalize();

    let dist = |p: Vector2<f32>| line_n.x * p.x + line_n.y * p.y - line_n.dot(line_p);

    let binding = Default::default();
    let mut inside_points: [&Vector2<f32>; 3] = [&binding; 3];
    let mut n_inside_point_count = 0;
    let mut outside_points: [&Vector2<f32>; 3] = [&binding; 3];
    let mut n_outside_point_count = 0;

    let p1 = in_tri.p1.map(|x| x as f32);
    let p2 = in_tri.p2.map(|x| x as f32);
    let p3 = in_tri.p3.map(|x| x as f32);

    let d0 = dist(p1);
    let d1 = dist(p2);
    let d2 = dist(p3);

    if d0 >= 0.0 {
        inside_points[n_inside_point_count] = &p1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &p1;
        n_outside_point_count += 1;
    }
    if d1 >= 0.0 {
        inside_points[n_inside_point_count] = &p2;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &p2;
        n_outside_point_count += 1;
    }
    if d2 >= 0.0 {
        inside_points[n_inside_point_count] = &p3;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &p3;
        n_outside_point_count += 1;
    }

    if n_inside_point_count == 0 {
        return (None, None);
    }

    if n_inside_point_count == 3 {
        return (Some(*in_tri), None);
    }

    if n_inside_point_count == 1 && n_outside_point_count == 2 {
        let out_tri = Triangle2D {
            p1: inside_points[0].map(|x| x as i16),
            p2: vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[0]),
            p3: vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[1]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };

        return (Some(out_tri), None);
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        let out_tri1 = Triangle2D {
            p1: inside_points[0].map(|x| x as i16),
            p2: inside_points[1].map(|x| x as i16),
            p3: vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[0]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };

        let out_tri2 = Triangle2D {
            p1: inside_points[1].map(|x| x as i16),
            p2: out_tri1.p3,
            p3: vector_intersect_line(line_p, &line_n, inside_points[1], outside_points[0]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        return (Some(out_tri1), Some(out_tri2));
    }
    (None, None)
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
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };

        return (Some(out_tri), None);
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        let out_tri1 = Triangle {
            p1: *inside_points[0],
            p2: *inside_points[1],
            p3: vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[0]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };

        let out_tri2 = Triangle {
            p1: *inside_points[1],
            p2: out_tri1.p3,
            p3: vector_intersect_plane(plane_p, &plane_n, inside_points[1], outside_points[0]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        return (Some(out_tri1), Some(out_tri2));
    }
    (None, None)
}

pub struct Renderer {
    pub camera: Camera,
    triangles_to_render: Vec<SmallTriangle2D>,
    tile_frame_buffer: [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    projection_matrix: Perspective3<f32>,
    pub enable_vsync: bool,
}

impl Renderer {
    pub fn new() -> Self {
        let renderer: Renderer = Renderer {
            camera: Camera::new(),
            projection_matrix: Perspective3::new(ASPECT_RATIO, FOV, ZNEAR, ZFAR),
            triangles_to_render: Vec::with_capacity(MAX_TRIANGLES),
            tile_frame_buffer: [Color { rgb565: 0 }; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
            enable_vsync: true,
        };

        renderer
    }

    pub fn update_fov(&mut self, new_fov: f32) {
        self.camera.set_fov(new_fov);
        self.projection_matrix =
            Perspective3::new(ASPECT_RATIO, self.camera.get_fov(), ZNEAR, ZFAR);
    }

    fn project_point(&self, point: Vector3<f32>) -> Vector2<f32> {
        self.projection_matrix.project_vector(&point).xy() * -1.0
    }

    fn clear_screen(&mut self, color: eadk::Color) {
        self.tile_frame_buffer.fill(color);
    }

    fn get_mat_view(&self) -> Matrix4<f32> {
        let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
        let target: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
        let look_dir = self.camera.get_rotation_matrix() * target.to_homogeneous();
        let target = self.camera.get_pos() + look_dir.xyz();

        let mat_camera = matrix_point_at(self.camera.get_pos(), &target, &up);

        mat_camera.try_inverse().unwrap()
    }

    fn add_3d_triangle_to_render(&mut self, tri: Triangle, mat_view: &Matrix4<f32>) {
        let mut tri = tri;

        let camera_ray = tri.p1 - self.camera.get_pos();

        if tri.get_normal().dot(&camera_ray) < 0.0 {
            tri.p1 = (mat_view * Vector4::new(tri.p1.x, tri.p1.y, tri.p1.z, 1.0)).xyz(); // try to_homogenous here
            tri.p2 = (mat_view * Vector4::new(tri.p2.x, tri.p2.y, tri.p2.z, 1.0)).xyz();
            tri.p3 = (mat_view * Vector4::new(tri.p3.x, tri.p3.y, tri.p3.z, 1.0)).xyz();

            let clipped_triangles: (Option<Triangle>, Option<Triangle>) = if tri.texture_id != 255 {
                triangle_clip_against_plane(
                    &Vector3::new(0.0, 0.0, 0.1),
                    &Vector3::new(0.0, 0.0, 1.0),
                    &tri,
                )
            } else {
                (Some(tri), None)
            };

            let mut project_and_add = |to_project: Triangle| {
                let projected_triangle = Triangle2D {
                    p1: ((self.project_point(to_project.p1) + Vector2::new(1., 1.))
                        .component_mul(&HALF_SCREEN))
                    .map(|x| x as i16),
                    p2: ((self.project_point(to_project.p2) + Vector2::new(1., 1.))
                        .component_mul(&HALF_SCREEN))
                    .map(|x| x as i16),
                    p3: ((self.project_point(to_project.p3) + Vector2::new(1., 1.))
                        .component_mul(&HALF_SCREEN))
                    .map(|x| x as i16),
                    texture_id: to_project.texture_id,
                    light: to_project.light,
                };

                let mut clip_buffer: heapless::Deque<Triangle2D, 16> = heapless::Deque::new(); // 2^4

                clip_buffer.push_back(projected_triangle).unwrap();
                let mut new_tris = 1;

                let mut clip_triangle = |line_p, line_n| {
                    while new_tris > 0 {
                        let test = clip_buffer.pop_front().unwrap();
                        new_tris -= 1;

                        let clipped = triangle_clip_against_line(&line_p, &line_n, &test);

                        if let Some(clipped_tri) = clipped.0 {
                            clip_buffer.push_back(clipped_tri).unwrap();
                        }
                        if let Some(clipped_tri) = clipped.1 {
                            clip_buffer.push_back(clipped_tri).unwrap();
                        }
                    }
                    new_tris = clip_buffer.len();
                };

                if tri.texture_id != 255 {
                    clip_triangle(Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0));
                    clip_triangle(Vector2::new(0.0, SCREEN_HEIGHTF), Vector2::new(0.0, -1.0));
                    clip_triangle(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
                    clip_triangle(
                        Vector2::new(SCREEN_WIDTHF - 1.0, 0.0),
                        Vector2::new(-1.0, 0.0),
                    );
                }

                for tri in clip_buffer {
                    self.triangles_to_render.push(tri.to_small()); // Do nothing if overflow
                }
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
        let tile_offset = Vector2::new(
            -((SCREEN_TILE_WIDTH * tile_x) as i16),
            -((SCREEN_TILE_HEIGHT * tile_y) as i16),
        );
        for tri in self.triangles_to_render.iter_mut() {
            let mut tri_copy = tri.to_tri_2d();
            tri_copy.p1 += tile_offset;

            tri_copy.p2 += tile_offset;

            tri_copy.p3 += tile_offset;

            draw_2d_triangle(&tri_copy, &mut self.tile_frame_buffer);
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
                        Color::from_888(pixel_value as u16, pixel_value as u16, pixel_value as u16);

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

    fn add_quad_to_render(
        &mut self,
        quad: &Quad,
        mat_view: &Matrix4<f32>,
        chunk_block_pos: Vector3<isize>,
    ) {
        let quad_triangles = quad.get_triangles(chunk_block_pos);
        self.add_3d_triangle_to_render(quad_triangles.0, mat_view);
        self.add_3d_triangle_to_render(quad_triangles.1, mat_view);
    }

    pub fn draw_image_negate(
        &mut self,
        image: &[u8],
        image_size: Vector2<isize>,
        pos: Vector2<isize>,
    ) {
        for y in 0..image_size.y {
            if pos.y + y < 0 || pos.y + y >= SCREEN_TILE_HEIGHT as isize {
                continue;
            }
            for x in 0..image_size.x {
                let dest = Vector2::new(x, y) + pos;

                if dest.x < 0 || dest.x >= SCREEN_TILE_WIDTH as isize {
                    continue;
                }

                let pixel = image[(x + image_size.x * y) as usize];

                if pixel == 0 {
                    let frame_buff_index = (dest.x + dest.y * SCREEN_TILE_WIDTH as isize) as usize;
                    let components = self.tile_frame_buffer[frame_buff_index].get_components();

                    let inverted_color = Color::from_components(
                        0b11111 - components.0,
                        0b111111 - components.1,
                        0b11111 - components.2,
                    );
                    self.tile_frame_buffer[frame_buff_index] = inverted_color;
                }
            }
        }
    }

    fn draw_ui(&mut self, fps_count: f32, tile_x: usize, tile_y: usize) {
        if tile_x == 0 && tile_y == 0 {
            self.draw_string(
                format!("FPS:{fps_count:.2}").as_str(),
                &Vector2::new(10, 10),
            );

            self.draw_string(
                format!("Tris:{}", self.triangles_to_render.len()).as_str(),
                &Vector2::new(10, 30),
            );

            self.draw_string(
                format!(
                    "{:.1},{:.1},{:.1}",
                    self.camera.get_pos().x,
                    self.camera.get_pos().y,
                    self.camera.get_pos().z
                )
                .as_str(),
                &Vector2::new(10, 50),
            );
        }
        let mut draw_cross = |x, y| {
            self.draw_image_negate(
                CROSS_DATA,
                Vector2::new(CROSS_WIDTH as isize, CROSS_HEIGHT as isize),
                Vector2::new(x, y),
            );
        };

        if tile_x == 0 && tile_y == 0 {
            draw_cross(
                (SCREEN_TILE_WIDTH - CROSS_WIDTH / 2) as isize,
                (SCREEN_TILE_HEIGHT - CROSS_HEIGHT / 2) as isize,
            )
        }
        if tile_x == 1 && tile_y == 0 {
            draw_cross(
                -((CROSS_WIDTH / 2) as isize),
                (SCREEN_TILE_HEIGHT - CROSS_HEIGHT / 2) as isize,
            )
        }
        if tile_x == 1 && tile_y == 1 {
            draw_cross(
                -((CROSS_WIDTH / 2) as isize),
                -((CROSS_HEIGHT / 2) as isize),
            );
        }
        if tile_x == 0 && tile_y == 1 {
            draw_cross(
                (SCREEN_TILE_WIDTH - CROSS_WIDTH / 2) as isize,
                -((CROSS_HEIGHT / 2) as isize),
            );
        }
    }

    pub fn draw_game(&mut self, world: &mut World, player: &Player, fps_count: f32) {
        self.triangles_to_render.clear();

        let mat_view = self.get_mat_view();

        let frustum = Frustum::new(
            &self.camera,
            ASPECT_RATIO,
            self.camera.get_fov(),
            ZNEAR,
            ZFAR,
        );

        for chunk in world.get_chunks_sorted_by_distance(*self.camera.get_pos()) {
            let chunk_blocks_pos = chunk.get_pos() * CHUNK_SIZE_I;
            let chunk_blocks_posf = chunk_blocks_pos.map(|x| x as f32);
            let chunk_blocks_pos_maxf =
                (chunk_blocks_pos + Vector3::repeat(CHUNK_SIZE_I)).map(|x| x as f32);

            if !(frustum.is_aabb_in_frustum(chunk_blocks_posf, chunk_blocks_pos_maxf)) {
                continue;
            }

            let need_sorting = chunk.need_sorting || self.camera.get_has_moved();

            let quads = chunk.get_mesh().get_reference_vec();

            if need_sorting {
                quads.sort_by(|a, b| -> Ordering {
                    let a_pos = a.get_pos().map(|x| x as isize) + chunk_blocks_pos;
                    let b_pos = b.get_pos().map(|x| x as isize) + chunk_blocks_pos;
                    let avec = Vector3::new(
                        a_pos.x as f32 + 0.5,
                        a_pos.y as f32 + 0.5,
                        a_pos.z as f32 + 0.5,
                    );

                    let bvec = Vector3::new(
                        b_pos.x as f32 + 0.5,
                        b_pos.y as f32 + 0.5,
                        b_pos.z as f32 + 0.5,
                    );

                    bvec.metric_distance(self.camera.get_pos())
                        .total_cmp(&avec.metric_distance(self.camera.get_pos()))
                });
            }
            for quad in quads {
                self.add_quad_to_render(quad, &mat_view, chunk_blocks_pos);
            }
        }

        // Finally add the player block marker
        let mut block_marker = player.get_block_marker();
        for quad in block_marker.0.get_reference_vec() {
            self.add_quad_to_render(quad, &mat_view, block_marker.1);
        }

        for x in 0..SCREEN_TILE_SUBDIVISION {
            for y in 0..SCREEN_TILE_SUBDIVISION {
                self.clear_screen(Color::from_components(0b01110, 0b110110, 0b11111));
                self.draw_triangles(x, y);

                self.draw_ui(fps_count, x, y);

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
        if self.enable_vsync {
            eadk::display::wait_for_vblank();
        }
    }

    pub fn draw_menu(&self, menu: &mut Menu) {
        if !menu.need_redraw {
            return;
        }

        menu.need_redraw = false;

        let mut element_y = menu.pos.y;

        let elements = menu.get_elements();
        for i in 0..elements.len() {
            let element = &elements[i];

            let default_rect = Rect {
                x: menu.pos.x as u16,
                y: element_y as u16,
                width: menu.width as u16,
                height: 30,
            };

            let draw_outline = || {
                push_rect_uniform(
                    Rect {
                        x: default_rect.x - 1,
                        y: default_rect.y - 1,
                        width: default_rect.width + 2,
                        height: 1,
                    },
                    MENU_OUTLINE_COLOR,
                );
                push_rect_uniform(
                    Rect {
                        x: default_rect.x - 1,
                        y: default_rect.y + default_rect.height,
                        width: default_rect.width + 2,
                        height: 1,
                    },
                    MENU_OUTLINE_COLOR,
                );
                push_rect_uniform(
                    Rect {
                        x: default_rect.x - 1,
                        y: default_rect.y,
                        width: 1,
                        height: default_rect.height,
                    },
                    MENU_OUTLINE_COLOR,
                );
                push_rect_uniform(
                    Rect {
                        x: default_rect.x + default_rect.width,
                        y: default_rect.y,
                        width: 1,
                        height: default_rect.height,
                    },
                    MENU_OUTLINE_COLOR,
                );
            };

            let element_bg_color = if i == menu.selected_index {
                MENU_ELEMENT_BACKGROUND_COLOR_HOVER
            } else {
                MENU_ELEMENT_BACKGROUND_COLOR
            };

            match element {
                MenuElement::Button { text, .. } => {
                    push_rect_uniform(default_rect, element_bg_color);
                    draw_outline();
                    let text_y = menu.pos.x + (menu.width - 10 * text.len()) / 2;
                    eadk::display::draw_string(
                        text,
                        eadk::Point {
                            x: text_y as u16,
                            y: (element_y + 6) as u16,
                        },
                        true,
                        MENU_TEXT_COLOR,
                        element_bg_color,
                    );
                }
                MenuElement::Slider { text_fn, value, .. } => {
                    push_rect_uniform(default_rect, element_bg_color);
                    let text = text_fn(*value);
                    let cursor_width = 20;
                    let x_pos =
                        default_rect.x + (value * (menu.width - cursor_width - 4) as f32) as u16;
                    let text_y = menu.pos.x + (menu.width - 10 * text.len()) / 2;
                    eadk::display::draw_string(
                        text.as_str(),
                        eadk::Point {
                            x: text_y as u16,
                            y: (element_y + 6) as u16,
                        },
                        true,
                        MENU_TEXT_COLOR,
                        element_bg_color,
                    );
                    push_rect_uniform(
                        Rect {
                            x: x_pos + 2,
                            y: default_rect.y + 2,
                            width: 20,
                            height: default_rect.height - 4,
                        },
                        Color::from_888(255, 255, 255),
                    );
                    draw_outline();
                }
                MenuElement::Label {
                    text, text_anchor, ..
                } => {
                    let text_y = match text_anchor {
                        TextAnchor::Left => menu.pos.x + 10,
                        TextAnchor::Center => menu.pos.x + (menu.width - 10 * text.len()) / 2,
                        TextAnchor::Right => menu.pos.x + menu.width - 10 * text.len() - 10,
                    };
                    eadk::display::draw_string(
                        text,
                        eadk::Point {
                            x: text_y as u16,
                            y: (element_y + 6) as u16,
                        },
                        true,
                        MENU_TEXT_COLOR,
                        MENU_BACKGROUND_COLOR,
                    );
                }
                MenuElement::Void { .. } => {}
            }

            element_y += if matches!(
                element,
                MenuElement::Label {
                    allow_margin: true,
                    ..
                } | MenuElement::Button {
                    allow_margin: true,
                    ..
                } | MenuElement::Slider {
                    allow_margin: true,
                    ..
                } | MenuElement::Void {
                    allow_margin: true,
                    ..
                }
            ) {
                40
            } else {
                30
            };
        }

        wait_for_vblank();
    }
}
