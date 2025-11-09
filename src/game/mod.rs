#[cfg(target_os = "none")]
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    format,
    string::{String, ToString},
};
use nalgebra::{Vector2, Vector3};
use serde::{Deserialize, Serialize};

use crate::{
    constants::{
        color_palette::MENU_BACKGROUND_COLOR,
        rendering::{MAX_FOV, MAX_RENDER_DISTANCE, MIN_FOV},
    },
    eadk::{self, Color},
    game::crafting_manager::CraftingManager,
    game_ui::GameUI,
    hud::Hud,
    input_manager::InputManager,
    inventory::ItemStack,
    menu::{Menu, MenuElement, TextAnchor},
    physic::PhysicEngine,
    player::Player,
    renderer::Renderer,
    save_manager::SaveManager,
    settings::Settings,
    timing::TimingManager,
    world::World,
};

mod crafting_manager;
mod game_menus;
pub mod game_uis;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum GameMode {
    Survival,
    Creative,
}

pub struct Game {
    renderer: Renderer,
    world: World,
    player: Player,
    save_manager: SaveManager,
    settings: Settings,
    input_manager: InputManager,
    hud: Hud,
    timing_manager: TimingManager,
    physic_engine: PhysicEngine,
    crafting_manager: CraftingManager,
}

impl Game {
    pub fn new() -> Self {
        let world = World::new();
        let player = Player::new();

        Game {
            renderer: Renderer::new(),
            world,
            player,
            save_manager: SaveManager::new(),
            settings: Settings::new(),
            input_manager: InputManager::new(),
            hud: Hud::new(),
            timing_manager: TimingManager::new(),
            physic_engine: PhysicEngine::new(),
            crafting_manager: CraftingManager::new()
        }
    }

    pub fn load_world(&mut self, file_name: &String, is_new: bool) -> GameState {
        // Load the world or create it if it doesn't exists yet
        if is_new {
            self.save_manager.set_file_name(file_name);

            self.world.load_area(0, 4, 0, 4, 0, 4);

            let player_spawn_pos = Vector3::new(16.5, 20., 16.5);

            self.player.inventory.fill(ItemStack::void());

            self.world.get_player_entity_mut().pos = player_spawn_pos;

            self.hud.sync(&self.player);
        } else if let Err(error) = self.save_manager.load_from_file(file_name.as_str()) {
            Renderer::show_msg(
                &[
                    "The world seems to be corrupted.",
                    "If you've created your world",
                    "in an old version, it",
                    "is no longer compatible.",
                    format!("{:?}", error).as_str(),
                ],
                Color::from_888(255, 100, 100),
            );
            self.input_manager.wait_delay_or_ok(15000);
            return GameState::GoMainMenu;
        } else {
            // Add chunks. Maybe move this code into world (TODO)
            for x in 0..4 {
                for y in 0..4 {
                    for z in 0..4 {
                        let chunk = self
                            .save_manager
                            .get_chunk_at_pos(Vector3::new(x, y, z))
                            .unwrap();

                        self.world.chunks_manager.push_chunk(chunk);
                    }
                }
            }

            // Load player data
            let player_entity = self.world.get_player_entity_mut();
            player_entity.pos = self.save_manager.get_player_pos();
            player_entity.rotation = self.save_manager.get_player_rot();

            self.player
                .set_inventory(self.save_manager.get_player_inventory());

            // Load world info
            let world_info = self.save_manager.get_current_loaded_world_info();
            self.world.set_seed(world_info.world_seed);
        }

        self.save_manager.clean(); // Clear save manager to save memory

        if self.save_manager.get_game_mode() == GameMode::Creative {
            self.world.get_player_entity_mut().gravity = false;
        }

        // Show a warning message
        Renderer::show_msg(
            &["To exit, press [EXE]", "DON'T press [Home]"],
            Color::from_888(255, 255, 255),
        );

        self.input_manager.wait_delay_or_ok(3000);
        GameState::InGame
    }

    /// The game loop. Handle physic, rendering etc ...
    pub fn game_loop(&mut self) -> GameState {
        loop {
            self.input_manager.update();
            self.timing_manager.update();

            if self.input_manager.is_just_pressed(eadk::input::Key::Exe) {
                self.exit_world();

                return GameState::GoMainMenu;
            }
            if self.input_manager.is_just_pressed(eadk::input::Key::Var) {
                if self.save_manager.get_game_mode() == GameMode::Creative {
                    return GameState::OpenPlayerInventory(game_uis::PlayerInventoryPage::Creative);
                } else {
                    return GameState::OpenPlayerInventory(game_uis::PlayerInventoryPage::Survival);
                }
            };

            self.player.update(
                self.timing_manager.get_delta_time(),
                &self.input_manager,
                &mut self.world,
                &mut self.renderer.camera,
                &self.hud,
                self.save_manager.get_game_mode(),
                &self.physic_engine,
                self.timing_manager.get_delta_time(),
            );
            self.hud.update(&self.input_manager, &self.player);
            self.hud.sync(&self.player);

            self.renderer
                .camera
                .update(self.timing_manager.get_delta_time(), &self.input_manager);

            self.world.chunks_manager.check_mesh_regeneration();
            self.world
                .update_entities(self.timing_manager.get_delta_time());
            self.physic_engine
                .process(&mut self.world, self.timing_manager.get_delta_time());

            self.renderer.draw_game(
                &mut self.world,
                &self.player,
                self.timing_manager.get_fps(),
                &self.hud,
                true,
            );
        }
    }

    pub fn update_settings(&mut self) {
        self.renderer.update_fov(self.settings.fov);
        self.renderer.enable_vsync = self.settings.vsync;
    }

    fn exit_world(&mut self) {
        for chunk in self.world.chunks_manager.chunks.iter() {
            self.save_manager.set_chunk(chunk);
        }
        self.world.clear();

        self.save_manager
            .update_player_data(&self.world, &self.player);

        self.save_manager.save_world_to_file();

        self.save_manager.clean();
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
                GameState::LoadWorld(filename, is_new) => self.load_world(&filename, is_new),
                GameState::InGame => self.game_loop(),
                GameState::DeleteWorld(filename) => self.delete_world_menu_loop(&filename),
                GameState::CreateWorld(file_name) => self.create_world_menu_loop(&file_name),
                GameState::OpenPlayerInventory(page) => self.player_inventory_loop(page),
                GameState::Quit => break,
            }
        }
    }
}

pub enum GameState {
    GoMainMenu,
    GoSetting,
    GoSelectWorld,
    InGame,
    OpenPlayerInventory(game_uis::PlayerInventoryPage),
    LoadWorld(String, bool), // String: filename, String: world name
    CreateWorld(String),     // String: file_name
    DeleteWorld(String),
    Quit,
}
