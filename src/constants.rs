use serde::{Deserialize, Serialize};

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

    pub const BLURING_SCREEN_SUBDIVISION: usize = 5;
    pub const BLURING_RADIUS: isize = 2;
}

pub mod color_palette {
    use crate::eadk::Color;

    pub const MENU_OUTLINE_COLOR: Color = Color::from_888(150, 150, 150);
    pub const MENU_ELEMENT_BACKGROUND_COLOR: Color = Color::from_888(230, 230, 230);
    pub const MENU_ELEMENT_BACKGROUND_COLOR_HOVER: Color = Color::from_888(190, 190, 190);
    pub const MENU_TEXT_COLOR: Color = Color::from_888(0, 0, 0);
    pub const MENU_BACKGROUND_COLOR: Color = Color::from_888(255, 255, 255);

    pub const GAMEUI_SLOT_COLOR: Color = Color::from_888(80, 80, 80);
    pub const GAMEUI_SLOT_DEFAULT_OUTLINE_COLOR: Color = Color::from_888(120, 120, 120);
}

pub mod menu {
    pub const SETTINGS_FILENAME: &str = "settings.ncd"; // NCD = NumCraftData
}

pub mod world {
    pub const CHUNK_SIZE: usize = 8; // MAX 8
}

pub mod player {
    use core::f32::consts::PI;

    pub const ROTATION_SPEED: f32 = PI / 3.0; // rad / sec
    pub const MOVEMENT_SPEED: f32 = 4.0;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum ItemType {
    Air = 0,

    StoneBlock = 1,
    GrassBlock = 2,
    DirtBlock = 3,
}

impl ItemType {
    pub fn get_texture_id(&self) -> u8 {
        match *self {
            ItemType::StoneBlock => 1,
            ItemType::GrassBlock => 2,
            ItemType::DirtBlock => 3,
            _ => 0,
        }
    }

    pub fn get_max_stack_amount(&self) -> u8 {
        match *self {
            ItemType::Air => 0,
            ItemType::StoneBlock => 64,
            ItemType::GrassBlock => 64,
            ItemType::DirtBlock => 64,
        }
    }

    pub fn get_matching_block_type(&self) -> Option<BlockType> {
        match self {
            ItemType::Air => None,
            ItemType::StoneBlock => Some(BlockType::Stone),
            ItemType::GrassBlock => Some(BlockType::Grass),
            ItemType::DirtBlock => Some(BlockType::Dirt),
        }
    }
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
