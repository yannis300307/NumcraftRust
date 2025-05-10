use crate::{
    constants::{world::*, BlockType},
    eadk::{self},
    mesh::{BlockFace, BlockFaceDir},
};
use alloc::vec::Vec;
use nalgebra::Vector3;

const BLOCK_COUNT: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct Chunk {
    blocks: [BlockType; BLOCK_COUNT],
    pos: Vector3<isize>,
    mesh: Vec<BlockFace>,
}

impl Chunk {
    pub fn new(pos: Vector3<isize>) -> Self {
        Chunk {
            blocks: [BlockType::Air; BLOCK_COUNT],
            pos,
            mesh: Vec::new(),
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

    pub fn get_mesh(&self) -> &Vec<BlockFace> {
        &self.mesh
    }

    pub fn generate_mesh(&mut self) {
        for x in 0..CHUNK_SIZE as isize {
            for y in 0..CHUNK_SIZE as isize {
                for z in 0..CHUNK_SIZE as isize {
                    if self.get_at(Vector3::new(x, y, z)) != BlockType::Air {
                        let bloc_pos = Vector3::new(
                            (x + self.pos.x * CHUNK_SIZE_I) as f32,
                            (y + self.pos.y * CHUNK_SIZE_I) as f32,
                            (z + self.pos.z * CHUNK_SIZE_I) as f32,
                        );

                        if self.get_at(Vector3::new(x, y, z - 1)) == BlockType::Air {
                            self.mesh.push(BlockFace {
                                pos: bloc_pos,
                                dir: BlockFaceDir::Front,
                                color: eadk::Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if self.get_at(Vector3::new(x, y, z + 1)) == BlockType::Air {
                            self.mesh.push(BlockFace {
                                pos: bloc_pos,
                                dir: BlockFaceDir::Back,
                                color: eadk::Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if self.get_at(Vector3::new(x + 1, y, z)) == BlockType::Air {
                            self.mesh.push(BlockFace {
                                pos: bloc_pos,
                                dir: BlockFaceDir::Right,
                                color: eadk::Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }
                        if self.get_at(Vector3::new(x - 1, y, z)) == BlockType::Air {
                            self.mesh.push(BlockFace {
                                pos: bloc_pos,
                                dir: BlockFaceDir::Left,
                                color: eadk::Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if self.get_at(Vector3::new(x, y - 1, z)) == BlockType::Air {
                            self.mesh.push(BlockFace {
                                pos: bloc_pos,
                                dir: BlockFaceDir::Up,
                                color: eadk::Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if self.get_at(Vector3::new(x, y + 1, z)) == BlockType::Air {
                            self.mesh.push(BlockFace {
                                pos: bloc_pos,
                                dir: BlockFaceDir::Down,
                                color: eadk::Color {
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
