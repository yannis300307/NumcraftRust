use crate::chunk::{self, Chunk};
use crate::constants::world::*;
use crate::eadk;
use crate::mesh::BlockFace;
use alloc::vec::Vec;
use nalgebra::Vector3;

pub struct World {
    chunks: Vec<chunk::Chunk>,
    mesh: Vec<BlockFace>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: Vec::new(),
            mesh: Vec::new(),
        }
    }

    pub fn add_chunk(&mut self, pos: Vector3<isize>) -> Option<&mut Chunk> {
        let chunk = Chunk::new(pos);
        self.chunks.push(chunk);

        self.chunks.last_mut()
    }

    pub fn get_mesh(&self) -> &Vec<BlockFace> { // TODO Render each chunk mesh independentely
        &self.mesh
    }

    pub fn generate_mesh(&mut self) -> &Vec<BlockFace> {
        for chunk in &self.chunks {
            chunk.add_mesh_to_world_mesh(&mut self.mesh);
        }

        &self.mesh
    }
}
