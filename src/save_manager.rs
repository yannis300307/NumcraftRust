#[cfg(target_os = "none")]
use alloc::{string::String, vec::Vec};

#[cfg(target_os = "none")]
use crate::alloc::borrow::ToOwned;

use lz4_flex::{compress, compress_prepend_size, decompress, decompress_size_prepended};
use nalgebra::Vector3;
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

use crate::{
    constants::{BlockType, save_manager::WORLD_VERSION, world::CHUNK_SIZE},
    eadk::{self},
    game::GameMode,
    inventory::Inventory,
    player::Player,
    renderer::Renderer,
    storage_lib::{
        storage_extapp_file_erase, storage_extapp_file_exists,
        storage_extapp_file_list_with_extension, storage_extapp_file_read,
        storage_extapp_file_read_header, storage_file_write,
    },
    world::{World, chunk::Chunk},
};

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub pos: (f32, f32, f32),
    pub rotation: (f32, f32), // Only Pitch and Yaw
    pub inventory: Inventory, // More in the futur
}

#[derive(Serialize, Deserialize)]
pub struct WorldInfo {
    pub world_version: u16,
    pub world_name: String,
    pub world_seed: i32,
    pub gamemode: GameMode,
}

impl WorldInfo {
    pub fn new() -> Self {
        WorldInfo {
            world_version: WORLD_VERSION,
            world_name: String::new(),
            world_seed: 1,
            gamemode: GameMode::Survival,
        }
    }
}

impl PlayerData {
    pub fn new() -> Self {
        PlayerData {
            pos: (0., 0., 0.),
            rotation: (0., 0.),
            inventory: Inventory::new(0),
        }
    }
}

pub struct SaveManager {
    chunks_data: [Vec<u8>; 64],
    player_data: PlayerData,
    world_info: WorldInfo,
    pub file_name: Option<String>,
}

impl SaveManager {
    pub fn new() -> Self {
        SaveManager {
            chunks_data: [const { Vec::new() }; 64],
            player_data: PlayerData::new(),
            world_info: WorldInfo::new(),
            file_name: None,
        }
    }

    pub fn get_current_loaded_world_info(&self) -> &WorldInfo {
        &self.world_info
    }

    pub fn set_world_seed(&mut self, seed: i32) {
        self.world_info.world_seed = seed;
    }

    pub fn set_world_name(&mut self, world_name: &String) {
        self.world_info.world_name = world_name.clone();
    }

    pub fn set_gamemode(&mut self, gamemode: GameMode) {
        self.world_info.gamemode = gamemode;
    }

    pub fn get_game_mode(&self) -> GameMode {
        self.world_info.gamemode
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

    pub fn set_file_name(&mut self, file_name: &String) {
        self.file_name = Some(file_name.clone());
    }

    pub fn update_player_data(&mut self, world: &World, player: &Player) {
        let player_entity = world.get_player_entity();

        self.player_data.pos.0 = player_entity.pos.x;
        self.player_data.pos.1 = player_entity.pos.y;
        self.player_data.pos.2 = player_entity.pos.z;

        self.player_data.rotation.0 = player_entity.rotation.x;
        self.player_data.rotation.1 = player_entity.rotation.y;

        self.player_data.inventory = player.inventory.clone();
    }

    pub fn get_existing_worlds(&self) -> Vec<String> {
        storage_extapp_file_list_with_extension(4, "ncw")
    }

    pub fn delete_world(&self, world_name: &String) {
        if storage_extapp_file_exists(world_name) {
            storage_extapp_file_erase(world_name);
        }
    }

    pub fn save_world_to_file(&self) {
        let data = self.get_raw();

        if let Some(file_name) = &self.file_name {
            if storage_extapp_file_exists(file_name) && !storage_extapp_file_erase(file_name) {
                Renderer::show_msg(
                    &["Unable to save.", "Cannot delete old save!"],
                    eadk::Color::from_888(255, 100, 100),
                );
                eadk::timing::msleep(3000);
            }
            if !storage_file_write(file_name, &data) {
                Renderer::show_msg(
                    &["Unable to save.", "Writing error!"],
                    eadk::Color::from_888(255, 100, 100),
                );
                eadk::timing::msleep(3000);
            }
        } else {
            Renderer::show_msg(&["Unable to save."], eadk::Color::from_888(255, 100, 100));
            eadk::timing::msleep(3000);
        }
    }

    fn get_raw(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        let raw_world_info = to_allocvec(&self.world_info).unwrap();
        data.extend((raw_world_info.len() as u16).to_be_bytes());
        data.extend(raw_world_info);

        let mut data_to_compress: Vec<u8> = Vec::new();

        for i in 0..self.chunks_data.len() {
            let size: u16 = self.chunks_data[i].len() as u16;
            data_to_compress.extend_from_slice(&size.to_be_bytes());
        }

        for i in 0..self.chunks_data.len() {
            data_to_compress.extend(&self.chunks_data[i]);
        }

        let raw_player_data = to_allocvec(&self.player_data).unwrap();
        data_to_compress.extend((raw_player_data.len() as u16).to_be_bytes());
        data_to_compress.extend(raw_player_data);

        data.extend_from_slice(&compress_prepend_size(&data_to_compress));

        data
    }

    fn read_world_info(&mut self, data: &[u8]) -> Result<usize, SaveFileLoadError> {
        let mut world_data_offset = 0;
        // If world info is missing, the world is currupted
        if world_data_offset + 1 >= data.len() {
            return Err(SaveFileLoadError::CorruptedWorld);
        }

        // Extract world info
        let world_info_size =
            u16::from_be_bytes([data[world_data_offset], data[world_data_offset + 1]]) as usize;

        world_data_offset += 2; // world info size

        // Check for overflow
        if world_data_offset + world_info_size > data.len() {
            return Err(SaveFileLoadError::CorruptedWorld);
        }

        // Read the raw data
        let world_info_raw = &data[world_data_offset..(world_data_offset + world_info_size)];

        if let Ok(world_info) = from_bytes::<WorldInfo>(world_info_raw) {
            self.world_info = world_info;
            Ok(world_data_offset + world_info_size)
        } else {
            return Err(SaveFileLoadError::CorruptedWorld);
        }
    }

    pub fn get_world_info(&self, filename: &String) -> Option<WorldInfo> {
        let raw_data = storage_extapp_file_read_header(filename, 2)?;
        let world_info_size = u16::from_be_bytes([raw_data[0], raw_data[1]]);
        let raw_data =
            &storage_extapp_file_read_header(filename, world_info_size as usize + 2)?[2..];

        if let Ok(world_info) = from_bytes::<WorldInfo>(&raw_data) {
            Some(world_info)
        } else {
            None
        }

        //Some(WorldInfo::new())
    }

    pub fn load_from_file(&mut self, filename: &str) -> Result<(), SaveFileLoadError> {
        self.file_name = Some(filename.to_owned());
        // Read file
        if let Some(raw_data) = storage_extapp_file_read(filename) {
            if let Ok(world_data_offset) = self.read_world_info(&raw_data) {
                // Decompress the entire file
                if let Ok(data) = decompress_size_prepended(&raw_data[world_data_offset..]) {
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
        Vector3::new(
            self.player_data.pos.0,
            self.player_data.pos.1,
            self.player_data.pos.2,
        )
    }

    pub fn get_player_inventory(&self) -> Inventory {
        self.player_data.inventory.clone()
    }

    pub fn get_player_rot(&self) -> Vector3<f32> {
        Vector3::new(self.player_data.rotation.0, self.player_data.rotation.1, 0.)
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

    2 + variable : Player info

    2 + variable : World Info
*/
