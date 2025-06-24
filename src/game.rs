use alloc::string::{String, ToString};
use nalgebra::{Vector2, Vector3};

use crate::{
    eadk::{self, input::KeyboardState},
    menu::Menu,
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

    pub fn main_menu_loop(&mut self) {
        let mut menu = Menu::new(Vector2::new(10, 10), 300)
            .add_element(crate::menu::MenuElement::Button {
                text: "Boutton".to_string(),
                is_pressed: false,
            })
            .add_element(crate::menu::MenuElement::Slider {
                text: "Slider".to_string(),
                value: 0.0,
                step_size: 0.1,
            })
            .add_element(crate::menu::MenuElement::Label {
                text: "Label Center".to_string(),
                text_anchor: crate::menu::TextAnchor::Center,
            })
            .add_element(crate::menu::MenuElement::Label {
                text: "Label Left".to_string(),
                text_anchor: crate::menu::TextAnchor::Left,
            })
            .add_element(crate::menu::MenuElement::Label {
                text: "Label Right".to_string(),
                text_anchor: crate::menu::TextAnchor::Right,
            });

        loop {
            let keyboard_state = eadk::input::KeyboardState::scan();
            let just_pressed_keyboard_state =
                keyboard_state.get_just_pressed(self.last_keyboard_state);
            self.last_keyboard_state = keyboard_state;

            if keyboard_state.key_down(eadk::input::Key::Exe) {
                return;
            }
            if just_pressed_keyboard_state.key_down(eadk::input::Key::Down) {
                menu.cursor_down();
            }
            if just_pressed_keyboard_state.key_down(eadk::input::Key::Up) {
                menu.cursor_up();
            }
            self.renderer.draw_menu(&menu);
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
