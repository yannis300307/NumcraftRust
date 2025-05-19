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

    pub fn generate_chunk(&mut self, noise: &FastNoiseLite) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let negative_1_to_1 = noise.get_noise_2d(x as f32, z as f32);
                let height = (negative_1_to_1 + 1.) / 2. * 4.0;
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

    pub fn generate_mesh(&mut self) {
        self.mesh.clear();

        for dir in QuadDir::iter() {
            for layer in 0..CHUNK_SIZE_I {
                let mut map: Bitmap<LAYER_SIZE_BITS> = Bitmap::new();
                // Generate the bitmap
                for u in 0..CHUNK_SIZE_I {
                    for v in 0..CHUNK_SIZE_I {
                        let can_render = match dir {
                            QuadDir::Top => {
                                !self.get_at(Vector3::new(u, layer, v)).is_air()
                                    && self.get_at(Vector3::new(u, layer - 1, v)).is_air()
                            }
                            QuadDir::Bottom => {
                                !self.get_at(Vector3::new(u, layer, v)).is_air()
                                    && self.get_at(Vector3::new(u, layer + 1, v)).is_air()
                            }
                            QuadDir::Front => {
                                !self.get_at(Vector3::new(u, v, layer)).is_air()
                                    && self.get_at(Vector3::new(u, v, layer - 1)).is_air()
                            }
                            QuadDir::Back => {
                                !self.get_at(Vector3::new(u, v, layer)).is_air()
                                    && self.get_at(Vector3::new(u, v, layer + 1)).is_air()
                            }
                            QuadDir::Right => {
                                !self.get_at(Vector3::new(layer, v, u)).is_air()
                                    && self.get_at(Vector3::new(layer + 1, v, u)).is_air()
                            }
                            QuadDir::Left => {
                                !self.get_at(Vector3::new(layer, v, u)).is_air()
                                    && self.get_at(Vector3::new(layer - 1, v, u)).is_air()
                            }
                        };
                        if can_render {
                            map.set((u + v * CHUNK_SIZE_I) as usize);
                        }
                    }
                }
                // Do the optimisation
                let rectangles = optimise_plane(&mut map);

                // Add the quads
                for rect in rectangles {
                    let pos = match dir {
                        QuadDir::Top | QuadDir::Bottom => Vector3::new(rect.0, layer, rect.1),
                        QuadDir::Front | QuadDir::Back => Vector3::new(rect.0, rect.1, layer),
                        QuadDir::Right | QuadDir::Left => Vector3::new(layer, rect.1, rect.0),
                    };
                    self.mesh.push(Quad {
                        pos: pos + self.pos * CHUNK_SIZE_I,
                        scale: Vector2::new(rect.2 as i8, rect.3 as i8),
                        dir,
                        color: Color {
                            rgb565: 0b1111111111111111,
                        },
                    });
                }
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
                'count_up: while y + height < CHUNK_SIZE_I {
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
