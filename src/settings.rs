use postcard::from_bytes;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{rendering::*, save_manager::*}, nadk::storage::{file_erase, file_exists, file_read, file_write},
};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub render_distance: usize,
    pub fov: f32,
    pub vsync: bool,
    pub reverse_controls: bool,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            render_distance: MAX_RENDER_DISTANCE,
            fov: FOV,
            vsync: false,
            reverse_controls: false
        }
    }

    pub fn save(&self) {
        if file_exists(SETTINGS_FILENAME) {
            file_erase(SETTINGS_FILENAME);
        }
        let raw = postcard::to_allocvec(self).unwrap();

        file_write(SETTINGS_FILENAME, &raw);
    }

    pub fn load(&mut self) {
        if file_exists(SETTINGS_FILENAME) {
            let raw = file_read(SETTINGS_FILENAME).unwrap();

            let object: Settings = from_bytes(&raw).unwrap_or(Settings::new());

            *self = object;
        }
    }
}
