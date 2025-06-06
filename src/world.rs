use libm::roundf;

use crate::chunk::{self, Chunk};
use crate::constants::BlockType;
use crate::constants::world::CHUNK_SIZE;
use crate::mesh::{Mesh, Quad};
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use fastnoise_lite::FastNoiseLite;
use nalgebra::Vector3;

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct World {
    pub chunks: Vec<chunk::Chunk>,
    gen_noise: FastNoiseLite,
}

fn get_chunk_local_coords(pos: Vector3<isize>) -> Vector3<isize> {
    Vector3::new(
        if pos.x < 0 {
            CHUNK_SIZE_I - 1 + pos.x % CHUNK_SIZE_I
        } else {
            pos.x % CHUNK_SIZE_I
        },
        if pos.y < 0 {
            CHUNK_SIZE_I - 1 + pos.y % CHUNK_SIZE_I
        } else {
            pos.y % CHUNK_SIZE_I
        },
        if pos.z < 0 {
            CHUNK_SIZE_I - 1 + pos.z % CHUNK_SIZE_I
        } else {
            pos.z % CHUNK_SIZE_I
        },
    )
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

    fn get_chunk_at_pos(&self, pos: Vector3<isize>) -> Option<&Chunk> {
        for chunk in &self.chunks {
            if *chunk.get_pos() == pos {
                return Some(chunk);
            }
        }
        None
    }

    pub fn generate_world_around_pos(&mut self, pos: Vector3<f32>, render_distance: isize) {
        let pos_chunk_coords = Vector3::new(
            roundf(pos.x / CHUNK_SIZE as f32) as isize,
            roundf(pos.y / CHUNK_SIZE as f32) as isize,
            roundf(pos.z / CHUNK_SIZE as f32) as isize,
        );
        self.chunks.retain(|chunk| {
            let relative_chunk_pos = chunk.get_pos() - pos_chunk_coords;
            !(relative_chunk_pos.x < -render_distance
                || relative_chunk_pos.x >= render_distance
                || relative_chunk_pos.y < -render_distance
                || relative_chunk_pos.y >= render_distance
                || relative_chunk_pos.z < -render_distance || relative_chunk_pos.z >= render_distance)
        });
        for x in -render_distance..render_distance {
            for y in -render_distance..render_distance {
                for z in -render_distance..render_distance {
                    let chunk_pos: Vector3<isize> = Vector3::new(x, y, z) + pos_chunk_coords;

                    if !self.get_chunk_exists_at(chunk_pos) {
                        if self.add_chunk(chunk_pos).is_none() {
                            continue;
                        };
                        let chunk = self.chunks.last_mut().unwrap();

                        chunk.generate_chunk(&self.gen_noise);
                    }
                }
            }
        }

        for i in 0..self.chunks.len() {
            if self.chunks[i].need_new_mesh {
                let new_mesh = Mesh::generate_chunk(self, &self.chunks[i]);
                self.chunks[i].set_mesh(new_mesh);
            }
        }
    }

    pub fn get_mesh(&self) -> Vec<&Vec<Quad>> {
        let mut world_mesh = Vec::new();
        for chunk in &self.chunks {
            world_mesh.push(&chunk.get_mesh().quads);
        }

        world_mesh
    }

    pub fn get_block_in_world(&self, pos: Vector3<isize>) -> Option<BlockType> {
        self.get_chunk_at_pos(pos / CHUNK_SIZE_I).map(|chunk| chunk.get_at_unchecked(get_chunk_local_coords(pos)))
    }
}
