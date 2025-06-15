// In src/constants.rs

// Import Color from eadk
use crate::eadk::Color;
// Publicly re-export QuadDir from mesh, so it can be accessed via `constants::QuadDir` or directly
pub use crate::mesh::QuadDir;

pub mod rendering {
    pub const SCREEN_WIDTH: usize = 320;
    pub const SCREEN_HEIGHT: usize = 240;

    pub const SCREEN_TILE_SUBDIVISION: usize = 2; // Minimum 2

    pub const FOV: f32 = core::f32::consts::PI / 4.0;

    pub const MAX_TRIANGLES: usize = 1500;

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

// --- UI Colors: Initialized using direct RGB565 u16 values
// (R5 << 11) | (G6 << 5) | B5
// R (0-255) -> 5 bits (0-31) = R / 8
// G (0-255) -> 6 bits (0-63) = G / 4
// B (0-255) -> 5 bits (0-31) = B / 8
pub static UI_WHITE: Color = Color { rgb565: 0b11111_111111_11111 }; // R=255, G=255, B=255
pub static UI_BLACK: Color = Color { rgb565: 0x0000 };                 // R=0, G=0, B=0
pub static UI_GREY: Color = Color { rgb565: 0b10000_100000_10000 };    // R=160, G=160, B=160 (approx)
pub static UI_LIGHT_GREY: Color = Color { rgb565: 0b11000_110001_11000 }; // R=198, G=198, B=198 (approx)
pub static UI_RED: Color = Color { rgb565: 0xF800 };                   // R=255, G=0, B=0
// --- End UI colors ---


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    // Add more block types if you want them in the inventory
}

impl BlockType {
    pub fn is_air(&self) -> bool {
        *self == BlockType::Air
    }

    pub fn get_texture_id(&self, dir: QuadDir) -> u8 {
        match *self {
            BlockType::Air => 0, // Texture ID for air
            BlockType::Stone => 1, // Texture ID for stone
            BlockType::Grass => {
                if dir == QuadDir::Top {
                    2 // Grass top texture ID
                } else {
                    3 // Grass side texture ID (or dirt if shared)
                }
            }
            BlockType::Dirt => 3, // Dirt texture ID
        }
    }
}

// Make this function public to be accessible from `inventory.rs`
pub fn get_quad_color_from_texture_id(id: u8) -> Color {
    match id {
        0 => Color { rgb565: 0x0000 },       // Black/Transparent for Air or missing texture
        1 => Color { rgb565: 0b10000_100000_10000 }, // Stone color (re-using grey approx)
        2 => Color { rgb565: 0b00001_111011_00000 },    // Grass top color (21, 147, 0)
        3 => Color { rgb565: 0b01101_011010_10010 },   // Dirt color (120, 77, 49)
        // 255 is reserved for block outline in player.rs
        255 => Color { rgb565: 0xFFFF }, // White for block marker outline
        _ => Color { rgb565: 0x0000 },       // Default to black for unknown IDs
    }
}
