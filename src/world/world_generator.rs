use fastnoise_lite::FastNoiseLite;
use libm::roundf;
use nalgebra::Vector3;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    constants::world::*,
    world::{
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
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(fastnoise_lite::NoiseType::OpenSimplex2));
        WorldGenerator { noise }
    }

    pub fn set_seed(&mut self, seed: i32) {
        self.noise.set_seed(Some(seed));
    }

    pub fn generate_chunk(
        &mut self,
        chunks_manager: &mut ChunksManager,
        chunk_pos: Vector3<isize>,
    ) {
        let chunk = chunks_manager.get_chunk_at_pos_mut(chunk_pos).unwrap(); // I assume that a valid pos is given to the generator

        if chunk.generated {
            return;
        }
        chunk.generated = true;

        let chunk_block_pos = chunk_pos * CHUNK_SIZE_I;

        let mut height_map = [0isize; CHUNK_SIZE * CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let negative_1_to_1 = self.noise.get_noise_2d(
                    (x as isize + chunk_block_pos.x) as f32,
                    (z as isize + chunk_block_pos.z) as f32,
                );
                let height = roundf((negative_1_to_1 + 1.) / 2. * 14.0 - 2.0) as isize;

                height_map[x + z * CHUNK_SIZE] = height;
            }
        }

        for x in 0..CHUNK_SIZE_I {
            for z in 0..CHUNK_SIZE_I {
                let height = height_map[x as usize + z as usize * CHUNK_SIZE];

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

        for x in 0..CHUNK_SIZE_I {
            for z in 0..CHUNK_SIZE_I {
                let world_pos = Vector3::new(
                    x + chunk_block_pos.x,
                    height_map[x as usize + z as usize * CHUNK_SIZE] + 1,
                    z + chunk_block_pos.z,
                );
                if rng.next_u32() < u32::MAX / 64 {
                    self.place_struct_check_space(
                        chunks_manager,
                        &TREE1,
                        world_pos - Vector3::new(1, 0, 1),
                        Vector3::new(1, 0, 1),
                    );
                }
            }
        }
    }

    /// Place a structure only if there is enough space
    pub fn place_struct_check_space(
        &self,
        chunks_manager: &mut ChunksManager,
        structure: &'static Structure,
        pos: Vector3<isize>,
        margins: Vector3<isize>,
    ) {
        for y in (-margins.y)..structure.size.y as isize + margins.y {
            for x in (-margins.x)..structure.size.x as isize + margins.x {
                for z in (-margins.z)..structure.size.z as isize + margins.z {
                    if !chunks_manager
                        .get_block_in_world(pos + Vector3::new(x as isize, y as isize, z as isize))
                        .is_none_or(|b| b.is_air())
                    {
                        return;
                    }
                }
            }
        }
        self.place_struct(chunks_manager, structure, pos);
    }

    pub fn place_struct(
        &self,
        chunks_manager: &mut ChunksManager,
        structure: &'static Structure,
        pos: Vector3<isize>,
    ) {
        for y in 0..structure.size.y {
            for x in 0..structure.size.x {
                for z in 0..structure.size.z {
                    if let Some(block) = structure.get_block_at(Vector3::new(x, y, z)) {
                        let dest_pos = pos + Vector3::new(x as isize, y as isize, z as isize);
                        chunks_manager.set_block_in_world(dest_pos, block);
                    }
                }
            }
        }
    }
}
