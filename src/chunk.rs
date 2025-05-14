use crate::{
    constants::{BlockType, world::*},
    eadk::{Color},
    mesh::{BlockFace, BlockFaceDir},
};
use alloc::vec::Vec;
use fastnoise_lite::FastNoiseLite;
use nalgebra::{Vector2, Vector3};

const BLOCK_COUNT: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct Chunk {
    blocks: [BlockType; BLOCK_COUNT],
    pos: Vector3<isize>,
    mesh: Vec<BlockFace>,
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

    pub fn generate_chunk(&mut self, noise: &FastNoiseLite) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let negative_1_to_1 = noise.get_noise_2d(x as f32, z as f32);
                let height = 7; //(negative_1_to_1 + 1.) / 2. * 4.0;
                self.set_at(
                    Vector3::new(x, height as usize, z),
                    crate::constants::BlockType::Stone,
                );
            }
        }
        self.generated = true
    }

    pub fn get_mesh(&self) -> &Vec<BlockFace> {
        &self.mesh
    }

    fn add_face_to_mesh(&mut self, pos: Vector3<isize>, color: Color, dir: BlockFaceDir) {
        for face in self.mesh.iter_mut() {
            if face.dir == dir
                && face.pos.y == pos.y
                && face.pos.x == pos.x + 1
                && face.pos.z == pos.z
            {
                face.scale.x += 1;
                return;
            }
            if face.dir == dir
                && face.pos.y == pos.y
                && face.pos.x == pos.x - 1
                && face.pos.z == pos.z
            {
                face.scale.x += 1; // move to the left and add 1 to the x scale
                face.pos.x -= 1;
                return;
            }
        }
        // else add new face
        self.mesh.push(BlockFace {
            pos,
            scale: Vector2::new(1, 1),
            dir,
            color,
        });
    }

    pub fn generate_mesh(&mut self) {
        self.mesh.clear();

        for x in 0..CHUNK_SIZE as isize {
            for y in 0..CHUNK_SIZE as isize {
                for z in 0..CHUNK_SIZE as isize {
                    if self.get_at(Vector3::new(x, y, z)) != BlockType::Air {
                        let bloc_pos = Vector3::new(
                            x + self.pos.x * CHUNK_SIZE_I,
                            y + self.pos.y * CHUNK_SIZE_I,
                            z + self.pos.z * CHUNK_SIZE_I,
                        );

                        if self.get_at(Vector3::new(x, y, z - 1)) == BlockType::Air {
                            self.add_face_to_mesh(
                                bloc_pos,
                                Color {
                                    rgb565: 0b1111111111111111,
                                },
                                BlockFaceDir::Front,
                            );
                        }

                        if self.get_at(Vector3::new(x, y, z + 1)) == BlockType::Air {
                            self.add_face_to_mesh(
                                bloc_pos,
                                Color {
                                    rgb565: 0b1111111111111111,
                                },
                                BlockFaceDir::Back,
                            );
                        }

                        if self.get_at(Vector3::new(x + 1, y, z)) == BlockType::Air {
                            self.add_face_to_mesh(
                                bloc_pos,
                                Color {
                                    rgb565: 0b1111111111111111,
                                },
                                BlockFaceDir::Right,
                            );
                        }
                        if self.get_at(Vector3::new(x - 1, y, z)) == BlockType::Air {
                            self.add_face_to_mesh(
                                bloc_pos,
                                Color {
                                    rgb565: 0b1111111111111111,
                                },
                                BlockFaceDir::Left,
                            );
                        }

                        if self.get_at(Vector3::new(x, y - 1, z)) == BlockType::Air {
                            self.add_face_to_mesh(
                                bloc_pos,
                                Color {
                                    rgb565: 0b1111111111111111,
                                },
                                BlockFaceDir::Up,
                            );
                        }

                        if self.get_at(Vector3::new(x, y + 1, z)) == BlockType::Air {
                            self.add_face_to_mesh(
                                bloc_pos,
                                Color {
                                    rgb565: 0b1111111111111111,
                                },
                                BlockFaceDir::Down,
                            );
                        }
                    }
                }
            }
        }

        let mut face = 0;
        while face != self.mesh.len() {
            let mut candidate: Option<usize> = None;
            for other in face..self.mesh.len() {
                if self.mesh[other].dir == self.mesh[face].dir
                    && self.mesh[face].pos.y == self.mesh[other].pos.y
                    && self.mesh[face].pos.x == self.mesh[other].pos.x
                    && self.mesh[face].scale.x == self.mesh[other].scale.x
                    && self.mesh[face].pos.z + self.mesh[face].scale.y as isize
                        == self.mesh[other].pos.z
                {
                    candidate = Some(other);
                }
            }

            if let Some(to_merge) = candidate {
                self.mesh[face].scale.y += self.mesh[to_merge].scale.y;
                self.mesh[face].pos.z -= 1;
                self.mesh.remove(to_merge);
            }

            face += 1;
        }
    }
}
