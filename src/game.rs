#[cfg(target_os = "none")]
use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
};
use nalgebra::{Vector2, Vector3};
use postcard::from_bytes;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{
        menu::{MENU_BACKGROUND_COLOR, SETTINGS_FILENAME},
        rendering::{FOV, MAX_FOV, MAX_RENDER_DISTANCE, MIN_FOV},
    }, eadk::{self, input::KeyboardState, Color, Point, SCREEN_RECT}, game_ui::{ContainerNeighbors, GameUI}, input_manager::InputManager, inventory::{Inventory, ItemStack}, menu::{Menu, MenuElement, TextAnchor}, player::Player, renderer::Renderer, save_manager::SaveManager, settings::Settings, storage_lib::{
        storage_extapp_file_erase, storage_extapp_file_exists, storage_extapp_file_read,
        storage_file_write,
    }, world::World
};

mod game_menus;
mod game_uis;

pub struct Game {
    renderer: Renderer,
    world: World,
    player: Player,
    last_keyboard_state: KeyboardState,
    save_manager: SaveManager,
    settings: Settings,
    input_manager: InputManager,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: Renderer::new(),
            world: World::new(),
            player: Player::new(),
            last_keyboard_state: KeyboardState::new(),
            save_manager: SaveManager::new(),
            settings: Settings::new(),
            input_manager: InputManager::new(),
        }
    }

    /// The game loop. Handle physic, rendering etc ...
    pub fn game_loop(&mut self, file_name: &String) -> GameState {
        // Load the world or create it if it doesn't exists yet
        if self.save_manager.load_from_file(file_name.as_str()).is_ok()
        // TODO: Show an error message instead
        {
            // Add chunks. Maybe move this code into world (TODO)
            for x in 0..4 {
                for y in 0..4 {
                    for z in 0..4 {
                        let chunk = self
                            .save_manager
                            .get_chunk_at_pos(Vector3::new(x, y, z))
                            .unwrap();

                        self.world.push_chunk(chunk);
                    }
                }
            }

            // Load player data
            self.player.set_pos_rotation(
                &mut self.renderer.camera,
                self.save_manager.get_player_rot(),
                self.save_manager.get_player_pos(),
            );

            // Load world info
            let world_info = self.save_manager.get_current_loaded_world_info();
            self.world.set_seed(world_info.world_seed);
        } else {
            self.world.load_area(0, 4, 0, 4, 0, 4);

            let player_spawn_pos = Vector3::new(
                16.,
                (self.world.get_terrain_height(Vector2::new(16, 16)) - 2) as f32,
                16.,
            );
            self.player.set_pos_rotation(
                &mut self.renderer.camera,
                Vector3::new(0., 0., 0.),
                player_spawn_pos,
            );
        }

        self.save_manager.clean(); // Clear save manager to save memory

        // Show a warning messge
        eadk::display::push_rect_uniform(SCREEN_RECT, Color::from_888(255, 255, 255));
        let show_msg = |message, y| {
            eadk::display::draw_string(
                message,
                Point {
                    x: ((320 - message.len() * 10) / 2) as u16,
                    y,
                },
                true,
                Color::from_888(0, 0, 0),
                Color::from_888(255, 255, 255),
            );
        };
        show_msg("To exit, press [EXE]", 90);
        show_msg("DON'T press [Home]", 110);

        eadk::timing::msleep(3000);

        // Delta time calculation stuff and loop
        let mut last = eadk::timing::millis();
        loop {
            let current = eadk::timing::millis();
            let delta = (current - last) as f32 / 1000.0;
            last = current;
            if !self.update_in_game(delta, file_name) {
                return GameState::GoMainMenu;
            }
        }
    }

    pub fn update_settings(&mut self) {
        self.renderer.update_fov(self.settings.fov);
        self.renderer.enable_vsync = self.settings.vsync;
    }

    fn exit_world(&mut self, world_name: &String) {
        for chunk in self.world.chunks.iter() {
            self.save_manager.set_chunk(chunk);
        }
        self.world.clear();

        self.save_manager.update_player_data(&self.player);

        self.save_manager.save_world_to_file(world_name.as_str());

        self.save_manager.clean();
    }

    pub fn update_in_game(&mut self, delta: f32, world_name: &String) -> bool {
        let keyboard_state = eadk::input::KeyboardState::scan();
        let just_pressed_keyboard_state = keyboard_state.get_just_pressed(self.last_keyboard_state);
        self.last_keyboard_state = keyboard_state;

        if keyboard_state.key_down(eadk::input::Key::Exe) {
            self.exit_world(world_name);

            return false;
        }

        self.player.update(
            delta,
            keyboard_state,
            just_pressed_keyboard_state,
            &mut self.world,
            &mut self.renderer.camera,
        );

        self.renderer.camera.update(delta, keyboard_state);

        //self.world.generate_world_around_pos(*self.renderer.camera.get_pos(), self.settings.render_distance as isize);
        self.world.check_mesh_regeneration();

        self.renderer
            .draw_game(&mut self.world, &self.player, 1.0 / delta);

        //eadk::timing::msleep(20);
        true
    }

    pub fn main_loop(&mut self) {
        let mut state = GameState::GoMainMenu;

        self.settings.load();
        self.update_settings();

        while !matches!(state, GameState::Quit) {
            state = match state {
                GameState::GoMainMenu => self.main_menu_loop(),
                GameState::GoSetting => self.settings_menu_loop(),
                GameState::GoSelectWorld => self.worlds_select_menu_loop(),
                GameState::LoadWorld(filename) => self.game_loop(&filename),
                GameState::DeleteWorld(filename) => self.delete_world_menu_loop(&filename),
                GameState::CreateWorld(file_name) => self.create_world_menu_loop(&file_name),
                GameState::Quit => break,
            }
        }
    }
}

pub enum GameState {
    GoMainMenu,
    GoSetting,
    GoSelectWorld,
    LoadWorld(String),   // String: filename, String: world name
    CreateWorld(String), // String: file_name
    DeleteWorld(String),
    Quit,
}
