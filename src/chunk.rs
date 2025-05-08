use crate::{
    constants::{BlockType, world::*},
    eadk::{self, Color},
    mesh::{Quad, QuadDir},
};
use alloc::vec::Vec;
use nalgebra::Vector3;

const BLOCK_COUNT: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct Chunk {
    blocks: [BlockType; BLOCK_COUNT],
    pos: Vector3<isize>,
}

impl Chunk {
    pub fn new(pos: Vector3<isize>) -> Self {
        Chunk {
            blocks: [BlockType::Air; BLOCK_COUNT],
            pos,
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

    pub fn get_at(&self, pos: Vector3<usize>) -> BlockType {
        if pos.x < CHUNK_SIZE && pos.y < CHUNK_SIZE && pos.z < CHUNK_SIZE {
            self.blocks[pos.x + pos.y * CHUNK_SIZE + pos.z * CHUNK_SIZE * CHUNK_SIZE]
        } else {
            BlockType::Air
        }
    }

    pub fn add_mesh_to_world_mesh(&self, world_mesh: &mut Vec<Quad>) {
        let chunk_world_pos = self.pos * (CHUNK_SIZE as isize);
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if self.get_at(Vector3::new(x, y, z)) != BlockType::Air {
                        let bloc_pos = Vector3::new(x as f32, y as f32, z as f32) + Vector3::<f32>::new(chunk_world_pos.x as f32, chunk_world_pos.y as f32, chunk_world_pos.z as f32); // TODO : MAKE CLEANER
                        world_mesh.push(Quad {pos: bloc_pos, dir: QuadDir::Front,color: eadk::Color {rgb565: 0b1111111111111111}});
                        world_mesh.push(Quad {pos: bloc_pos, dir: QuadDir::Back,color: eadk::Color {rgb565: 0b1111111111111111}});
                        world_mesh.push(Quad {pos: bloc_pos, dir: QuadDir::Right,color: eadk::Color {rgb565: 0b1111111111111111}});
                        world_mesh.push(Quad {pos: bloc_pos, dir: QuadDir::Left,color: eadk::Color {rgb565: 0b1111111111111111}});
                        world_mesh.push(Quad {pos: bloc_pos, dir: QuadDir::Up,color: eadk::Color {rgb565: 0b1111111111111111}});
                        world_mesh.push(Quad {pos: bloc_pos, dir: QuadDir::Down,color: eadk::Color {rgb565: 0b1111111111111111}});

                    }
                }
            }
        }
    }
}
