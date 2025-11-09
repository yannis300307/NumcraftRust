use cbitmap::bitmap::BitsManage;
use libm::{cosf, floorf, powf, sinf, sqrtf, tanf};
use nalgebra::{SimdPartialOrd, Vector3};

#[cfg(target_os = "none")]
use alloc::{boxed::Box, vec::Vec};

use crate::{
    camera::Camera,
    constants::{
        BlockType,
        rendering::{FOV, SCREEN_WIDTH},
    },
    eadk::Color,
    renderer::{SCREEN_HEIGHTF, SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH, SCREEN_WIDTHF},
    world::chunk_manager::{self, ChunksManager},
};

struct RayTracer {}

// temp
const MAP_SIZE: Vector3<isize> = Vector3::new(32, 32, 32);

#[inline]
pub fn ray(
    dir: Vector3<f32>,
    pos: Vector3<f32>,
    blocks: &cbitmap::bitmap::Bitmap<BITMAP_SIZE>,
) -> f32 {
    // Lodev.org also explains this additional optimistaion (but it's beyond scope of video)
    // olc::vf2d vRayUnitStepSize = { abs(1.0f / dir.x), abs(1.0f / dir.y) };

    let v_ray_unit_step_size = Vector3::new(
        if dir.x != 0.0 {
            1.0 / dir.x.abs()
        } else {
            f32::INFINITY
        },
        if dir.y != 0.0 {
            1.0 / dir.y.abs()
        } else {
            f32::INFINITY
        },
        if dir.z != 0.0 {
            1.0 / dir.z.abs()
        } else {
            f32::INFINITY
        },
    );

    let pos_floor = Vector3::new(floorf(pos.x), floorf(pos.y), floorf(pos.z));
    let pos_frac = Vector3::new(
        pos.x - pos_floor.x,
        pos.y - pos_floor.y,
        pos.z - pos_floor.z,
    );

    let mut v_map_check = pos_floor.map(|v| v as isize);
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
    while f_distance < f_max_distance  {
        // Walk along shortest path
        if v_ray_length.x < v_ray_length.y && v_ray_length.x < v_ray_length.z  {
            v_map_check.x += v_step.x;
            f_distance = v_ray_length.x;
            v_ray_length.x += v_ray_unit_step_size.x;
        } else if v_ray_length.y < v_ray_length.x && v_ray_length.y < v_ray_length.z  {
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
            if blocks.get_bool(
                (v_map_check.x + v_map_check.y * 32 + v_map_check.z * 1024) as usize,
            ) {
                return f_distance;
            }
        }
    }

    // Calculate intersection location
    -1.
}

/*pub fn draw_terrain(camera: &Camera, chunks_manager: &ChunksManager, tile_frame_buffer: &mut [Color], tile_x: usize, tile_y: usize) {
    for x in 0..SCREEN_TILE_WIDTH {
        for y in 0..SCREEN_TILE_HEIGHT {
            let dir = *camera.get_rotation() + Vector3::new(camera.get_fov()/180. * (x + tile_x * SCREEN_TILE_WIDTH - SCREEN_WIDTH / 2) as f32 / SCREEN_WIDTHF, camera.get_fov() / 180. * (y + tile_y * SCREEN_TILE_HEIGHT) as f32 / SCREEN_HEIGHTF, 0.);
            println!("{:?}", dir);
            let result = ray(dir, *camera.get_pos(), chunks_manager);
            if result > 0. {
                let value = (result / 32. * 255.) as u16;
                tile_frame_buffer[x + y * SCREEN_TILE_WIDTH] = Color::from_888(value, value, value);
            }
        }
    }
}*/

const BITMAP_SIZE: usize = 4 * 8 * 4 * 8 * 4;
pub fn draw_terrain(
    camera: &Camera,
    chunks_manager: &ChunksManager,
    tile_frame_buffer: &mut [Color],
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

    let map_x_f = MAP_SIZE.x as f32;

    let mut blocks = cbitmap::bitmap::Bitmap::<BITMAP_SIZE>::new();

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let index = x + y * 32 + z * 32 * 32;
                if chunks_manager
                    .get_block_in_world_unchecked(Vector3::new(x as isize, y as isize, z as isize))
                    .is_air()
                {
                    blocks.reset(index);
                } else {
                    blocks.set(index);
                }
            }
        }
    }

    for y in 0..SCREEN_TILE_HEIGHT {
        let y_offset = y * SCREEN_TILE_WIDTH;
        let py = (((y + tile_y * SCREEN_TILE_HEIGHT) as f32 + 0.5) / SCREEN_HEIGHTF) * 2.0 - 1.0;

        for x in 0..SCREEN_TILE_WIDTH {
            let px = (((x + tile_x * SCREEN_TILE_WIDTH) as f32 + 0.5) / SCREEN_WIDTHF) * 2.0 - 1.0;
            let cam_ray = Vector3::new(px * aspect * fov_scale, -py * fov_scale, 1.0);
            let dir = forward * cam_ray.z + right * cam_ray.x + up * cam_ray.y;

            let result = ray(dir, *camera.get_pos(), &blocks);
            if result > 0.0 {
                let value = ((result / map_x_f) * 255.0) as u16;
                tile_frame_buffer[x + y_offset] = Color::from_888(value, value, value);
            }
        }
    }
}
