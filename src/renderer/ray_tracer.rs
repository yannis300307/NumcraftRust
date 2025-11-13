use cbitmap::bitmap::{Bitmap, BitsManage};
use libm::{cosf, floorf, powf, sinf, sqrtf, tanf};
use nalgebra::{SimdPartialOrd, Vector2, Vector3};

#[cfg(target_os = "none")]
use alloc::{boxed::Box, vec::Vec};

use crate::{
    camera::Camera,
    constants::{BlockType, get_quad_color_from_texture_id, rendering::BITMAP_SIZE},
    eadk::Color,
    renderer::{SCREEN_HEIGHTF, SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH, SCREEN_WIDTHF}, world::chunk_manager::{self, ChunksManager},
};

struct RayTracer {}

// temp
const MAP_SIZE: Vector3<isize> = Vector3::new(32, 32, 32);

#[inline(always)]
pub fn ray(
    dir: Vector3<f32>,
    pos_floor: Vector3<isize>,
    pos_frac: Vector3<f32>,
    blocks: &cbitmap::bitmap::Bitmap<BITMAP_SIZE>,
) -> Option<RayResult> {
    let v_ray_unit_step_size =
        Vector3::new(1.0 / dir.x.abs(), 1.0 / dir.y.abs(), 1.0 / dir.z.abs());

    let mut v_map_check = pos_floor;
    let mut v_ray_length: Vector3<f32> = Vector3::default();
    let mut v_step: Vector3<isize> = Vector3::default();

    // Establish Starting Conditions
    if dir.x < 0.0 {
        v_step.x = -1;
        v_ray_length.x = pos_frac.x * v_ray_unit_step_size.x;
    } else {
        v_step.x = 1;
        v_ray_length.x = (1.0 - pos_frac.x) * v_ray_unit_step_size.x;
    }

    if dir.y < 0.0 {
        v_step.y = -1;
        v_ray_length.y = pos_frac.y * v_ray_unit_step_size.y;
    } else {
        v_step.y = 1;
        v_ray_length.y = (1.0 - pos_frac.y) * v_ray_unit_step_size.y;
    }

    if dir.z < 0.0 {
        v_step.z = -1;
        v_ray_length.z = pos_frac.z * v_ray_unit_step_size.z;
    } else {
        v_step.z = 1;
        v_ray_length.z = (1.0 - pos_frac.z) * v_ray_unit_step_size.z;
    }

    // Perform "Walk" until collision or range check
    let f_max_distance = 16.;
    let mut f_distance = 0.;

    let max_x = MAP_SIZE.x;
    let max_y = MAP_SIZE.y;
    let max_z = MAP_SIZE.z;
    while f_distance < f_max_distance {
        // Walk along shortest path
        if v_ray_length.x < v_ray_length.y && v_ray_length.x < v_ray_length.z {
            v_map_check.x += v_step.x;
            f_distance = v_ray_length.x;
            v_ray_length.x += v_ray_unit_step_size.x;
        } else if v_ray_length.y < v_ray_length.x && v_ray_length.y < v_ray_length.z {
            v_map_check.y += v_step.y;
            f_distance = v_ray_length.y;
            v_ray_length.y += v_ray_unit_step_size.y;
        } else {
            v_map_check.z += v_step.z;
            f_distance = v_ray_length.z;
            v_ray_length.z += v_ray_unit_step_size.z;
        }

        // Test tile at new test point
        if v_map_check.x >= 0
            && v_map_check.x < max_x
            && v_map_check.y >= 0
            && v_map_check.y < max_y
            && v_map_check.z >= 0
            && v_map_check.z < max_z
        {
            if blocks.get_bool((v_map_check.x + v_map_check.y * 32 + v_map_check.z * 1024) as usize)
            {
                let uv = Vector2::new(x, y);
                return Some(RayResult {distance: f_distance, pos: v_map_check, uv});
            }
        }
    }

    // Calculate intersection location
    None
}

struct RayResult {
    distance: f32,
    pos: Vector3<isize>,
    uv: Vector2<f32>,
}

pub fn draw_terrain(
    camera: &Camera,
    blocks: &Bitmap<BITMAP_SIZE>,
    tile_frame_buffer: &mut [Color],
    chunks_manager: &ChunksManager,
    tile_x: usize,
    tile_y: usize,
) {
    let fov_rad = camera.get_fov();
    let fov_scale = tanf(fov_rad * 0.5);
    let aspect = SCREEN_WIDTHF / SCREEN_HEIGHTF;

    // treat camera.get_rotation() as (yaw_deg, pitch_deg, _)
    let rot = *camera.get_rotation();
    let pitch = rot.x;
    let yaw = rot.y;

    // forward from yaw/pitch
    let forward = Vector3::new(
        cosf(pitch) * sinf(yaw),
        sinf(pitch),
        cosf(pitch) * cosf(yaw),
    )
    .normalize();
    let world_up = Vector3::new(0.0, 1.0, 0.0);
    let right = forward.cross(&world_up).normalize();
    let up = right.cross(&forward).normalize();

    let inv_map_x_f = 1. / MAP_SIZE.x as f32;
    let screen_width_coef = 2. / SCREEN_WIDTHF;
    let screen_height_coef = 2. / SCREEN_HEIGHTF;

    let pos = *camera.get_pos();

    let pos_floor = Vector3::new(floorf(pos.x), floorf(pos.y), floorf(pos.z));
    let pos_frac = Vector3::new(
        pos.x - pos_floor.x,
        pos.y - pos_floor.y,
        pos.z - pos_floor.z,
    );
    let pos_floor = pos_floor.map(|x| x as isize);

    for y in (0..SCREEN_TILE_HEIGHT).step_by(2) {
        let py = (((y + tile_y * SCREEN_TILE_HEIGHT) as f32 + 0.5) * screen_height_coef) - 1.0;

        for x in (0..SCREEN_TILE_WIDTH).step_by(2) {
            let px = (((x + tile_x * SCREEN_TILE_WIDTH) as f32 + 0.5) * screen_width_coef) - 1.0;
            let cam_ray = Vector3::new(px * aspect * fov_scale, -py * fov_scale, 1.0);
            let dir = forward * cam_ray.z + right * cam_ray.x + up * cam_ray.y;

            if let Some(result) = ray(dir, pos_floor, pos_frac, &blocks) && result.distance > 0.0 {
                let color = get_quad_color_from_texture_id(chunks_manager.get_block_in_world_unchecked(result.pos).get_texture_id(super::mesh::QuadDir::Top));
                tile_frame_buffer[x + y * SCREEN_TILE_WIDTH] = color;
                tile_frame_buffer[x + 1 + y * SCREEN_TILE_WIDTH] = color;
                tile_frame_buffer[x + (y + 1) * SCREEN_TILE_WIDTH] = color;
                tile_frame_buffer[x + 1 + (y + 1) * SCREEN_TILE_WIDTH] = color;
            }
        }
    }
}
