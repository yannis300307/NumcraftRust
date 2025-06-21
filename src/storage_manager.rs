use alloc::vec::Vec;
use lz4_flex::compress;

use crate::{chunk::{self, Chunk}, constants::world::CHUNK_SIZE, storage_lib::storage_file_write};

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct SaveManager {
    chunks_data: [Vec<u8>; 64],
}

impl SaveManager {
    pub fn new() -> Self {
        SaveManager {
            chunks_data: [ const { Vec::new() }; 64],
        }
    }

    pub fn set_chunk(&mut self, chunk: &Chunk) -> bool {
        let pos = chunk.get_pos();

        if pos.x < 0 || pos.x >= 4 || pos.y < 0 || pos.y >= 4 || pos.z < 0 || pos.z >= 4 {
            return false;
        }

        let compressed = compress(&chunk.get_all_blocks().map(|b| b as u8));

        let index = (pos.x + pos.y * 4 + pos.z * 16) as usize;

        self.chunks_data[index] = compressed;
        
        true
    }

    pub fn save_world_to_file(&self) {
        let data = self.get_raw();

        storage_file_write("world.py", data.as_slice());
    }

    fn get_raw(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        for i in 0..self.chunks_data.len() {
            let size = self.chunks_data[i].len();
            data.push((size >> 8) as u8);
            data.push((size&0xFF) as u8);
        }

        for i in 0..self.chunks_data.len() {
            data.extend(&self.chunks_data[i]);
        }

        compress(&data)
    }

    fn load_from_file(&mut self) {
        // storage_extapp_fileRead("world.py");
    }
}

/*
Save file format. World is 4 x 4 x 4 chunks.

Header:
    4x4x4 x 2 B array : represent the compressed size of the chunk for each chunk

    4x4x4 x variable size : chunks data.
*/
