use postcard::from_bytes;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{rendering::*, save_manager::*},
    storage_lib::{
        storage_extapp_file_erase, storage_extapp_file_exists, storage_extapp_file_read,
        storage_file_write,
    },
};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub render_distance: usize,
    pub fov: f32,
    pub vsync: bool,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            render_distance: MAX_RENDER_DISTANCE,
            fov: FOV,
            vsync: false,
        }
    }

    pub fn save(&self) {
        if storage_extapp_file_exists(SETTINGS_FILENAME) {
            storage_extapp_file_erase(SETTINGS_FILENAME);
        }
        let raw = postcard::to_allocvec(self).unwrap();

        storage_file_write(SETTINGS_FILENAME, &raw);
    }

    pub fn load(&mut self) {
        if storage_extapp_file_exists(SETTINGS_FILENAME) {
            let raw = storage_extapp_file_read(SETTINGS_FILENAME).unwrap();

            let object: Settings = from_bytes(&raw).unwrap();

            *self = object;
        }
    }
}
