use alloc::{format, vec::Vec};
use lz4_flex::{compress, compress_prepend_size, decompress, decompress_size_prepended};
use nalgebra::Vector3;

use crate::{
    chunk::Chunk,
    constants::{world::CHUNK_SIZE, BlockType},
    storage_lib::{storage_extapp_fileErase, storage_extapp_fileExists, storage_extapp_fileRead, storage_file_write},
};

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct SaveManager {
    chunks_data: [Vec<u8>; 64],
}

impl SaveManager {
    pub fn new() -> Self {
        SaveManager {
            chunks_data: [const { Vec::new() }; 64],
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

        if storage_extapp_fileExists("world.ncw") {
            storage_extapp_fileErase("world.ncw");
        }
        storage_file_write("world.ncw", &data);
    }

    fn get_raw(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        for i in 0..self.chunks_data.len() {
            let size: u16 = self.chunks_data[i].len() as u16;
            data.extend_from_slice(&size.to_be_bytes());
        }

        for i in 0..self.chunks_data.len() {
            data.extend(&self.chunks_data[i]);
        }

        compress_prepend_size(&data)
    }

    pub fn load_from_file(&mut self) -> Result<(), SaveFileLoadError> {
        if let Some(raw_data) = storage_extapp_fileRead("world.ncw") {
            if let Ok(data) = decompress_size_prepended(&raw_data) {
                let mut current_pos = 128;
                for i in 0..64 {
                    let size = u16::from_be_bytes([data[i * 2], data[i * 2 + 1]]) as usize;

                    if current_pos + size > data.len() {
                        return Err(SaveFileLoadError::CorruptedWorld);
                    }
                    let raw_chunk = &data[current_pos..(current_pos + size)];

                    self.chunks_data[i] = raw_chunk.to_vec();

                    current_pos += size;
                }
                Ok(())
            } else {
                Err(SaveFileLoadError::CorruptedWorld)
            }
        } else {
            Err(SaveFileLoadError::FileNotFound)
        }
    }

    pub fn get_chunk_at_pos(&self, pos: Vector3<isize>) -> Result<Chunk, ChunkReadingError> {
        if pos.x < 0 || pos.x >= 4 || pos.y < 0 || pos.y >= 4 || pos.z < 0 || pos.z >= 4 {
            return Err(ChunkReadingError::OOBChunk);
        }

        let index = (pos.x + pos.y * 4 + pos.z * 16) as usize;

        let raw_chunk = &self.chunks_data[index];

        if let Ok(chunk_data) = decompress(raw_chunk, 512) {
            if chunk_data.len() != 512 {
                return Err(ChunkReadingError::CorruptedChunk);
            }

            let mut chunk = Chunk::new(pos);

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        if let Some(block_type) = BlockType::get_from_id(
                            chunk_data[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE],
                        ) {
                            chunk.set_at(Vector3::new(x, y, z), block_type);
                        } else {
                            return Err(ChunkReadingError::CorruptedChunk);
                        }
                    }
                }
            }

            Ok(chunk)
        } else {
            Err(ChunkReadingError::CorruptedChunk)
        }
    }

    pub fn clean(&mut self) {
        for chunk in self.chunks_data.iter_mut() {
            chunk.clear();
        }
    }
}

#[derive(Debug)]
pub enum ChunkReadingError {
    OOBChunk,
    CorruptedChunk,
}

#[derive(Debug)]
pub enum SaveFileLoadError {
    FileNotFound,
    CorruptedWorld,
}

/*
Save file format. World is 4 x 4 x 4 chunks.

Header:
    4x4x4 x 2 B array : represent the compressed size of the chunk for each chunk

    4x4x4 x variable size : chunks data.
*/
