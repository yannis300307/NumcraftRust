use crate::{eadk::Color, mesh::QuadDir};

pub mod rendering {
    pub const SCREEN_WIDTH: usize = 320;
    pub const SCREEN_HEIGHT: usize = 240;

    pub const SCREEN_TILE_SUBDIVISION: usize = 2; // Minimum 2

    pub const MIN_FOV: f32 = 30.;
    pub const MAX_FOV: f32 = 110.;

    pub const FOV: f32 = 45.;

    pub const MAX_TRIANGLES: usize = 1300;

    pub const MAX_RENDER_DISTANCE: usize = 2; // You shouldn't go higher
}

pub mod menu {
    use crate::eadk::Color;

    pub const MENU_OUTLINE_COLOR: Color = Color::from_888(150, 150, 150);
    pub const MENU_ELEMENT_BACKGROUND_COLOR: Color = Color::from_888(230, 230, 230);
    pub const MENU_ELEMENT_BACKGROUND_COLOR_HOVER: Color = Color::from_888(190, 190, 190);
    pub const MENU_TEXT_COLOR: Color = Color::from_888(0, 0, 0);
    pub const MENU_BACKGROUND_COLOR: Color = Color::from_888(255, 255, 255);

    pub const SETTINGS_FILENAME: &str = "settings.ncd"; // NCD = NumCraftData
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

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(BlockType::Air),
            1 => Some(BlockType::Stone),
            2 => Some(BlockType::Grass),
            3 => Some(BlockType::Dirt),
            _ => None,
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
