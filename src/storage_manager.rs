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
Save file format. World is 4 x 4 x 4 chunks.

Header:
    4x4x4 x 3 B array : 
        3 B per chunks: 1B for world chunk position, 2 B as u16 for chunk index position in save file
    
    4x4x4 variable size chunks data.
  
*/