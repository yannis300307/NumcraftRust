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

        self.world.add_chunk(Vector3::new(0, 0, 0)).unwrap();
        self.world.add_chunk(Vector3::new(1, 0, 0)).unwrap();
        self.world.add_chunk(Vector3::new(1, 0, 1)).unwrap();
        self.world.add_chunk(Vector3::new(0, 0, 1)).unwrap();

        self.world.generate_world();

        self.world.generate_mesh();

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
        self.renderer.update(&self.world.get_mesh(), 0.0);
        self.renderer.camera.update(delta, keyboard_state);

        //eadk::timing::msleep(20);
        true
    }
}
