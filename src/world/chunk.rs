use crate::{
    constants::{BlockType, world::*},
    renderer::mesh::Mesh,
};

use nalgebra::Vector3;

const BLOCK_COUNT: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct Chunk {
    blocks: [BlockType; BLOCK_COUNT],
    pos: Vector3<isize>,
    pub mesh: Mesh,
    pub generated: bool,
    pub need_new_mesh: bool,
    pub need_sorting: bool,
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(pos: Vector3<isize>) -> Self {
        Chunk {
            blocks: [BlockType::Air; BLOCK_COUNT],
            pos,
            mesh: Mesh::new(),
            generated: false,
            need_new_mesh: true,
            need_sorting: false,
        }
    }

    pub fn set_at(&mut self, pos: Vector3<usize>, block_type: BlockType) -> bool {
        if pos.x < CHUNK_SIZE && pos.y < CHUNK_SIZE && pos.z < CHUNK_SIZE {
            self.blocks[pos.x + pos.y * CHUNK_SIZE + pos.z * CHUNK_SIZE * CHUNK_SIZE] = block_type;
            true
        } else {
            false
        }
    }

    pub fn get_at(&self, pos: Vector3<isize>) -> Option<BlockType> {
        if pos.x < CHUNK_SIZE_I
            && pos.y < CHUNK_SIZE_I
            && pos.z < CHUNK_SIZE_I
            && pos.x >= 0
            && pos.y >= 0
            && pos.z >= 0
        {
            Some(
                self.blocks
                    [(pos.x + pos.y * CHUNK_SIZE_I + pos.z * CHUNK_SIZE_I * CHUNK_SIZE_I) as usize],
            )
        } else {
            None
        }
    }

    pub fn get_at_unchecked(&self, pos: Vector3<isize>) -> BlockType {
        self.blocks[(pos.x + pos.y * CHUNK_SIZE_I + pos.z * CHUNK_SIZE_I * CHUNK_SIZE_I) as usize]
    }

    pub fn get_pos(&self) -> &Vector3<isize> {
        &self.pos
    }

    pub fn get_all_blocks(&self) -> &[BlockType; BLOCK_COUNT] {
        &self.blocks
    }

    pub fn get_mesh(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    pub fn set_mesh(&mut self, new_mesh: Mesh) {
        self.mesh = new_mesh;
        self.need_new_mesh = false;
        self.need_sorting = true;
    }
}
