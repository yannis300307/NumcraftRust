use nalgebra::Vector3;

use crate::{
    constants::rendering::RENDER_DISTANCE,
    eadk::{self, input::KeyboardState},
    player::Player,
    renderer::Renderer,
    world::World,
};

pub struct Game {
    renderer: Renderer,
    world: World,
    player: Player,
    last_keyboard_state: KeyboardState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: Renderer::new(),
            world: World::new(),
            player: Player::new(),
            last_keyboard_state: KeyboardState::new(),
        }
    }

    pub fn start(&mut self) {
        let mut last = eadk::timing::millis();

        self.world.load_area(0, 4, 0, 4, 0, 4);

        loop {
            let current = eadk::timing::millis();
            let delta = (current - last) as f32 / 1000.0;
            last = current;
            if !self.update(delta) {
                break;
            }
        }
    }

    pub fn update(&mut self, delta: f32) -> bool {
        let keyboard_state = eadk::input::KeyboardState::scan();
        let just_pressed_keyboard_state = keyboard_state.get_just_pressed(self.last_keyboard_state);
        self.last_keyboard_state = keyboard_state;
        if keyboard_state.key_down(eadk::input::Key::Home) {
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
