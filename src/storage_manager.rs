use alloc::vec::Vec;
use miniz_oxide::deflate::compress_to_vec;

use crate::chunk::Chunk;

struct ChunkStorage {
    x: i16,
    y: i16,
    z: i16,
    blocks: Vec<u8>
}

struct SaveFile {
    
}

impl ChunkStorage {
    pub fn from_chunk(chunk: &Chunk) -> Self {
        let pos = chunk.get_pos();
        
        let compressed = compress_to_vec(&chunk.get_all_blocks().map(|b| b as u8), 6);
        ChunkStorage { x: pos.x as i16, y: pos.y as i16, z: pos.z as i16, blocks: compressed }
    }
}