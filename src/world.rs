use crate::chunk::{self, Chunk};
use crate::mesh::BlockFace;
use alloc::vec::Vec;
use fastnoise_lite::FastNoiseLite;
use nalgebra::Vector3;

pub struct World {
    chunks: Vec<chunk::Chunk>,
    gen_noise: FastNoiseLite,
}

impl World {
    pub fn new() -> Self {
        let mut world = World {
            chunks: Vec::new(),
            gen_noise: FastNoiseLite::new(),
        };

        world
            .gen_noise
            .set_noise_type(Some(fastnoise_lite::NoiseType::OpenSimplex2));

        world
    }

    pub fn add_chunk(&mut self, pos: Vector3<isize>) -> Option<&mut Chunk> {
        let chunk = Chunk::new(pos);
        self.chunks.push(chunk);

        self.chunks.last_mut()
    }

    pub fn generate_world(&mut self) {
        for chunk in self.chunks.iter_mut() {
            if !chunk.generated {
                chunk.generate_chunk(&self.gen_noise);
            }
        }
    }

    pub fn get_mesh(&self) -> Vec<&Vec<BlockFace>> {
        let mut world_mesh = Vec::new();
        for chunk in &self.chunks {
            world_mesh.push(chunk.get_mesh());
        }

        world_mesh
    }

    pub fn generate_mesh(&mut self) {
        for chunk in self.chunks.iter_mut() {
            chunk.generate_mesh();
        }
    }
}
