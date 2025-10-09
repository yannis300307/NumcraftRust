use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::{eadk::Color, physic::BoundingBox, renderer::mesh::QuadDir};

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

    pub const MAX_ENTITY_RENDER_DISTANCE: f32 = 10.;

    pub const ITEM_ENTITY_SPRITE_SIZE: f32 = 0.8;
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

pub mod save_manager {
    pub const SETTINGS_FILENAME: &str = "settings.ncd"; // NCD = NumCraftData

    pub const WORLD_VERSION: u16 = 0; // Update the version at each world breaking update
}

pub mod world {
    pub const CHUNK_SIZE: usize = 8; // MAX 8

    pub const MAX_ITEM_MERGING_DISTANCE: f32 = 2.;
    pub const ITEM_MAGNET_FORCE: f32 = 10.;
    pub const MAX_PLAYER_ITEM_MAGNET_DISTANCE: f32 = 2.2;
}

pub mod player {
    use core::f32::consts::PI;

    pub const ROTATION_SPEED: f32 = PI / 3.0; // rad / sec
    pub const FLY_SPEED: f32 = 4.0;
    pub const WALK_FORCE: f32 = 20.0;
    pub const MAX_WALKING_VELOCITY: f32 = 4.;
    pub const JUMP_FORCE: f32 = 5.;
}

pub mod physic {
    use nalgebra::Vector3;

    pub const GRAVITY_FACTOR: f32 = 10.0;
    pub const MAX_FALLING_VELOCITY: f32 = 5.;
    pub const ON_FLOOR_FRICTION: f32 = 10.;

    pub const BLOCK_COLLISION_SCANNING_SIZE: Vector3<isize> = Vector3::new(2, 3, 2);
}

#[allow(unreachable_patterns)]
impl EntityType {
    pub fn get_bbox(&self) -> Option<BoundingBox> {
        match self {
            EntityType::Player => Some(BoundingBox {
                offset: Vector3::new(-0.4, -0.5, -0.4),
                size: Vector3::new(0.8, 1.8, 0.8),
            }),
            EntityType::Item => Some(BoundingBox {
                offset: Vector3::new(-0.2, -0.2, -0.2),
                size: Vector3::new(0.4, 0.4, 0.4),
            }),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Player = 0,
    Item = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Sand = 4,
    Cobblestone = 5,
    Border = 6,
    Log = 7,
    Leaves = 8,
    Planks = 9,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum ItemType {
    Air = 0,

    StoneBlock = 1,
    GrassBlock = 2,
    DirtBlock = 3,
    SandBlock = 4,
    CobblestoneBlock = 5,
    BorderBlock = 6,
    LogBlock = 7,
    LeavesBlock = 8,
    PlanksBlock = 9,
}

impl ItemType {
    pub fn get_texture_id(&self) -> u8 {
        match *self {
            ItemType::Air => 0,

            ItemType::StoneBlock => 1,
            ItemType::GrassBlock => 2,
            ItemType::DirtBlock => 3, // 4 is the other texture of the grass block
            ItemType::SandBlock => 5,
            ItemType::CobblestoneBlock => 6,
            ItemType::BorderBlock => 7,
            ItemType::LogBlock => 8,
            ItemType::LeavesBlock => 9,
            ItemType::PlanksBlock => 10,
        }
    }

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(ItemType::Air),

            1 => Some(ItemType::StoneBlock),
            2 => Some(ItemType::GrassBlock),
            3 => Some(ItemType::DirtBlock),
            4 => Some(ItemType::SandBlock),
            5 => Some(ItemType::CobblestoneBlock),
            6 => Some(ItemType::BorderBlock),
            7 => Some(ItemType::LogBlock),
            8 => Some(ItemType::LeavesBlock),
            9 => Some(ItemType::PlanksBlock),
            _ => None,
        }
    }

    pub fn get_max_stack_amount(&self) -> u8 {
        match *self {
            ItemType::Air => 0,
            ItemType::StoneBlock => 64,
            ItemType::GrassBlock => 64,
            ItemType::DirtBlock => 64,
            ItemType::SandBlock => 64,
            ItemType::CobblestoneBlock => 64,
            ItemType::BorderBlock => 64,
            ItemType::LogBlock => 64,
            ItemType::LeavesBlock => 64,
            ItemType::PlanksBlock => 64,
        }
    }

    pub fn get_matching_block_type(&self) -> Option<BlockType> {
        match self {
            ItemType::Air => None,
            ItemType::StoneBlock => Some(BlockType::Stone),
            ItemType::GrassBlock => Some(BlockType::Grass),
            ItemType::DirtBlock => Some(BlockType::Dirt),
            ItemType::SandBlock => Some(BlockType::Sand),
            ItemType::CobblestoneBlock => Some(BlockType::Cobblestone),
            ItemType::BorderBlock => Some(BlockType::Border),
            ItemType::LogBlock => Some(BlockType::Log),
            ItemType::LeavesBlock => Some(BlockType::Leaves),
            ItemType::PlanksBlock => Some(BlockType::Planks),
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
            BlockType::Sand => 5,
            BlockType::Cobblestone => 6,
            BlockType::Border => 7,
            BlockType::Log => 8,
            BlockType::Leaves => 9,
            BlockType::Planks => 10,
        }
    }

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(BlockType::Air),
            1 => Some(BlockType::Stone),
            2 => Some(BlockType::Grass),
            3 => Some(BlockType::Dirt),
            4 => Some(BlockType::Sand),
            5 => Some(BlockType::Cobblestone),
            6 => Some(BlockType::Border),
            7 => Some(BlockType::Log),
            8 => Some(BlockType::Leaves),
            9 => Some(BlockType::Planks),
            _ => None,
        }
    }

    pub const fn get_hardness(&self) -> f32 {
        match self {
            BlockType::Air => 0.,
            BlockType::Stone => 2.,
            BlockType::Grass => 1.2,
            BlockType::Dirt => 1.,
            BlockType::Sand => 1.,
            BlockType::Cobblestone => 2.2,
            BlockType::Border => -1.,
            BlockType::Log => 1.5,
            BlockType::Leaves => 0.3,
            BlockType::Planks => 1.2,
        }
    }

    pub const fn get_dropped_item_type(&self) -> ItemType {
        match self {
            BlockType::Air => ItemType::Air,
            BlockType::Stone => ItemType::CobblestoneBlock,
            BlockType::Grass => ItemType::DirtBlock,
            BlockType::Dirt => ItemType::DirtBlock,
            BlockType::Sand => ItemType::SandBlock,
            BlockType::Cobblestone => ItemType::CobblestoneBlock,
            BlockType::Border => ItemType::BorderBlock,
            BlockType::Log => ItemType::LogBlock,
            BlockType::Leaves => ItemType::LeavesBlock,
            BlockType::Planks => ItemType::PlanksBlock,
        }
    }
}

pub fn get_quad_color_from_texture_id(id: u8) -> Color {
    match id {
        1 => Color::from_888(160, 160, 160),
        2 => Color::from_888(21, 147, 0),
        3 => Color::from_888(120, 77, 49),
        5 => Color::from_888(208, 199, 6),
        6 => Color::from_888(178, 178, 178),
        7 => Color::from_888(19, 19, 19),
        8 => Color::from_888(79, 53, 30),
        9 => Color::from_888(36, 75, 37),
        10 => Color::from_888(152, 124, 61),

        _ => Color::from_888(0, 0, 0),
        // 255 is reserved for block outline
    }
}
