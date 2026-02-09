use crate::{
    constants::get_quad_color_from_texture_id,
    hud::Hud,
    nadk::display::{ScreenRect, push_rect, wait_for_vblank},
    player::Player,
    renderer::{
        frustum::Frustum,
        mesh::{Quad, Triangle, Triangle2D},
        *,
    },
    world::World,
};

use libm::ceilf;
use nalgebra::{max, min};

pub fn scanline_loop(
    range: &mut [Vector2<f32>; 2],
    d_dy: [Vector2<f32>; 2],
    y_start: u8,
    y_end: u8,
    color: Color565,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f32; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
) {
    let fwidth: i16 = SCREEN_TILE_WIDTH as i16;
    let mut x_start: u16;
    let mut x_end: u16;
    let mut i_y: usize = SCREEN_TILE_WIDTH * y_start as usize;
    let mut i: usize;

    let mut dz_dx: f32;
    let mut z: f32;

    for _y in y_start..y_end {
        x_start = min(max(range[0].x as i16, 0), fwidth) as u16;
        x_end = max(min(ceilf(range[1].x) as i16, fwidth), 0) as u16;

        dz_dx = (range[1].y - range[0].y) / (range[1].x - range[0].x) as f32;
        z = range[0].y + dz_dx * (x_start as f32 - range[0].x);
        i = i_y + x_start as usize;
        for _x in x_start..x_end {
            if z < depth_buffer[i] {
                frame_buffer[i] = color;
                depth_buffer[i] = z;
            }
            i += 1;
            z += dz_dx;
        }

        range[0] += d_dy[0];
        range[1] += d_dy[1];
        i_y += SCREEN_TILE_WIDTH;
    }
}

/// Fill a triangle in the frame buffer
pub fn fill_triangle(
    tri: &Triangle2D,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f32; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    color: Color565,
) {
    let mut point1 = (tri.p1.x, tri.p1.y, tri.t1.z);
    let mut point2 = (tri.p2.x, tri.p2.y, tri.t2.z);
    let mut point3 = (tri.p3.x, tri.p3.y, tri.t3.z);

    if point1.1 > point2.1 {
        swap(&mut point1, &mut point2);
    }
    if point1.1 > point3.1 {
        swap(&mut point1, &mut point3);
    }
    if point2.1 > point3.1 {
        swap(&mut point2, &mut point3);
    }

    let mut d_dy: [Vector2<f32>; 2] = [Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)];
    let mut range: [Vector2<f32>; 2] = [Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)];
    let mut y_start: u8;
    let mut y_end: u8;

    if point1.1 < SCREEN_TILE_HEIGHT as i16 && point2.1 >= 0 {
        y_start = max(point1.1, 0) as u8;
        y_end = min(point2.1, SCREEN_TILE_HEIGHT as i16) as u8;
        d_dy[0].x = (point2.0 - point1.0) as f32;
        d_dy[0].y = (point2.2 - point1.2) as f32;
        d_dy[1].x = (point3.0 - point1.0) as f32;
        d_dy[1].y = (point3.2 - point1.2) as f32;
        d_dy[0] /= (point2.1 - point1.1) as f32;
        d_dy[1] /= (point3.1 - point1.1) as f32;
        range[0].x = point1.0 as f32 + d_dy[0].x * (y_start as i16 - point1.1) as f32;
        range[1].x = point1.0 as f32 + d_dy[1].x * (y_start as i16 - point1.1) as f32;
        range[0].y = point1.2 as f32 + d_dy[0].y * (y_start as i16 - point1.1) as f32;
        range[1].y = point1.2 as f32 + d_dy[1].y * (y_start as i16 - point1.1) as f32;

        if d_dy[0].x > d_dy[1].x {
            d_dy.swap(0, 1);
            range.swap(0, 1);
        }
        scanline_loop(
            &mut range,
            d_dy,
            y_start,
            y_end,
            color,
            frame_buffer,
            depth_buffer,
        );
    }

    if point2.1 < SCREEN_TILE_HEIGHT as i16 && point3.1 >= 0 {
        y_start = max(point2.1, 0) as u8;
        y_end = min(point3.1, SCREEN_TILE_HEIGHT as i16) as u8;
        d_dy[0].x = (point3.0 - point1.0) as f32;
        d_dy[0].y = (point3.2 - point1.2) as f32;
        d_dy[1].x = (point3.0 - point2.0) as f32;
        d_dy[1].y = (point3.2 - point2.2) as f32;
        d_dy[0] /= (point3.1 - point1.1) as f32;
        d_dy[1] /= (point3.1 - point2.1) as f32;
        range[0].x = point1.0 as f32 + d_dy[0].x * (y_start as i16 - point1.1) as f32;
        range[1].x = point2.0 as f32 + d_dy[1].x * (y_start as i16 - point2.1) as f32;
        range[0].y = point1.2 as f32 + d_dy[0].y * (y_start as i16 - point1.1) as f32;
        range[1].y = point2.2 as f32 + d_dy[1].y * (y_start as i16 - point2.1) as f32;

        if d_dy[0].x < d_dy[1].x {
            d_dy.swap(0, 1);
            range.swap(0, 1);
        }
        scanline_loop(
            &mut range,
            d_dy,
            y_start,
            y_end,
            color,
            frame_buffer,
            depth_buffer,
        );
    }
}

fn textured_triangle(
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    mut point1: Vector2<i16>,
    mut tex1: Vector3<f32>,
    mut point2: Vector2<i16>,
    mut tex2: Vector3<f32>,
    mut point3: Vector2<i16>,
    mut tex3: Vector3<f32>,
) {
    if point2.y < point1.y {
        swap(&mut point1, &mut point2);
        swap(&mut tex1, &mut tex2);
    }

    if point3.y < point1.y {
        swap(&mut point1, &mut point3);
        swap(&mut tex1, &mut tex3);
    }

    if point3.y < point2.y {
        swap(&mut point2, &mut point3);
        swap(&mut tex2, &mut tex3);
    }

    let mut dpoint1 = point2 - point1;
    let mut dtex1 = tex2 - tex1;

    let dpoint2 = point3 - point1;
    let dtex2 = tex3 - tex1;

    let mut tex_coords;

    let mut dax_step = 0.0;
    let mut dbx_step = 0.0;
    let mut dtex1_step = Vector3::repeat(0.0);
    let mut dtex2_step = Vector3::repeat(0.0);

    if dpoint1.y != 0 {
        dax_step = dpoint1.x as f32 / dpoint1.y.abs() as f32;
    }
    if dpoint2.y != 0 {
        dbx_step = dpoint2.x as f32 / dpoint2.y.abs() as f32;
    }

    if dpoint1.y != 0 {
        dtex1_step = dtex1 / (dpoint1.y.abs() as f32);
    }
    if dpoint2.y != 0 {
        dtex2_step = dtex2 / (dpoint2.y.abs() as f32);
    }

    if dpoint1.y != 0 {
        for i in point1.y..=point2.y {
            if i >= SCREEN_TILE_HEIGHT as i16 || i < 0 {
                continue;
            }
            let mut ax = (point1.x as f32 + (i - point1.y) as f32 * dax_step) as i16;
            let mut bx = (point1.x as f32 + (i - point1.y) as f32 * dbx_step) as i16;

            let mut tex_s = tex1 + (i - point1.y) as f32 * dtex1_step;
            let mut tex_e = tex1 + (i - point1.y) as f32 * dtex2_step;

            if ax > bx {
                swap(&mut ax, &mut bx);
                swap(&mut tex_s, &mut tex_e);
            }

            let tstep = 1.0 / ((bx - ax) as f32);
            let mut t = 0.0;

            for j in ax..bx {
                if j >= SCREEN_TILE_WIDTH as i16 || j < 0 {
                    break;
                }
                tex_coords = (1.0 - t) * tex_s + t * tex_e;
                let index = (i * SCREEN_TILE_WIDTH as i16 + j) as usize;
                let z_inv = 1.0 / tex_coords.z;
                if tex_coords.z < depth_buffer[index] as f32 {
                    let texture_pixel_index = ((((tex_coords.x * z_inv) * 8.0) as usize)
                        + ((tex_coords.y * z_inv * 8.0) as usize) * 128)
                        * 2;
                    let pixel = u16::from_be_bytes([
                        *unsafe { TILESET_DATA.get_unchecked(texture_pixel_index) },
                        *unsafe { TILESET_DATA.get_unchecked(texture_pixel_index + 1) },
                    ]);
                    unsafe { *frame_buffer.get_unchecked_mut(index) = Color565 { value: pixel } };
                    unsafe { *depth_buffer.get_unchecked_mut(index) = tex_coords.z as f16; };
                }
                t += tstep;
            }
        }
    }

    dpoint1 = point3 - point2;
    dtex1 = tex3 - tex2;

    if dpoint1.y != 0 {
        dax_step = dpoint1.x as f32 / dpoint1.y.abs() as f32;
    }
    if dpoint2.y != 0 {
        dbx_step = dpoint2.x as f32 / dpoint2.y.abs() as f32;
    }

    dtex1_step.x = 0.0;
    dtex1_step.y = 0.0;
    if dpoint1.y != 0 {
        dtex1_step = dtex1 / (dpoint1.y.abs() as f32);
    }

    if dpoint1.y != 0 {
        for i in point2.y..point3.y {
            if i >= SCREEN_TILE_HEIGHT as i16 || i < 0 {
                continue;
            }
            let mut ax = (point2.x as f32 + (i - point2.y) as f32 * dax_step) as i16;
            let mut bx = (point1.x as f32 + (i - point1.y) as f32 * dbx_step) as i16;

            let mut tex_s = tex2 + (i - point2.y) as f32 * dtex1_step;
            let mut tex_e = tex1 + (i - point1.y) as f32 * dtex2_step;

            if ax > bx {
                swap(&mut ax, &mut bx);
                swap(&mut tex_s, &mut tex_e);
            }

            let tstep = 1.0 / ((bx - ax) as f32);
            let mut t = 0.0;

            for j in ax..bx {
                if j >= SCREEN_TILE_WIDTH as i16 || j < 0 {
                    break;
                }
                tex_coords = (1.0 - t) * tex_s + t * tex_e;
                let index = (i * SCREEN_TILE_WIDTH as i16 + j) as usize;
                let z_inv = 1.0 / tex_coords.z;
                if tex_coords.z < depth_buffer[index] as f32 {
                    let texture_pixel_index = ((((tex_coords.x * z_inv) * 8.0) as usize)
                        + ((tex_coords.y * z_inv * 8.0) as usize) * 128)
                        * 2;
                    let pixel = u16::from_be_bytes([
                        *unsafe { TILESET_DATA.get_unchecked(texture_pixel_index) },
                        *unsafe { TILESET_DATA.get_unchecked(texture_pixel_index + 1) },
                    ]);
                    unsafe { *frame_buffer.get_unchecked_mut(index) = Color565 { value: pixel } };
                    unsafe { *depth_buffer.get_unchecked_mut(index) = tex_coords.z as f16; };
                }
                t += tstep;
            }
        }
    }
}

// Draw a line in the frame buffer
pub fn draw_line(
    pos1: (isize, isize),
    pos2: (isize, isize),
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    color: Color565,
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

fn draw_2d_triangles(
    tri: &Triangle2D,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
) {
    if tri.texture_id == 255 {
        // Block marker
        draw_line(
            (tri.p1.x as isize, tri.p1.y as isize),
            (tri.p2.x as isize, tri.p2.y as isize),
            frame_buffer,
            Color565::new(0b11111, 0b0, 0b0),
        );
        draw_line(
            (tri.p2.x as isize, tri.p2.y as isize),
            (tri.p3.x as isize, tri.p3.y as isize),
            frame_buffer,
            Color565::new(0b11111, 0b0, 0b0),
        );
    } else {
        // Normal Triangle
        textured_triangle(
            frame_buffer,
            depth_buffer,
            tri.p1,
            tri.t1,
            tri.p2,
            tri.t2,
            tri.p3,
            tri.t3,
            //get_quad_color_from_texture_id(tri.texture_id).apply_light(tri.light * 17),
        );
        /*draw_line(
            (tri.p1.x as isize, tri.p1.y as isize),
            (tri.p2.x as isize, tri.p2.y as isize),
            frame_buffer,
            Color565::new(0b11111, 0b0, 0b0),
        );
        draw_line(
            (tri.p2.x as isize, tri.p2.y as isize),
            (tri.p3.x as isize, tri.p3.y as isize),
            frame_buffer,
            Color565::new(0b11111, 0b0, 0b0),
        );
        draw_line(
            (tri.p3.x as isize, tri.p3.y as isize),
            (tri.p1.x as isize, tri.p1.y as isize),
            frame_buffer,
            Color565::new(0b11111, 0b0, 0b0),
        );*/
    }
}

// Takes a Triangle2D and draw it as a filled triangle or lines depending of the texture_id
pub fn clip_and_draw_2d_triangle(
    tri: Triangle2D,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
) {
    let mut clip_buffer: heapless::Deque<Triangle2D, 16> = heapless::Deque::new(); // 2^4

    clip_buffer.push_back(tri).unwrap();
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
        clip_triangle(
            Vector2::new(0.0, SCREEN_TILE_HEIGHT as f32),
            Vector2::new(0.0, -1.0),
        );
        clip_triangle(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
        clip_triangle(
            Vector2::new(SCREEN_TILE_WIDTH as f32, 0.0),
            Vector2::new(-1.0, 0.0),
        );
    }

    for cliped_tri in clip_buffer {
        draw_2d_triangles(&cliped_tri, frame_buffer, depth_buffer);
    }
}

pub fn matrix_point_at(
    pos: &Vector3<f32>,
    target: &Vector3<f32>,
    up: &Vector3<f32>,
) -> Matrix4<f32> {
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

pub fn vector_intersect_plane(
    plane_p: &Vector3<f32>,
    plane_n: &Vector3<f32>,
    line_start: &Vector3<f32>,
    line_end: &Vector3<f32>,
) -> (Vector3<f32>, f32) {
    let plane_n = plane_n.normalize();
    let plane_d = -plane_n.dot(plane_p);
    let ad = line_start.dot(&plane_n);
    let bd = line_end.dot(&plane_n);
    let t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end = line_end - line_start;
    let line_to_intersect = line_start_to_end * t;
    (line_start + line_to_intersect, t)
}

pub fn vector_intersect_line(
    line_p: &Vector2<f32>,
    line_n: &Vector2<f32>,
    point_start: &Vector2<f32>,
    point_end: &Vector2<f32>,
) -> (Vector2<i16>, f32) {
    let line_n = line_n.normalize();
    let line_d = -line_n.dot(line_p);
    let ad = point_start.dot(&line_n);
    let bd = point_end.dot(&line_n);
    let t = (-line_d - ad) / (bd - ad);
    let point_start_to_end = point_end - point_start;
    let point_to_intersect = point_start_to_end * t;
    let coords = point_start + point_to_intersect;
    (coords.map(|x| x as i16), t)
}

pub fn triangle_clip_against_line(
    line_p: &Vector2<f32>,
    line_n: &Vector2<f32>,
    in_tri: &Triangle2D,
) -> (Option<Triangle2D>, Option<Triangle2D>) {
    let line_n = line_n.normalize();

    let dist = |p: Vector2<f32>| line_n.x * p.x + line_n.y * p.y - line_n.dot(line_p);

    let default = Default::default();
    let mut inside_points: [&Vector2<f32>; 3] = [&default; 3];
    let mut n_inside_point_count = 0;
    let mut outside_points: [&Vector2<f32>; 3] = [&default; 3];
    let mut n_outside_point_count = 0;

    let default = Default::default();
    let mut inside_tex: [&Vector3<f32>; 3] = [&default; 3];
    let mut n_inside_tex_count = 0;
    let mut outside_tex: [&Vector3<f32>; 3] = [&default; 3];
    let mut n_outside_tex_count = 0;

    let p1 = Vector2::new(in_tri.p1.x as f32, in_tri.p1.y as f32);
    let p2 = Vector2::new(in_tri.p2.x as f32, in_tri.p2.y as f32);
    let p3 = Vector2::new(in_tri.p3.x as f32, in_tri.p3.y as f32);

    let d0 = dist(p1);
    let d1 = dist(p2);
    let d2 = dist(p3);

    if d0 >= 0.0 {
        inside_points[n_inside_point_count] = &p1;
        inside_tex[n_inside_tex_count] = &in_tri.t1;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &p1;
        outside_tex[n_outside_tex_count] = &in_tri.t1;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }
    if d1 >= 0.0 {
        inside_points[n_inside_point_count] = &p2;
        inside_tex[n_inside_tex_count] = &in_tri.t2;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &p2;
        outside_tex[n_outside_tex_count] = &in_tri.t2;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }
    if d2 >= 0.0 {
        inside_points[n_inside_point_count] = &p3;
        inside_tex[n_inside_tex_count] = &in_tri.t3;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &p3;
        outside_tex[n_outside_tex_count] = &in_tri.t3;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }

    if n_inside_point_count == 0 {
        return (None, None);
    }

    if n_inside_point_count == 3 {
        return (Some(*in_tri), None);
    }

    if n_inside_point_count == 1 && n_outside_point_count == 2 {
        let p1 = inside_points[0];
        let (p2, t) = vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[0]);
        let t2 = t * (outside_tex[0] - inside_tex[0]) + inside_tex[0];
        let (p3, t) = vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[1]);
        let t3 = t * (outside_tex[1] - inside_tex[0]) + inside_tex[0];
        let out_tri = Triangle2D {
            p1: p1.map(|x| x as i16),
            p2,
            p3,
            t1: *inside_tex[0],
            t2,
            t3,
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };

        return (Some(out_tri), None);
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        let p1 = inside_points[0];
        let p2 = inside_points[1];
        let (p3, t) = vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[0]);
        let t3 = t * (outside_tex[0] - inside_tex[0]) + inside_tex[0];
        let out_tri1 = Triangle2D {
            p1: p1.map(|x| x as i16),
            p2: p2.map(|x| x as i16),
            p3,
            t1: *inside_tex[0],
            t2: *inside_tex[1],
            t3,
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };

        let (p3, t) = vector_intersect_line(line_p, &line_n, inside_points[1], outside_points[0]);
        let t3 = t * (outside_tex[0] - inside_tex[1]) + inside_tex[1];
        let out_tri2 = Triangle2D {
            p1: inside_points[1].map(|x: f32| x as i16),
            p2: out_tri1.p3,
            p3,
            t1: *inside_tex[1],
            t2: out_tri1.t3,
            t3,
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        return (Some(out_tri1), Some(out_tri2));
    }
    (None, None)
}

pub fn triangle_clip_against_plane(
    plane_p: &Vector3<f32>,
    plane_n: &Vector3<f32>,
    in_tri: &Triangle,
) -> (Option<Triangle>, Option<Triangle>) {
    let plane_n = plane_n.normalize();

    let dist = |p: Vector3<f32>| {
        plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - plane_n.dot(plane_p)
    };

    let default = Default::default();
    let mut inside_points: [&Vector3<f32>; 3] = [&default; 3];
    let mut n_inside_point_count = 0;
    let mut outside_points: [&Vector3<f32>; 3] = [&default; 3];
    let mut n_outside_point_count = 0;

    let default = Default::default();
    let mut inside_tex: [&Vector2<f32>; 3] = [&default; 3];
    let mut n_inside_tex_count = 0;
    let mut outside_tex: [&Vector2<f32>; 3] = [&default; 3];
    let mut n_outside_tex_count = 0;

    let d0 = dist(in_tri.p1);
    let d1 = dist(in_tri.p2);
    let d2 = dist(in_tri.p3);

    if d0 >= 0.0 {
        inside_points[n_inside_point_count] = &in_tri.p1;
        inside_tex[n_inside_tex_count] = &in_tri.t1;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &in_tri.p1;
        outside_tex[n_outside_tex_count] = &in_tri.t1;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }
    if d1 >= 0.0 {
        inside_points[n_inside_point_count] = &in_tri.p2;
        inside_tex[n_inside_tex_count] = &in_tri.t2;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &in_tri.p2;
        outside_tex[n_outside_tex_count] = &in_tri.t2;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }
    if d2 >= 0.0 {
        inside_points[n_inside_point_count] = &in_tri.p3;
        inside_tex[n_inside_tex_count] = &in_tri.t3;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &in_tri.p3;
        outside_tex[n_outside_tex_count] = &in_tri.t3;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }

    if n_inside_point_count == 0 {
        return (None, None);
    }

    if n_inside_point_count == 3 {
        return (Some(*in_tri), None);
    }

    if n_inside_point_count == 1 && n_outside_point_count == 2 {
        let (p2, t) =
            vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[0]);
        let t2 = t * (outside_tex[0] - inside_tex[0]) + inside_tex[0];
        let (p3, t) =
            vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[1]);
        let t3 = t * (outside_tex[1] - inside_tex[0]) + inside_tex[0];
        let out_tri = Triangle {
            p1: *inside_points[0],
            p2,
            p3,
            t1: *inside_tex[0],
            t2,
            t3,
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };

        return (Some(out_tri), None);
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        let (p3, t) =
            vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[0]);
        let t3 = t * (outside_tex[0] - inside_tex[0]) + inside_tex[0];
        let out_tri1 = Triangle {
            p1: *inside_points[0],
            p2: *inside_points[1],
            p3,
            t1: *inside_tex[0],
            t2: *inside_tex[1],
            t3,
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };

        let (p3, t) =
            vector_intersect_plane(plane_p, &plane_n, inside_points[1], outside_points[0]);
        let t3 = t * (outside_tex[0] - inside_tex[1]) + inside_tex[1];
        let out_tri2 = Triangle {
            p1: *inside_points[1],
            p2: out_tri1.p3,
            p3,
            t1: *inside_tex[1],
            t2: out_tri1.t3,
            t3,
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        return (Some(out_tri1), Some(out_tri2));
    }
    (None, None)
}

impl Renderer {
    pub fn update_fov(&mut self, new_fov: f32) {
        self.camera.set_fov(new_fov);
        self.projection_matrix =
            Perspective3::new(ASPECT_RATIO, self.camera.get_fov(), ZNEAR, ZFAR);
    }

    pub fn project_point(&self, point: Vector3<f32>) -> Vector3<f32> {
        self.projection_matrix.project_vector(&point)
    }

    pub fn clear_screen(&mut self, color: Color565) {
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
                let mut p1 = self.project_point(to_project.p1);
                let mut p2 = self.project_point(to_project.p2);
                let mut p3 = self.project_point(to_project.p3);
                let w1 = -to_project.p1.z;
                let w2 = -to_project.p2.z;
                let w3 = -to_project.p3.z;
                let t1 = Vector3::new(to_project.t1.x / w1, to_project.t1.y / w1, 1.0 / w1);
                let t2 = Vector3::new(to_project.t2.x / w2, to_project.t2.y / w2, 1.0 / w2);
                let t3 = Vector3::new(to_project.t3.x / w3, to_project.t3.y / w3, 1.0 / w3);
                let projected_triangle = Triangle2D {
                    p1: (p1.xy() + Vector2::repeat(1.))
                        .component_mul(&HALF_SCREEN)
                        .map(|x| x as i16),
                    p2: (p2.xy() + Vector2::repeat(1.))
                        .component_mul(&HALF_SCREEN)
                        .map(|x| x as i16),
                    p3: (p3.xy() + Vector2::repeat(1.))
                        .component_mul(&HALF_SCREEN)
                        .map(|x| x as i16),
                    t1,
                    t2,
                    t3,
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
                    clip_triangle(Vector2::new(SCREEN_WIDTHF, 0.0), Vector2::new(-1.0, 0.0));
                }

                for tri in clip_buffer {
                    if self.triangles_to_render.len() >= MAX_TRIANGLES {
                        // TODO : Find a proper fix for this
                        break;
                    }
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
        for tri in self.triangles_to_render.iter_mut().rev() {
            let mut tri_copy = tri.to_tri_2d();
            tri_copy.p1 += tile_offset;

            tri_copy.p2 += tile_offset;

            tri_copy.p3 += tile_offset;

            clip_and_draw_2d_triangle(
                tri_copy,
                &mut self.tile_frame_buffer,
                &mut self.tile_depth_buffer,
            );
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

    pub fn draw_game(
        &mut self,
        world: &mut World,
        player: &Player,
        frame_time: u64,
        hud: &Hud,
        draw_hud: bool,
    ) {
        let mat_view = self.get_mat_view();

        let frustum = Frustum::new(
            &self.camera,
            ASPECT_RATIO,
            self.camera.get_fov(),
            ZNEAR,
            ZFAR,
        );

        // Add the player block marker
        let mut block_marker = player.get_block_marker();
        for quad in block_marker.0.get_reference_vec() {
            self.add_quad_to_render(quad, &mat_view, block_marker.1);
        }

        for chunk in world
            .chunks_manager
            .get_chunks_sorted_by_distance(*self.camera.get_pos())
        {
            let chunk_blocks_pos = chunk.get_pos() * CHUNK_SIZE_I;
            let chunk_blocks_posf = chunk_blocks_pos.map(|x| x as f32);
            let chunk_blocks_pos_maxf =
                (chunk_blocks_pos + Vector3::repeat(CHUNK_SIZE_I)).map(|x| x as f32);

            if !(frustum.is_aabb_in_frustum(chunk_blocks_posf, chunk_blocks_pos_maxf)) {
                continue;
            }

            let need_sorting = chunk.need_sorting || self.camera.get_has_moved();

            let quads = chunk.get_mesh().get_reference_vec();

            /*if need_sorting {
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
                        .reverse()
                });
            }*/
            for quad in quads {
                self.add_quad_to_render(quad, &mat_view, chunk_blocks_pos);
            }
        }

        for x in 0..SCREEN_TILE_SUBDIVISION {
            for y in 0..SCREEN_TILE_SUBDIVISION {
                self.clear_screen(Color565::new(0b01110, 0b110110, 0b11111));
                for i in 0..SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT {
                    self.tile_depth_buffer[i] = f16::MAX;
                }
                self.draw_triangles(x, y);
                self.draw_flat_model_entities(world, &mat_view, x, y, &frustum);

                if draw_hud {
                    self.draw_hud(hud, frame_time, x, y);
                }

                push_rect(
                    ScreenRect {
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
            wait_for_vblank();
        }
        self.triangles_to_render.clear();
    }
}
