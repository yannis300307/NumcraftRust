use core::ops::BitAnd;

use crate::{
    constants::{BlockType, world::*},
    eadk::Color,
    mesh::{Quad, QuadDir},
};
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use cbitmap::bitmap::{self, Bitmap, BitsManage};
use fastnoise_lite::FastNoiseLite;
use nalgebra::{Vector2, Vector3};

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

    pub fn generate_chunk(&mut self, noise: &FastNoiseLite) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let negative_1_to_1 = noise.get_noise_2d(x as f32, z as f32);
                let height = 7; //(negative_1_to_1 + 1.) / 2. * 4.0;
                if x == 3 && z == 4 {
                    continue;
                }
                self.set_at(
                    Vector3::new(x, height as usize, z),
                    crate::constants::BlockType::Stone,
                );
            }
        }
        self.generated = true
    }

    pub fn get_mesh(&self) -> &Vec<Quad> {
        &self.mesh
    }

    fn add_face_to_mesh(&mut self, pos: Vector3<isize>, color: Color, dir: QuadDir) {
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
        self.mesh.push(Quad {
            pos,
            scale: Vector2::new(1, 1),
            dir,
            color,
        });
    }

    pub fn generate_mesh(&mut self) {
        self.mesh.clear();

        /*for x in 0..CHUNK_SIZE as isize {
            for y in 0..CHUNK_SIZE as isize {
                for z in 0..CHUNK_SIZE as isize {
                    if self.get_at(Vector3::new(x, y, z)) != BlockType::Air {
                        let bloc_pos = Vector3::new(
                            x + self.pos.x * CHUNK_SIZE_I,
                            y + self.pos.y * CHUNK_SIZE_I,
                            z + self.pos.z * CHUNK_SIZE_I,
                        );

                        /*if self.get_at(Vector3::new(x, y, z - 1)) == BlockType::Air {
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
                        }*/

                        if self.get_at(Vector3::new(x, y - 1, z)) == BlockType::Air {
                            self.add_face_to_mesh(
                                bloc_pos,
                                Color {
                                    rgb565: 0b1111111111111111,
                                },
                                QuadDir::Up,
                            );
                        }

                        /*if self.get_at(Vector3::new(x, y + 1, z)) == BlockType::Air {
                            self.add_face_to_mesh(
                                bloc_pos,
                                Color {
                                    rgb565: 0b1111111111111111,
                                },
                                BlockFaceDir::Down,
                            );
                        }*/
                    }
                }
            }
        }*/

        /*for layer in 0..CHUNK_SIZE_I {
            let mut processed_mask = [false; CHUNK_SIZE * CHUNK_SIZE];
            for v in 0..CHUNK_SIZE_I {
                // On a 2D plane
                let mut lenght = 0;
                let mut last_type = self.get_at(Vector3::new(0, layer, 0));
                for u in 0..CHUNK_SIZE_I {
                    let current_block_type = self.get_at(Vector3::new(u, layer, v));
                    if current_block_type != last_type
                        || u == CHUNK_SIZE_I - 1
                        || self.get_at(Vector3::new(u, layer - 1, v)) != BlockType::Air
                        || processed_mask[u as usize + CHUNK_SIZE * (v as usize)]
                    {
                        if lenght > 0 && last_type != BlockType::Air {
                            // Create quad
                            if u == CHUNK_SIZE_I - 1 {
                                lenght += 1
                            }

                            // Count the maximum height we can
                            let mut height = 1;
                            let mut last_type2 = current_block_type;
                            'v2: for v2 in v..CHUNK_SIZE_I {
                                for u2 in u..CHUNK_SIZE_I {
                                    let current_block_type2 = self.get_at(Vector3::new(u2, layer, v2));
                                    if current_block_type2 != last_type2
                                        || self.get_at(Vector3::new(u2, layer - 1, v2))
                                            != BlockType::Air
                                    {
                                        break 'v2;
                                    }
                                    last_type2 = current_block_type2;
                                }
                                for u2 in u as usize..(u+lenght) as usize {
                                    processed_mask[u2 + CHUNK_SIZE * (v2 as usize)] = true;
                                }

                                height += 1;
                            }

                            self.mesh.push(Quad {
                                pos: Vector3::new(u - 1, layer, v) + self.pos * CHUNK_SIZE_I,
                                scale: Vector2::new(lenght as i8, height as i8),
                                dir: QuadDir::Up,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                            lenght = 0;
                        }
                    } else {
                        lenght += 1;
                    }
                    processed_mask[u as usize + CHUNK_SIZE * v as usize] = true;
                    last_type = current_block_type;
                }
            }
        }*/

        /*let mut face = 0;
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
        }*/
        for layer in 0..CHUNK_SIZE_I {
            let mut map: Bitmap<LAYER_SIZE_BITS> = Bitmap::new();
            // Generate the bitmap
            for u in 0..CHUNK_SIZE_I {
                for v in 0..CHUNK_SIZE_I {
                    if self.get_at(Vector3::new(u, layer, v)) != BlockType::Air {
                        map.set((u + v * CHUNK_SIZE_I) as usize);
                    }
                }
            }
            // Do the optimisation
            let rectangles = optimise_plane(&mut map);

            for rect in rectangles {
                self.mesh.push(Quad {
                    pos: Vector3::new(rect.0, layer, rect.1),
                    scale: Vector2::new(rect.2 as i8, rect.3 as i8),
                    dir: QuadDir::Top,
                    color: Color {
                        rgb565: 0b1111111111111111,
                    },
                });
            }
        }
    }
}

fn optimise_plane(map: &mut Bitmap<LAYER_SIZE_BITS>) -> Vec<(isize, isize, isize, isize)> {
    let mut rectangles: Vec<(isize, isize, isize, isize)> = Vec::new();

    // Iterate horizontaly
    let mut y = 0;

    while y < CHUNK_SIZE_I {
        let mut x = 0;
        while x < CHUNK_SIZE_I {
            // If we encounter a 1, we start counting the rectangle width
            if map.get_bool((x + y * CHUNK_SIZE_I) as usize) {
                let mut lenght = 0;
                while x < CHUNK_SIZE_I && map.get_bool((x + y * CHUNK_SIZE_I) as usize) {
                    x += 1;
                    lenght += 1;
                }
                
                // next, start counting up.
                let mut height = 0;
                'count_up: while y+height < CHUNK_SIZE_I {
                    // Check if we have the same space at this level
                    for i in x - lenght..x {
                        // If we encounter a 0, we stop counting up
                        if !map.get_bool((i + (y + height) * CHUNK_SIZE_I) as usize) {
                            break 'count_up;
                        }
                    }
                    // If this height is fine, add one to the height counter
                    height += 1
                }
                // Add the rectangle to the vector
                rectangles.push((x - lenght, y, lenght, height));

                // Then, remove all the 1 we have already processed
                for x1 in x - lenght..x {
                    for y1 in y..y + height {
                        map.reset((x1 + y1 * CHUNK_SIZE_I) as usize);
                    }
                }
            }
            x += 1;
        }
        y += 1
    }
    rectangles
}
