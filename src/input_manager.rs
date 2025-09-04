use crate::eadk::{
    self,
    input::{Key, KeyboardState},
};

pub struct InputManager {
    keyboard_state: KeyboardState,
    last_keyboard_state: KeyboardState,
    just_pressed: KeyboardState,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            keyboard_state: KeyboardState::new(),
            last_keyboard_state: KeyboardState::new(),
            just_pressed: KeyboardState::new(),
        }
    }

    pub fn update(&mut self) {
        self.last_keyboard_state = self.keyboard_state;
        self.keyboard_state = eadk::input::KeyboardState::scan();
        self.just_pressed = self
            .keyboard_state
            .get_just_pressed(self.last_keyboard_state);
    }

    pub fn is_just_pressed(&self, key: Key) -> bool {
        self.just_pressed.key_down(key)
    }

    pub fn is_keydown(&self, key: Key) -> bool {
        self.keyboard_state.key_down(key)
    }
}
