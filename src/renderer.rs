#[cfg(target_os = "none")]
use alloc::format;

#[cfg(target_os = "none")]
use alloc::vec::Vec;

use nalgebra::{Matrix4, Perspective3, Vector2, Vector3, Vector4};

use core::{cmp::Ordering, f32, mem::swap};

use crate::{
    camera::Camera,
    constants::{rendering::*, world::CHUNK_SIZE},
    eadk::Color,
    renderer::mesh::SmallTriangle2D,
};

mod engine_3d;
mod hud;
mod menus;
mod ui;
mod frustum;
pub mod mesh;
mod misc;
mod entity;

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

static FONT_DATA: &[u8] = include_bytes!("../target/assets/font.bin");
const FONT_WIDTH: usize = 1045;
const FONT_HEIGHT: usize = 15;

static CROSS_DATA: &[u8] = include_bytes!("../target/assets/cross.bin");
const CROSS_WIDTH: usize = 14;
const CROSS_HEIGHT: usize = 14;

const FONT_CHAR_WIDTH: usize = 11;
static FONT_ORDER: &str = "!\" $%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^+`abcdefghijklmnopqrstuvwxyz{|}~€";

static TILESET_DATA: &[u8] = include_bytes!("../target/assets/tileset.bin");

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
}
