use nalgebra::Vector3;

use crate::{
    eadk::{self, input::KeyboardState},
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
            if !self.update(delta) {
                break;
            }
        }
    }

    fn quit(&mut self) {
        for chunk in self.world.chunks.iter() {
            self.save_manager.set_chunk(chunk);
        }

        self.save_manager.save_world_to_file("world.ncw");
    }

    pub fn update(&mut self, delta: f32) -> bool {
        let keyboard_state = eadk::input::KeyboardState::scan();
        let just_pressed_keyboard_state = keyboard_state.get_just_pressed(self.last_keyboard_state);
        self.last_keyboard_state = keyboard_state;

        if keyboard_state.key_down(eadk::input::Key::Home) {
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
            .update(&mut self.world, &self.player, 1.0 / delta);

        //eadk::timing::msleep(20);
        true
    }
}
