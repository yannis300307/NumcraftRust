use crate::{eadk::Color, mesh::QuadDir};

pub mod rendering {
    pub const SCREEN_WIDTH: usize = 320;
    pub const SCREEN_HEIGHT: usize = 240;

    pub const SCREEN_TILE_SUBDIVISION: usize = 2; // Minimum 2

    pub const FOV: f32 = core::f32::consts::PI / 4.0;

    pub const MAX_TRIANGLES: usize = 1300;

    pub const RENDER_DISTANCE: usize = 2;
}

pub mod world {
    pub const CHUNK_SIZE: usize = 8; // MAX 8
}

pub mod player {
    use core::f32::consts::PI;

    pub const ROTATION_SPEED: f32 = PI / 3.0; // rad / sec
    pub const MOVEMENT_SPEED: f32 = 2.0;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
}

impl BlockType {
    pub fn is_air(&self) -> bool {
        *self == BlockType::Air
    }

    pub fn get_texture_id(&self, dir: QuadDir) -> u8 {
        match *self {
            BlockType::Air => 0,
            BlockType::Stone => 1,
            BlockType::Grass => {
                if dir == QuadDir::Top {
                    2
                } else {
                    3
                }
            }
            BlockType::Dirt => 3,
        }
    }
}

pub fn get_quad_color_from_texture_id(id: u8) -> Color {
    match id {
        0 => Color::from_888(0, 0, 0),
        1 => Color::from_888(160, 160, 160),
        2 => Color::from_888(21, 147, 0),
        3 => Color::from_888(120, 77, 49),
        _ => Color::from_888(0, 0, 0),
        // 255 is reserved for block outline
    }
}
