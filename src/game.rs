use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
};
use nalgebra::{Rotation, Vector2, Vector3};
use postcard::from_bytes;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{
        menu::{MENU_BACKGROUND_COLOR, SETTINGS_FILENAME},
        rendering::{FOV, MAX_FOV, MAX_RENDER_DISTANCE, MIN_FOV},
    },
    eadk::{self, Color, Point, SCREEN_RECT, input::KeyboardState},
    menu::{Menu, MenuElement, TextAnchor},
    player::Player,
    renderer::Renderer,
    storage_lib::{
        storage_extapp_file_erase, storage_extapp_file_exists, storage_extapp_file_read,
        storage_file_write,
    },
    storage_manager::SaveManager,
    world::World,
};

pub struct Game {
    renderer: Renderer,
    world: World,
    player: Player,
    last_keyboard_state: KeyboardState,
    save_manager: SaveManager,
    settings: Settings,
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
        }
    }

    /// The game loop. Handle physic, rendering etc ...
    pub fn game_loop(&mut self, world_name: &String) -> GameState {
        // Load the world or create it if it doesn't exists yet
        if self
            .save_manager
            .load_from_file(world_name.as_str())
            .is_ok()
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

            self.player.set_pos_rotation(
                &mut self.renderer.camera,
                self.save_manager.get_player_rot(),
                self.save_manager.get_player_pos(),
            );
        } else {
            self.world.load_area(0, 4, 0, 4, 0, 4);
            self.player.set_pos_rotation(&mut self.renderer.camera, Vector3::new(0., 0., 0.), Vector3::new(16., 16., 16.)); // Hard codded map center. TODO : calculate height if random seed generation is implemented (it will)
        }

        self.save_manager.clean(); // Clear save manager to save memory

        // Show a warning messge
        eadk::display::push_rect_uniform(SCREEN_RECT, Color::from_888(255, 255, 255));
        let show_msg = |message, y| {
            eadk::display::draw_string(
                message,
                Point {
                    x: ((320 - message.len() * 10) / 2) as u16,
                    y: y,
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
            if !self.update_in_game(delta, world_name) {
                return GameState::GoMainMenu;
            }
        }
    }

    /// The menu the user can go to select the world to load
    pub fn worlds_select_menu_loop(&mut self) -> GameState {
        // Create a new menu with a title
        let mut menu = Menu::new(Vector2::new(10, 20), 300, 1).with_element(MenuElement::Label {
            text: "Select a world".to_string(),
            text_anchor: TextAnchor::Center,
            allow_margin: true,
            id: 0,
        });

        // Get the list of all the existing worlds. World name must be "world{i}.ncw" (NCW = "NumCraft World" btw)
        let worlds = self.save_manager.get_existing_worlds();

        // Max 4 worlds because it's enough for the storage memory amount we have and because. I can't fit more than 4 buttons on the screen ;-)
        for i in 0..4 {
            let world_name = format!("world{i}.ncw");
            let button_text = if worlds.contains(&world_name) {
                format!("Load {}", world_name)
            } else {
                format!("Create world{i}.ncw")
            };
            menu.add_element(MenuElement::Button {
                text: button_text,
                allow_margin: true,
                id: 1 + i,
                is_pressed: false,
            });
        }

        // Clear the screen
        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        loop {
            // Get keyboard state and calculate the new presses
            let keyboard_state = eadk::input::KeyboardState::scan();
            let just_pressed_keyboard_state =
                keyboard_state.get_just_pressed(self.last_keyboard_state);
            self.last_keyboard_state = keyboard_state;

            // Handle the navigation in the menu
            menu.check_inputs(keyboard_state, just_pressed_keyboard_state);

            // Exit the menu when [Back] is pressed
            if keyboard_state.key_down(eadk::input::Key::Back) {
                return GameState::GoMainMenu;
            }

            // Handle buttons
            for element in menu.get_elements_mut() {
                match element {
                    MenuElement::Button {
                        id,
                        is_pressed: true,
                        ..
                    } => {
                        let world_slot = *id - 1; // Please change this if you change the button's id. Not 100% safe but who cares about safety?
                        let world_name = format!("world{world_slot}.ncw");

                        // Load the world (and create a new world if it doesn't exists yet)
                        return GameState::LoadWorld(world_name.to_owned());
                    }
                    _ => (),
                }
            }

            // Set all "is_pressed" to false
            menu.finish_buttons_handling();

            self.renderer.draw_menu(&mut menu);
            eadk::timing::msleep(50);
        }
    }

    pub fn settings_menu_loop(&mut self) -> GameState {
        // Temporary variables used to store settings
        let mut vsync_enabled = self.settings.vsync;
        let mut fov: f32 = self.settings.fov;
        let mut render_distance: usize = self.settings.render_distance;

        // Create the menu.
        let mut menu = Menu::new(Vector2::new(10, 20), 300, 1)
            .with_element(MenuElement::Label {
                text: "Settings".to_string(),
                text_anchor: TextAnchor::Center,
                allow_margin: true,
                id: 0,
            })
            .with_element(MenuElement::Slider {
                text_fn: |value| {
                    format!(
                        "Render Distance: {}",
                        libm::roundf(value * MAX_RENDER_DISTANCE as f32) as usize
                    )
                },
                value: render_distance as f32 / MAX_RENDER_DISTANCE as f32,
                step_size: 0.5,
                allow_margin: false,
                id: 1,
            })
            .with_element(MenuElement::Slider {
                text_fn: |value| {
                    format!(
                        "Field of view: {}°",
                        libm::roundf(MIN_FOV + (MAX_FOV - MIN_FOV) * value)
                    )
                },
                value: (fov - MIN_FOV) / (MAX_FOV - MIN_FOV), // The opposite of the above calculation
                step_size: 0.04,
                allow_margin: false,
                id: 2,
            })
            .with_element(MenuElement::Button {
                text: if vsync_enabled {
                    "Vsync: Enabled".to_string()
                } else {
                    "Vsync: Disabled".to_string()
                },
                is_pressed: false,
                allow_margin: true,
                id: 3,
            })
            .with_element(MenuElement::Button {
                text: "Save".to_string(),
                is_pressed: false,
                allow_margin: true,
                id: 4,
            });

        // Clear the screen
        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        loop {
            let keyboard_state = eadk::input::KeyboardState::scan();
            let just_pressed_keyboard_state =
                keyboard_state.get_just_pressed(self.last_keyboard_state);
            self.last_keyboard_state = keyboard_state;

            if keyboard_state.key_down(eadk::input::Key::Back) {
                return GameState::GoMainMenu;
            }

            menu.check_inputs(keyboard_state, just_pressed_keyboard_state);

            let mut need_redraw = false;

            for element in menu.get_elements_mut() {
                match element {
                    MenuElement::Button {
                        // Save
                        id: 4,
                        is_pressed: true,
                        ..
                    } => {
                        // Save settings
                        self.settings.fov = fov;
                        self.settings.render_distance = render_distance;
                        self.settings.vsync = vsync_enabled;
                        self.update_settings();
                        self.settings.save();

                        return GameState::GoMainMenu;
                    }
                    MenuElement::Button {
                        // Enable / Disable Vsync
                        text,
                        is_pressed: true,
                        id: 3,
                        ..
                    } => {
                        vsync_enabled = !vsync_enabled;
                        *text = if vsync_enabled {
                            "Vsync: Enabled".to_string()
                        } else {
                            "Vsync: Disabled".to_string()
                        };
                        need_redraw = true;
                    }
                    MenuElement::Slider { value, id: 2, .. } => {
                        fov = libm::roundf(MIN_FOV + (MAX_FOV - MIN_FOV) * *value)
                    }
                    MenuElement::Slider { value, id: 1, .. } => {
                        render_distance = libm::roundf(*value * MAX_RENDER_DISTANCE as f32) as usize
                    }
                    _ => (),
                }
            }

            menu.finish_buttons_handling();

            if need_redraw {
                menu.need_redraw = true;
            }

            self.renderer.draw_menu(&mut menu);
            eadk::timing::msleep(50);
        }
    }

    pub fn update_settings(&mut self) {
        self.renderer.update_fov(self.settings.fov);
        self.renderer.enable_vsync = self.settings.vsync;
    }

    pub fn main_menu_loop(&mut self) -> GameState {
        let mut menu = Menu::new(Vector2::new(10, 40), 300, 2)
            .with_element(MenuElement::Label {
                text: "Numcraft".to_string(),
                text_anchor: TextAnchor::Center,
                allow_margin: true,
                id: 0,
            })
            .with_element(MenuElement::Void {
                allow_margin: true,
                id: 1,
            })
            .with_element(MenuElement::Button {
                text: "Load world".to_string(),
                is_pressed: false,
                allow_margin: true,
                id: 2,
            })
            .with_element(MenuElement::Button {
                text: "Settings".to_string(),
                is_pressed: false,
                allow_margin: true,
                id: 3,
            })
            .with_element(MenuElement::Label {
                text: "Press [Home] to quit".to_string(),
                text_anchor: TextAnchor::Center,
                allow_margin: true,
                id: 4,
            });

        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        loop {
            let keyboard_state = eadk::input::KeyboardState::scan();
            let just_pressed_keyboard_state =
                keyboard_state.get_just_pressed(self.last_keyboard_state);
            self.last_keyboard_state = keyboard_state;

            menu.check_inputs(keyboard_state, just_pressed_keyboard_state);

            if just_pressed_keyboard_state.key_down(eadk::input::Key::Home) {
                return GameState::Quit;
            }

            for element in menu.get_elements_mut() {
                match element {
                    MenuElement::Button {
                        id: 3,
                        is_pressed: true,
                        ..
                    } => {
                        return GameState::GoSetting;
                    }
                    MenuElement::Button {
                        id: 2,
                        is_pressed: true,
                        ..
                    } => return GameState::GoSelectWorld,
                    _ => (),
                }
            }

            menu.finish_buttons_handling();

            self.renderer.draw_menu(&mut menu);
            eadk::timing::msleep(50);
        }
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
                GameState::LoadWorld(world_name) => self.game_loop(&world_name),
                GameState::Quit => break,
            }
        }
    }
}

pub enum GameState {
    GoMainMenu,
    GoSetting,
    GoSelectWorld,
    LoadWorld(String), // String: World name
    Quit,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    render_distance: usize,
    fov: f32,
    vsync: bool,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            render_distance: MAX_RENDER_DISTANCE,
            fov: FOV,
            vsync: true,
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
