#[cfg(target_os = "none")]
use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
};
use nalgebra::{Vector2, Vector3};

use crate::{
    constants::{
        menu::MENU_BACKGROUND_COLOR,
        rendering::{MAX_FOV, MAX_RENDER_DISTANCE, MIN_FOV},
    },
    eadk::{self, Color, Point, SCREEN_RECT},
    game_ui::GameUI,
    hud::Hud,
    input_manager::InputManager,
    inventory::ItemStack,
    menu::{Menu, MenuElement, TextAnchor},
    player::Player,
    renderer::Renderer,
    save_manager::SaveManager,
    settings::Settings,
    world::World,
};

mod game_menus;
pub mod game_uis;

pub struct Game {
    renderer: Renderer,
    world: World,
    player: Player,
    save_manager: SaveManager,
    settings: Settings,
    input_manager: InputManager,
    hud: Hud,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: Renderer::new(),
            world: World::new(),
            player: Player::new(),
            save_manager: SaveManager::new(),
            settings: Settings::new(),
            input_manager: InputManager::new(),
            hud: Hud::new(),
        }
    }

    pub fn load_world(&mut self, file_name: &String) -> GameState {
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
            self.hud.sync(&self.player);
        }

        self.save_manager.clean(); // Clear save manager to save memory

        // Show a warning message
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

        //eadk::timing::msleep(3000);
        GameState::InGame
    }

    /// The game loop. Handle physic, rendering etc ...
    pub fn game_loop(&mut self) -> GameState {
        // Delta time calculation stuff and loop
        let mut last = eadk::timing::millis();
        loop {
            let current = eadk::timing::millis();
            let delta = (current - last) as f32 / 1000.0;
            last = current;

            self.input_manager.update();

            if self.input_manager.is_just_pressed(eadk::input::Key::Exe) {
                self.exit_world();

                return GameState::GoMainMenu;
            }
            if self.input_manager.is_just_pressed(eadk::input::Key::Var) {
                return GameState::OpenPlayerInventory(game_uis::PlayerInventoryPage::Normal);
            };

            self.player.update(
                delta,
                &self.input_manager,
                &mut self.world,
                &mut self.renderer.camera,
                &mut self.hud,
            );
            self.hud.update(&self.input_manager);
            self.hud.sync(&self.player);

            self.renderer.camera.update(delta, &self.input_manager);

            self.world.check_mesh_regeneration();

            self.renderer
                .draw_game(&mut self.world, &self.player, 1.0 / delta, &self.hud, true);
        }
    }

    pub fn update_settings(&mut self) {
        self.renderer.update_fov(self.settings.fov);
        self.renderer.enable_vsync = self.settings.vsync;
    }

    fn exit_world(&mut self) {
        for chunk in self.world.chunks.iter() {
            self.save_manager.set_chunk(chunk);
        }
        self.world.clear();

        self.save_manager.update_player_data(&self.player);

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
                GameState::LoadWorld(filename) => self.load_world(&filename),
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
    LoadWorld(String),   // String: filename, String: world name
    CreateWorld(String), // String: file_name
    DeleteWorld(String),
    Quit,
}
