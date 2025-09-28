use crate::game::*;

impl Game {
    pub fn create_world_menu_loop(&mut self, file_name: &String) -> GameState {
        let mut menu = Menu::new(Vector2::new(10, 20), 300, 1)
            .with_element(MenuElement::Label {
                text: "Create new world".to_string(),
                text_anchor: TextAnchor::Center,
                allow_margin: true,
            })
            .with_element(MenuElement::Entry {
                placeholder_text: "World name".to_string(),
                value: String::new(),
                allow_margin: false,
                max_len: 15,
                digits_only: false,
                id: 0,
            })
            .with_element(MenuElement::Entry {
                placeholder_text: "World seed".to_string(),
                value: format!("{}", eadk::random() % 1_000_000_000),
                allow_margin: true,
                max_len: 9,
                digits_only: true,
                id: 1,
            })
            .with_element(MenuElement::Button {
                text: "Game mode : Survival".to_string(),
                is_pressed: false,
                allow_margin: true,
                id: 2,
            })
            .with_element(MenuElement::Button {
                text: "Create world".to_string(),
                is_pressed: false,
                allow_margin: false,
                id: 3,
            });

        // Clear the screen
        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        self.timing_manager.reset();

        let mut game_mode = GameMode::Survival;

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);

            // Exit the menu when [Back] is pressed
            if self.input_manager.is_keydown(eadk::input::Key::Back) {
                return GameState::GoSelectWorld;
            }

            // Handle the navigation in the menu
            menu.check_inputs(&self.input_manager);
            for element in menu.get_elements_mut() {
                match element {
                    MenuElement::Button {
                        is_pressed: true,
                        id: 3,
                        ..
                    } => {
                        let mut world_name = String::new();
                        let mut seed = "1".to_string();
                        for other_element in menu.get_elements() {
                            if let MenuElement::Entry { value, id: 0, .. } = &other_element {
                                world_name = value.clone();
                            }
                            if let MenuElement::Entry { value, id: 1, .. } = &other_element {
                                seed = value.clone();
                            }
                        }

                        if world_name.is_empty() {
                            world_name = "Unnamed".to_string();
                        }

                        if seed.is_empty() {
                            seed = format!("{}", eadk::random() % 1_000_000_000);
                        }

                        let world_seed = seed.parse::<i32>().unwrap_or(1);
                        self.world.set_seed(world_seed);

                        self.save_manager.set_world_seed(world_seed);
                        self.save_manager.set_world_name(&world_name);
                        self.save_manager.set_gamemode(game_mode);

                        return GameState::LoadWorld(file_name.clone(), true);
                    }
                    MenuElement::Button {
                        is_pressed: true,
                        id: 2,
                        text,
                        ..
                    } => {
                        match game_mode {
                            GameMode::Survival => {
                                *text = "Game mode : Creative".to_string();
                                game_mode = GameMode::Creative;
                            }
                            GameMode::Creative => {
                                *text = "Game mode : Survival".to_string();
                                game_mode = GameMode::Survival;
                            }
                        };
                    }
                    _ => (),
                }
            }
            menu.finish_buttons_handling();

            self.renderer.draw_menu(&mut menu);
            eadk::timing::msleep(50);
        }
    }

    pub fn delete_world_menu_loop(&mut self, world_name: &String) -> GameState {
        let mut menu = Menu::new(Vector2::new(10, 70), 300, 2)
            .with_element(MenuElement::Label {
                text: format!("Delete {}?", world_name),
                text_anchor: TextAnchor::Center,
                allow_margin: false,
            })
            .with_element(MenuElement::Label {
                text: "This cannot be undone.".to_string(),
                text_anchor: TextAnchor::Center,
                allow_margin: true,
            })
            .with_element(MenuElement::Button {
                text: format!("Yes, delete {}", world_name),
                is_pressed: false,
                allow_margin: true,
                id: 0,
            })
            .with_element(MenuElement::Button {
                text: "No, go back".to_string(),
                is_pressed: false,
                allow_margin: false,
                id: 1,
            });

        // Clear the screen
        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        self.timing_manager.reset();

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);

            // Exit the menu when [Back] is pressed
            if self.input_manager.is_keydown(eadk::input::Key::Back) {
                return GameState::GoSelectWorld;
            }

            // Handle the navigation in the menu
            menu.check_inputs(&self.input_manager);
            for element in menu.get_elements() {
                match element {
                    MenuElement::Button {
                        // Confirm delete
                        id: 0,
                        is_pressed: true,
                        ..
                    } => {
                        self.save_manager.delete_world(world_name);
                        return GameState::GoSelectWorld;
                    }
                    MenuElement::Button {
                        // Cancel delete
                        id: 1,
                        is_pressed: true,
                        ..
                    } => {
                        return GameState::GoSelectWorld;
                    }
                    _ => (),
                }
            }
            menu.finish_buttons_handling();

            self.renderer.draw_menu(&mut menu);
            eadk::timing::msleep(50);
        }
    }

    /// The menu the user can go to select the world to load
    pub fn worlds_select_menu_loop(&mut self) -> GameState {
        // Create a new menu with a title
        let mut menu = Menu::new(Vector2::new(10, 20), 300, 1).with_element(MenuElement::Label {
            text: "Select a world".to_string(),
            text_anchor: TextAnchor::Center,
            allow_margin: true,
        });

        // Get the list of all the existing worlds. World name must be "world{i}.ncw" (NCW = "NumCraft World" btw)
        let worlds = self.save_manager.get_existing_worlds();

        // Max 4 worlds because it's enough for the storage memory amount we have and because. I can't fit more than 4 buttons on the screen ;-)
        for i in 0..4 {
            let filename = format!("world{i}.ncw");

            let world_name = if let Some(world_info) = self.save_manager.get_world_info(&filename) {
                world_info.world_name
            } else {
                filename.clone()
            };
            let world_exists = worlds.contains(&filename);
            let button_text = if world_exists {
                format!("Load {}", world_name)
            } else {
                format!("Create world{i}.ncw")
            };
            menu.add_element(MenuElement::Button {
                text: button_text,
                allow_margin: true,
                id: i,
                is_pressed: false,
            });
            if world_exists {
                menu.add_element(MenuElement::ButtonOption {
                    text: "Delete".to_string(),
                    is_pressed: false,
                    id: i,
                });
            }
        }

        // Clear the screen
        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        self.timing_manager.reset();

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);

            // Handle the navigation in the menu
            menu.check_inputs(&self.input_manager);

            // Exit the menu when [Back] is pressed
            if self.input_manager.is_keydown(eadk::input::Key::Back) {
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
                        let world_slot = *id; // Please change this if you change the button's id. Not 100% safe but who cares about safety?
                        let filename = format!("world{world_slot}.ncw");

                        if worlds.contains(&filename) {
                            // Load the world
                            return GameState::LoadWorld(filename.to_owned(), false);
                        } else {
                            return GameState::CreateWorld(filename.to_owned());
                        }
                    }
                    MenuElement::ButtonOption {
                        is_pressed: true,
                        id,
                        ..
                    } => {
                        let world_slot = *id; // Please change this if you change the button's id.
                        let filename = format!("world{world_slot}.ncw");

                        // Delete the world
                        return GameState::DeleteWorld(filename.to_owned());
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
            })
            .with_element(MenuElement::Label {
                text: format!("Numcraft v{} by Yannis", env!("CARGO_PKG_VERSION")),
                text_anchor: TextAnchor::Left,
                allow_margin: false,
            });

        // Clear the screen
        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        self.timing_manager.reset();

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);

            if self.input_manager.is_keydown(eadk::input::Key::Back) {
                return GameState::GoMainMenu;
            }

            menu.check_inputs(&self.input_manager);

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

    pub fn main_menu_loop(&mut self) -> GameState {
        let mut menu = Menu::new(Vector2::new(10, 40), 300, 2)
            .with_element(MenuElement::Label {
                text: "Numcraft".to_string(),
                text_anchor: TextAnchor::Center,
                allow_margin: true,
            })
            .with_element(MenuElement::Void { allow_margin: true })
            .with_element(MenuElement::Button {
                text: "Load world".to_string(),
                is_pressed: false,
                allow_margin: true,
                id: 0,
            })
            .with_element(MenuElement::Button {
                text: "Settings".to_string(),
                is_pressed: false,
                allow_margin: true,
                id: 1,
            })
            .with_element(MenuElement::Label {
                text: "Press [Home] to quit".to_string(),
                text_anchor: TextAnchor::Center,
                allow_margin: true,
            });

        eadk::display::push_rect_uniform(eadk::SCREEN_RECT, MENU_BACKGROUND_COLOR);

        self.timing_manager.reset();

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);

            menu.check_inputs(&self.input_manager);

            if self.input_manager.is_keydown(eadk::input::Key::Home) {
                return GameState::Quit;
            }

            for element in menu.get_elements_mut() {
                match element {
                    MenuElement::Button {
                        id: 1,
                        is_pressed: true,
                        ..
                    } => {
                        return GameState::GoSetting;
                    }
                    MenuElement::Button {
                        id: 0,
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
}
