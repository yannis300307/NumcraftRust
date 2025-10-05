use fastnoise_lite::FastNoiseLite;
use libm::roundf;
use nalgebra::Vector3;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    constants::world::*,
    world::{
        chunk::Chunk,
        chunk_manager::ChunksManager,
        structures::{Structure, TREE1},
    },
};

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct WorldGenerator {
    noise: FastNoiseLite,
}

impl WorldGenerator {
    pub fn new() -> Self {
        WorldGenerator {
            noise: FastNoiseLite::new(),
        }
    }

    pub fn set_seed(&mut self, seed: i32) {
        self.noise.set_seed(Some(seed));
    }

    pub fn generate_chunk(&mut self, chunk: &mut Chunk) {
        if chunk.generated {
            return;
        }

        let chunk_pos = chunk.get_pos().clone();

        let chunk_block_pos = chunk_pos * CHUNK_SIZE_I;
        for x in 0..CHUNK_SIZE_I {
            for z in 0..CHUNK_SIZE_I {
                let negative_1_to_1 = self.noise.get_noise_2d(
                    (x + chunk_block_pos.x) as f32,
                    (z + chunk_block_pos.z) as f32,
                );
                let height = roundf((negative_1_to_1 + 1.) / 2. * 14.0 - 2.0) as isize;

                for y in 0..CHUNK_SIZE_I {
                    let block_y = chunk_block_pos.y + y;

                    if block_y == height {
                        chunk.set_at(
                            Vector3::new(x as usize, y as usize, z as usize),
                            crate::constants::BlockType::Grass,
                        );
                    }

                    if block_y < height && block_y >= height - 3 {
                        chunk.set_at(
                            Vector3::new(x as usize, y as usize, z as usize),
                            crate::constants::BlockType::Dirt,
                        );
                    }

                    if block_y < height - 3 {
                        chunk.set_at(
                            Vector3::new(x as usize, y as usize, z as usize),
                            crate::constants::BlockType::Stone,
                        );
                    }
                }
            }
        }

        // Generate a pseudo random seed
        let seed = ((chunk_pos.x as i64 + 2147483648) * 1000
            + (chunk_pos.y as i64 + 2147483648) * 1000000
            + (chunk_pos.z as i64 + 2147483648) * 1000000000
            + (self.noise.seed as i64 + 2147483648)) as u64;
        let mut rng = XorShiftRng::seed_from_u64(seed);

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if rng.next_u32() < u32::MAX / 1024 {
                    self.place_struct(chunk, &TREE1, Vector3::new(x, 1, z));
                }
            }
        }

        chunk.generated = true;
    }

    pub fn place_struct(
        &self,
        chunk: &mut Chunk,
        structure: &'static Structure,
        pos: Vector3<usize>,
    ) {
        for y in 0..structure.size.y {
            for x in 0..structure.size.x {
                for z in 0..structure.size.z {
                    if let Some(block) = structure.get_block_at(Vector3::new(x, y, z)) {
                        chunk.set_at(
                            pos + Vector3::new(x as usize, y as usize, z as usize),
                            block,
                        );
                    }
                }
            }
        }
    }
}
