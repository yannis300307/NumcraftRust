use alloc::{
    format,
    string::{String, ToString},
};
use nalgebra::{Vector2, Vector3};

use crate::{
    constants::{
        menu::MENU_BACKGROUND_COLOR,
        rendering::{MAX_FOV, MAX_RENDER_DISTANCE, MIN_FOV},
    },
    eadk::{self, Color, input::KeyboardState},
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

    pub fn game_loop(&mut self) {
        let mut last = eadk::timing::millis();

        if self.save_manager.load_from_file("world.ncw").is_ok() {
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
            if !self.update_in_game(delta) {
                break;
            }
        }
    }

    pub fn settings_menu_loop(&mut self) {
        let mut menu = Menu::new(Vector2::new(10, 20), 300, 1)
            .add_element(MenuElement::Label {
                text: "Settings".to_string(),
                text_anchor: TextAnchor::Center,
                allow_margin: true,
                id: 0,
            })
            .add_element(MenuElement::Slider {
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
            .add_element(MenuElement::Slider {
                text_fn: |value| format!("{}", libm::roundf(MIN_FOV + (MAX_FOV - MIN_FOV) * value)),
                value: 0.2,
                step_size: 0.04,
                allow_margin: false,
                id: 2,
            })
            .add_element(MenuElement::Button {
                text: "Vsync: Enabled".to_string(),
                is_pressed: false,
                allow_margin: false,
                id: 3,
            })
            .add_element(MenuElement::Button {
                text: "Save".to_string(),
                is_pressed: false,
                allow_margin: false,
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

            for element in menu.get_elements_mut() { // TODO : Bad code, remake this
                match element {
                    MenuElement::Button {
                        id: 4,
                        is_pressed: true,
                        ..
                    } => {
                        eadk::timing::msleep(300);
                        return;
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
                    MenuElement::Button { // Disable all buttons
                        is_pressed, ..
                    } => {
                        *is_pressed = false;
                    }
                    _ => (),
                }
            }

            if need_redraw {
                menu.need_redraw = true;
            }

            self.renderer.draw_menu(&mut menu);
            eadk::timing::msleep(50);
        }
    }

    pub fn main_menu_loop(&mut self) {
        let mut menu = Menu::new(Vector2::new(10, 40), 300, 2)
            .add_element(MenuElement::Label {
                text: "Numcraft".to_string(),
                text_anchor: TextAnchor::Center,
                allow_margin: true,
                id: 0,
            })
            .add_element(MenuElement::Void {
                allow_margin: true,
                id: 1,
            })
            .add_element(MenuElement::Button {
                text: "Load world".to_string(),
                is_pressed: false,
                allow_margin: true,
                id: 2,
            })
            .add_element(MenuElement::Button {
                text: "Settings".to_string(),
                is_pressed: false,
                allow_margin: true,
                id: 3,
            })
            .add_element(MenuElement::Label {
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
                return;
            }

            for element in menu.get_elements() {
                if matches!(
                    element,
                    MenuElement::Button {
                        id: 3,
                        is_pressed: true,
                        ..
                    }
                ) {
                    self.settings_menu_loop();
                }
            }

            self.renderer.draw_menu(&mut menu);
            eadk::timing::msleep(50);
        }
    }

    fn quit(&mut self) {
        for chunk in self.world.chunks.iter() {
            self.save_manager.set_chunk(chunk);
        }

        self.save_manager.save_world_to_file("world.ncw");
    }

    pub fn update_in_game(&mut self, delta: f32) -> bool {
        let keyboard_state = eadk::input::KeyboardState::scan();
        let just_pressed_keyboard_state = keyboard_state.get_just_pressed(self.last_keyboard_state);
        self.last_keyboard_state = keyboard_state;

        if keyboard_state.key_down(eadk::input::Key::Exe) {
            self.quit();

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
}
