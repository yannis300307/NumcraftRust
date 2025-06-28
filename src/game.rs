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
    eadk::{self, Color, Point, input::KeyboardState},
    menu::{Menu, MenuElement, TextAnchor},
    player::Player,
    renderer::Renderer,
    storage_manager::SaveManager,
    world::World,
};

pub struct Game {
    renderer: Renderer,
    world: World,
    player: Player,
    last_keyboard_state: KeyboardState,
    save_manager: SaveManager,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: Renderer::new(),
            world: World::new(),
            player: Player::new(),
            last_keyboard_state: KeyboardState::new(),
            save_manager: SaveManager::new(),
        }
    }

    pub fn game_loop(&mut self, world_name: &String) -> GameState {
        let mut last = eadk::timing::millis();

        if self.save_manager.load_from_file(world_name.as_str()).is_ok() {
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
        } else {
            self.world.load_area(0, 4, 0, 4, 0, 4);
        }

        self.save_manager.clean();

        loop {
            let current = eadk::timing::millis();
            let delta = (current - last) as f32 / 1000.0;
            last = current;
            if !self.update_in_game(delta, world_name) {
                return GameState::GoMainMenu;
            }
        }
    }

    pub fn worlds_select_menu_loop(&mut self) -> GameState {
        let mut menu = Menu::new(Vector2::new(10, 20), 300, 1).with_element(MenuElement::Label {
            text: "Select a world".to_string(),
            text_anchor: TextAnchor::Center,
            allow_margin: true,
            id: 0,
        });

        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        let worlds = self.save_manager.get_existing_worlds();

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

        loop {
            let keyboard_state = eadk::input::KeyboardState::scan();
            let just_pressed_keyboard_state =
                keyboard_state.get_just_pressed(self.last_keyboard_state);
            self.last_keyboard_state = keyboard_state;

            menu.check_inputs(keyboard_state, just_pressed_keyboard_state);

            if keyboard_state.key_down(eadk::input::Key::Back) {
                return GameState::GoMainMenu;
            }

            for element in menu.get_elements_mut() {
                match element {
                    MenuElement::Button {
                        id,
                        is_pressed: true,
                        ..
                    } => {
                        let world_slot = *id - 1; // So please change this if you change your button's id. Not 100% safe but...

                        if world_slot < worlds.len() {
                            let world_filename = &worlds[world_slot];

                            return GameState::LoadWorld(world_filename.to_owned());
                        } else {
                            return GameState::LoadWorld(format!("world{}.ncw", world_slot));
                        }
                    }

                    _ => (),
                }
            }

            menu.finish_buttons_handling();

            self.renderer.draw_menu(&mut menu);
            eadk::timing::msleep(50);
        }
    }

    pub fn settings_menu_loop(&mut self) -> GameState {
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
                value: 1.,
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
                value: 0.2,
                step_size: 0.04,
                allow_margin: false,
                id: 2,
            })
            .with_element(MenuElement::Button {
                text: "Vsync: Enabled".to_string(),
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

        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        let mut vsync_enabled = true;

        loop {
            let keyboard_state = eadk::input::KeyboardState::scan();
            let just_pressed_keyboard_state =
                keyboard_state.get_just_pressed(self.last_keyboard_state);
            self.last_keyboard_state = keyboard_state;

            menu.check_inputs(keyboard_state, just_pressed_keyboard_state);

            let mut need_redraw = false;

            for element in menu.get_elements_mut() {
                match element {
                    MenuElement::Button {
                        id: 4,
                        is_pressed: true,
                        ..
                    } => {
                        return GameState::GoMainMenu;
                    }
                    MenuElement::Button {
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

    fn quit(&mut self, world_name: &String) {
        for chunk in self.world.chunks.iter() {
            self.save_manager.set_chunk(chunk);
        }

        self.world.clear();

        self.save_manager.save_world_to_file(world_name.as_str());

        self.save_manager.clean();
    }

    pub fn update_in_game(&mut self, delta: f32, world_name: &String) -> bool {
        let keyboard_state = eadk::input::KeyboardState::scan();
        let just_pressed_keyboard_state = keyboard_state.get_just_pressed(self.last_keyboard_state);
        self.last_keyboard_state = keyboard_state;

        if keyboard_state.key_down(eadk::input::Key::Exe) {
            self.quit(world_name);

            return false;
        }

        self.player.update(
            delta,
            keyboard_state,
            just_pressed_keyboard_state,
            &mut self.world,
            &mut self.renderer.camera,
        );

        //self.world.generate_world_around_pos(*self.renderer.camera.get_pos(), RENDER_DISTANCE as isize);
        self.world.check_mesh_regeneration();

        self.renderer
            .draw_game(&mut self.world, &self.player, 1.0 / delta);

        //eadk::timing::msleep(20);
        true
    }

    pub fn main_loop(&mut self) {
        let mut state = GameState::GoMainMenu;
        while !matches!(state, GameState::Quit) {
            state = match state {
                GameState::GoMainMenu => self.main_menu_loop(),
                GameState::GoSetting => self.settings_menu_loop(),
                GameState::GoSelectWorld => self.worlds_select_menu_loop(),
                GameState::LoadWorld(world_name) => self.game_loop(&world_name),
                GameState::Quit => return,
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
