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

/// A const function to create a Color from 8-bit R, G, B components.
/// This is necessary for initializing `static` Color values in a `no_std` environment,
/// as `Color::from_components` is not a `const fn`.
const fn const_color_from_888(r: u8, g: u8, b: u8) -> Color {
    // Convert 8-bit RGB (0-255) to 16-bit RGB565 format:
    // Red: 5 bits (r / 8) -> shift left 11 bits
    // Green: 6 bits (g / 4) -> shift left 5 bits
    // Blue: 5 bits (b / 8)
    Color {
        rgb565: ((r as u16 / 8) << 11) | ((g as u16 / 4) << 5) | (b as u16 / 8)
    }
}

// --- UI Colors: Now initialized using our custom `const_color_from_888` function
pub static UI_WHITE: Color = const_color_from_888(255, 255, 255);
pub static UI_BLACK: Color = const_color_from_888(0, 0, 0);
pub static UI_GREY: Color = const_color_from_888(160, 160, 160);    // Approx. 160 for 8-bit to 5/6-bit conversion
pub static UI_LIGHT_GREY: Color = const_color_from_888(198, 198, 198); // Approx. 198 for 8-bit to 5/6-bit conversion
pub static UI_RED: Color = const_color_from_888(255, 0, 0);
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
    // Changed to use Color::from_components for runtime calls for consistency
    // Assuming Color::from_components expects 0-255 range for u16 inputs like Color::from_888
    match id {
        0 => Color::from_components(0, 0, 0),       // Black/Transparent for Air or missing texture
        1 => Color::from_components(160, 160, 160), // Stone color
        2 => Color::from_components(21, 147, 0),    // Grass top color
        3 => Color::from_components(120, 77, 49),   // Dirt color
        // 255 is reserved for block outline in player.rs
        255 => Color::from_components(255, 255, 255), // White for block marker outline
        _ => Color::from_components(0, 0, 0),       // Default to black for unknown IDs
    }
}
