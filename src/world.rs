use libm::roundf;

use crate::chunk::{self, Chunk};
use crate::constants::world::CHUNK_SIZE;
use crate::mesh::Quad;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use fastnoise_lite::FastNoiseLite;
use nalgebra::Vector3;

pub struct World {
    pub chunks: Vec<chunk::Chunk>,
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

    fn get_chunk_exists_at(&self, pos: Vector3<isize>) -> bool {
        for chunk in &self.chunks {
            if *chunk.get_pos() == pos {
                return true;
            }
        }
        false
    }

    pub fn generate_world_around_pos(&mut self, pos: Vector3<f32>, render_distance: isize) {
        self.chunks.retain(|chunk| {
            let chunk_pos = chunk.get_pos();
            if chunk_pos.x < -render_distance || chunk_pos.x >= render_distance 
            || chunk_pos.y < -render_distance || chunk_pos.y >= render_distance
            || chunk_pos.z < -render_distance || chunk_pos.z >= render_distance {
                false
            } else 
            {true}
        });
        for x in -render_distance..render_distance {
            for y in -render_distance..render_distance {
                for z in -render_distance..render_distance {
                    let chunk_pos: Vector3<isize> = Vector3::new(
                        roundf(pos.x / CHUNK_SIZE as f32) as isize + x,
                        roundf(pos.y / CHUNK_SIZE as f32) as isize + y,
                        roundf(pos.z / CHUNK_SIZE as f32) as isize + z,
                    );

                    if !self.get_chunk_exists_at(chunk_pos) {
                        if self.add_chunk(chunk_pos).is_none() {continue;};
                        self.chunks.last_mut().unwrap().generate_chunk(&self.gen_noise);
                    }
                }
            }
        }
    }

    pub fn get_mesh(&self) -> Vec<&Vec<Quad>> {
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
