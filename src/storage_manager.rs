use alloc::vec::Vec;
use lz4_flex::compress;

use crate::{chunk::Chunk, constants::world::CHUNK_SIZE};

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

struct SaveFile {
    indexes: [u16; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
    chunks_data: Vec<Vec<u8>>,
    last_pos: u16
}

impl SaveFile {
    pub fn new() -> Self {
        SaveFile {
            indexes: [0; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            chunks_data: Vec::new(),
            last_pos: 0
        }
    }

    pub fn add_chunk(&mut self, chunk: &Chunk) -> bool {
        let pos = chunk.get_pos();

        if pos.x < 0 || pos.x >= 4 || pos.y < 0 || pos.y >= 4 || pos.z < 0 || pos.z >= 4 {
            return false;
        }

        let compressed = compress(&chunk.get_all_blocks().map(|b| b as u8));

        let data_len = compressed.len();

        self.chunks_data.push(compressed);

        let index = (pos.x + pos.y * CHUNK_SIZE_I + pos.z * CHUNK_SIZE_I * CHUNK_SIZE_I) as usize;

        self.indexes[index] = self.last_pos;

        self.last_pos += data_len as u16;
        
        true
    }
}

/*
Save file format. World is 4 x 4 x 4 chunks.

Header:
    4x4x4 x 3 B array :
        3 B per chunks: 1B for world chunk position, 2 B as u16 for chunk index position in save file

    4x4x4 variable size chunks data.
*/
