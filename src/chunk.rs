use nalgebra::Vector3;
use crate::constants::world::*;

const BLOCK_COUNT: usize = CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE;

pub struct Chunk {
    blocks: [u8; BLOCK_COUNT],
    pos: Vector3<isize>,
    is_void: bool
}

impl Chunk{
    pub fn new(pos: Vector3<isize>) -> Self {
        Chunk { blocks: [0; BLOCK_COUNT], pos, is_void: true }
    }
    
    pub fn get_mesh() {

    }
}