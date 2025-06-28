use alloc::{string::String, vec::Vec};
use lz4_flex::{compress, compress_prepend_size, decompress, decompress_size_prepended};
use nalgebra::Vector3;
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    chunk::Chunk,
    constants::{BlockType, world::CHUNK_SIZE},
    player::Player,
    storage_lib::{
        storage_extapp_file_erase, storage_extapp_file_exists,
        storage_extapp_file_list_with_extension, storage_extapp_file_read, storage_file_write,
    },
};

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub pos: (f32, f32, f32),
    pub rotation: (f32, f32), // Only Pitch and Yaw
                              // More in the futur
}

impl PlayerData {
    pub fn new() -> Self {
        PlayerData {
            pos: (0., 0., 0.),
            rotation: (0., 0.),
        }
    }
}

pub struct SaveManager {
    chunks_data: [Vec<u8>; 64],
    player_data: PlayerData,
}

impl SaveManager {
    pub fn new() -> Self {
        SaveManager {
            chunks_data: [const { Vec::new() }; 64],
            player_data: PlayerData::new(),
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

    pub fn update_player_data(&mut self, player: &Player) {
        self.player_data.pos.0 = player.pos.x;
        self.player_data.pos.1 = player.pos.y;
        self.player_data.pos.2 = player.pos.z;

        self.player_data.rotation.0 = player.rotation.x;
        self.player_data.rotation.1 = player.rotation.y;
    }

    pub fn get_existing_worlds(&self) -> Vec<String> {
        storage_extapp_file_list_with_extension(4, "ncw")
    }

    pub fn save_world_to_file(&self, filename: &str) {
        let data = self.get_raw();

        if storage_extapp_file_exists(filename) {
            storage_extapp_file_erase(filename);
        }
        storage_file_write(filename, &data);
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

        let raw_player_data = to_allocvec(&self.player_data).unwrap();
        data.extend((raw_player_data.len() as u16).to_be_bytes());
        data.extend(raw_player_data);

        compress_prepend_size(&data)
    }

    pub fn load_from_file(&mut self, filename: &str) -> Result<(), SaveFileLoadError> {
        if let Some(raw_data) = storage_extapp_file_read(filename) {
            // Read file
            if let Ok(data) = decompress_size_prepended(&raw_data) {
                // Decompress the entire file
                let mut current_pos = 128;
                for i in 0..64 {
                    let size = u16::from_be_bytes([data[i * 2], data[i * 2 + 1]]) as usize; // Get the compressed chunk size from the headers

                    if current_pos + size > data.len() {
                        // Check for corruption. If overflow, the size is wrong and the world is ... unusable ...
                        return Err(SaveFileLoadError::CorruptedWorld);
                    }
                    let raw_chunk = &data[current_pos..(current_pos + size)];

                    self.chunks_data[i] = raw_chunk.to_vec();

                    current_pos += size;
                }

                // If player data is missing, the world is currupted
                if current_pos + 1 >= data.len() {
                    return Err(SaveFileLoadError::CorruptedWorld);
                }

                // Extract player_data
                let player_data_size =
                    u16::from_be_bytes([data[current_pos], data[current_pos + 1]]) as usize;

                current_pos += 2; // player data size

                // Check for overflow
                if current_pos + player_data_size > data.len() {
                    return Err(SaveFileLoadError::CorruptedWorld);
                }

                // Read the raw data
                let player_data_raw = &data[current_pos..(current_pos + player_data_size)];

                if let Ok(player_data) = from_bytes::<PlayerData>(player_data_raw) {
                    self.player_data = player_data;
                } else {
                    return Err(SaveFileLoadError::CorruptedWorld);
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

    pub fn get_player_pos(&self) -> Vector3<f32> {
        Vector3::new(self.player_data.pos.0,self.player_data.pos.1, self.player_data.pos.2)
    }

    pub fn get_player_rot(&self) -> Vector3<f32> {
        Vector3::new(self.player_data.rotation.0,self.player_data.rotation.1, 0.)
    }

    pub fn clean(&mut self) {
        for chunk in self.chunks_data.iter_mut() {
            chunk.clear();
        }

        self.player_data = PlayerData::new();
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
