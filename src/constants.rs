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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
}

impl BlockType {
    pub fn is_air(&self) -> bool {
        *self == BlockType::Air
    }
}
