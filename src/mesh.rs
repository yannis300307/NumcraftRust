#[cfg(target_os = "none")]
use alloc::vec::Vec;

use nalgebra::{Vector2, Vector3};
use strum::EnumIter;

use crate::{
    chunk::Chunk,
    constants::{BlockType, world::CHUNK_SIZE},
    eadk::{self, Color},
    world::World,
};

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

#[derive(PartialEq, Eq, EnumIter, Clone, Copy)]
pub enum QuadDir {
    Front = 1,
    Back = 2,
    Top = 3,
    Bottom = 4,
    Right = 5,
    Left = 6,
}

pub struct Quad {
    pub pos: Vector3<i16>,
    pub dir: QuadDir,
    pub color: eadk::Color,
}

impl Quad {
    pub fn get_triangles(&self) -> (Triangle, Triangle) {
        match self.dir {
            QuadDir::Front => (
                Triangle {
                    p3: Vector3::new(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32),
                    p2: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p1: Vector3::new(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    p3: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
            ),
            QuadDir::Back => (
                Triangle {
                    p1: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p3: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p3: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ), // TODO sort points from p1 to p3
                    color: self.color,
                },
            ),
            QuadDir::Top => (
                Triangle {
                    p3: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p1: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p2: Vector3::new(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32),
                    p3: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
            ),
            QuadDir::Bottom => (
                Triangle {
                    p1: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p3: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p3: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
            ),
            QuadDir::Right => (
                Triangle {
                    p1: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p3: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p3: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
            ),
            QuadDir::Left => (
                Triangle {
                    p3: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p1: Vector3::new(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32),
                    color: self.color,
                },
                Triangle {
                    p1: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + 1) as f32,
                    ),
                    p3: Vector3::new(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32),
                    color: self.color,
                },
            ),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
    pub p3: Vector3<f32>,
    pub color: eadk::Color,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub color: eadk::Color,
}

impl Triangle {
    pub fn get_normal(&self) -> Vector3<f32> {
        let a = self.p2 - self.p1;
        let b = self.p3 - self.p1;
        a.cross(&b).normalize()
    }
}

fn get_block_in_chunk_or_world(
    pos: Vector3<isize>,
    world: &World,
    chunk: &Chunk,
) -> Option<BlockType> {
    if pos.x < 0
        || pos.x >= CHUNK_SIZE_I
        || pos.y < 0
        || pos.y >= CHUNK_SIZE_I
        || pos.z < 0
        || pos.z >= CHUNK_SIZE_I
    {
        world.get_block_in_world(pos + *chunk.get_pos() * CHUNK_SIZE_I)
    } else {
        Some(chunk.get_at_unchecked(pos))
    }
}

pub struct Mesh {
    pub quads: Vec<Quad>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh { quads: Vec::new() }
    }

    pub fn generate_chunk(world: &World, chunk: &Chunk) -> Self {
        let mut quads = Vec::new();

        for x in 0..CHUNK_SIZE as isize {
            for y in 0..CHUNK_SIZE as isize {
                for z in 0..CHUNK_SIZE as isize {
                    if chunk.get_at(Vector3::new(x, y, z)).unwrap() != BlockType::Air {
                        let bloc_pos = Vector3::new(x, y, z) + *chunk.get_pos() * CHUNK_SIZE_I;
                        let bloc_pos = bloc_pos.map(|x| x as i16);

                        if get_block_in_chunk_or_world(Vector3::new(x, y, z - 1), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Front,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if get_block_in_chunk_or_world(Vector3::new(x, y, z + 1), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Back,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if get_block_in_chunk_or_world(Vector3::new(x + 1, y, z), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Right,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }
                        if get_block_in_chunk_or_world(Vector3::new(x - 1, y, z), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Left,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if get_block_in_chunk_or_world(Vector3::new(x, y - 1, z), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad {
                                pos: bloc_pos,
                                dir: QuadDir::Top,
                                color: Color {
                                    rgb565: 0b1111111111111111,
                                },
                            });
                        }

                        if get_block_in_chunk_or_world(Vector3::new(x, y + 1, z), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad {
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

        Mesh { quads: quads }
    }
}
