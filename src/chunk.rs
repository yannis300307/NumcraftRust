use crate::{
    constants::{BlockType, world::*},
    eadk::Color,
    mesh::{Quad, QuadDir},
};
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use cbitmap::bitmap::{Bitmap, BitsManage};
use fastnoise_lite::FastNoiseLite;
use nalgebra::{Vector2, Vector3};
use strum::IntoEnumIterator;

const BLOCK_COUNT: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;
const LAYER_SIZE_BITS: usize = (CHUNK_SIZE * CHUNK_SIZE).div_ceil(8);

pub struct Chunk {
    blocks: [BlockType; BLOCK_COUNT],
    pos: Vector3<isize>,
    mesh: Vec<Quad>,
    pub generated: bool,
}

impl Chunk {
    pub fn new(pos: Vector3<isize>) -> Self {
        Chunk {
            blocks: [BlockType::Air; BLOCK_COUNT],
            pos,
            mesh: Vec::new(),
            generated: false,
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

    pub fn get_at(&self, pos: Vector3<isize>) -> BlockType {
        if pos.x < CHUNK_SIZE_I
            && pos.y < CHUNK_SIZE_I
            && pos.z < CHUNK_SIZE_I
            && pos.x >= 0
            && pos.y >= 0
            && pos.z >= 0
        {
            self.blocks
                [(pos.x + pos.y * CHUNK_SIZE_I + pos.z * CHUNK_SIZE_I * CHUNK_SIZE_I) as usize]
        } else {
            BlockType::Air
        }
    }

    pub fn get_pos(&self) -> &Vector3<isize> {
        &self.pos
    }

    pub fn generate_chunk(&mut self, noise: &FastNoiseLite) {
        let chunk_block_pos = self.pos*CHUNK_SIZE_I;
        for x in 0..CHUNK_SIZE_I {
            for z in 0..CHUNK_SIZE_I {
                let negative_1_to_1 = noise.get_noise_2d((x + chunk_block_pos.x) as f32, (z + chunk_block_pos.z) as f32);
                let height = (negative_1_to_1 + 1.) / 2. * 8.0;

                self.set_at(
                    Vector3::new(x as usize, height as usize, z as usize),
                    crate::constants::BlockType::Stone,
                );
            }
        }
        self.generated = true
    }

    pub fn get_mesh(&self) -> &Vec<Quad> {
        &self.mesh
    }

    pub fn generate_mesh(&mut self) {
        self.mesh.clear();

        for x in 0..CHUNK_SIZE as isize {
            for y in 0..CHUNK_SIZE as isize {
                for z in 0..CHUNK_SIZE as isize {
                    if self.get_at(Vector3::new(x, y, z)) != BlockType::Air {
                        let bloc_pos = Vector3::new(x, y, z) + self.pos * CHUNK_SIZE_I;

                        if self.get_at(Vector3::new(x, y, z - 1)) == BlockType::Air {
                            self.mesh.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Front,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if self.get_at(Vector3::new(x, y, z + 1)) == BlockType::Air {
                            self.mesh.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Back,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if self.get_at(Vector3::new(x + 1, y, z)) == BlockType::Air {
                            self.mesh.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Right,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }
                        if self.get_at(Vector3::new(x - 1, y, z)) == BlockType::Air {
                            self.mesh.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Left,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if self.get_at(Vector3::new(x, y - 1, z)) == BlockType::Air {
                            self.mesh.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Top,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if self.get_at(Vector3::new(x, y + 1, z)) == BlockType::Air {
                            self.mesh.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Bottom,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }
                    }
                }
            }
        }
    }
}
