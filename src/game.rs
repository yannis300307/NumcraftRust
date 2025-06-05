use nalgebra::Vector3;

use crate::{eadk, renderer::Renderer, world::World};

pub struct Game {
    renderer: Renderer,
    world: World,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: Renderer::new(),
            world: World::new(),
        }
    }

    pub fn start(&mut self) {
        let mut last = eadk::timing::millis();

        for x in 0..2 {
            for z in 0..2 {
                self.world.add_chunk(Vector3::new(x, 0, z)).unwrap();
            }
        }

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
        if keyboard_state.key_down(eadk::input::Key::Home) {
            return false;
        }

        self.world
            .generate_world_around_pos(*self.renderer.camera.get_pos(), 1);

        self.renderer.update(&self.world.get_mesh(), 1.0 / delta);
        self.renderer.camera.update(delta, keyboard_state);

        //eadk::timing::msleep(20);
        true
    }
}
