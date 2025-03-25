use crate::{eadk, renderer::Renderer};

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: Renderer::new(),
        }
    }

    pub fn start(&mut self) {
        let last = eadk::timing::millis();
        loop {
            let current = eadk::timing::millis();
            let delta = (current - last) as f32 / 1000.0;
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
        self.renderer.update();
        self.renderer.camera.update(delta, keyboard_state);

        eadk::timing::msleep(100);
        true
    }
}
