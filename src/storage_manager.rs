use alloc::vec::Vec;
use lz4_flex::compress;

use crate::chunk::Chunk;

struct ChunkStorage {
    x: i16,
    y: i16,
    z: i16,
    blocks: Vec<u8>
}

impl ChunkStorage {
    pub fn from_chunk(chunk: &Chunk) -> Self {
        let pos = chunk.get_pos();
        

        let compressed = compress(&chunk.get_all_blocks().map(|b| b as u8));
        ChunkStorage { x: pos.x as i16, y: pos.y as i16, z: pos.z as i16, blocks: compressed }
    }
}


/*
Save file format.

Header:
  x_world_size: u8
  y_world_size: u8
  z_world_size: u8

  
*/