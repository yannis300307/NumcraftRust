use alloc::vec::Vec;
use nalgebra::Vector3;
use crate::chunk::{self, Chunk};
use crate::constants::world::*;

struct World {
    chunks: Vec<chunk::Chunk>
}

impl World {
    fn add_chunk(&mut self, pos: Vector3<isize>) {
        self.chunks.push(Chunk::new(pos));
    }
}