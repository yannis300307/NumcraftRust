use enum_iterator::all;

use crate::{eadk::{
    self,
    input::{Key, KeyboardState},
}, timing::TimingManager};

pub struct InputManager {
    keyboard_state: KeyboardState,
    last_keyboard_state: KeyboardState,
    just_pressed: KeyboardState,
    last_pressed: Option<Key>,
    last_pressed_timer: f32,
    impulsed_key_buffer: Option<Key>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            keyboard_state: KeyboardState::new(),
            last_keyboard_state: KeyboardState::new(),
            just_pressed: KeyboardState::new(),

            last_pressed: None,
            last_pressed_timer: 0.,

            impulsed_key_buffer: None
        }
    }

    pub fn update(&mut self) {
        self.last_keyboard_state = self.keyboard_state;
        self.keyboard_state = eadk::input::KeyboardState::scan();
        self.just_pressed = self
            .keyboard_state
            .get_just_pressed(self.last_keyboard_state);
    }

    pub fn update_timing(&mut self, timing_manager: &TimingManager) {
        self.impulsed_key_buffer = None;

        let last_pressed = self.get_last_pressed();
        if last_pressed.is_some() {
            self.last_pressed = last_pressed;
            self.last_pressed_timer = 0.5;
            self.impulsed_key_buffer = self.last_pressed;
        }

        if let Some(key) = self.last_pressed
            && !self.is_keydown(key)
        {
            self.last_pressed = None;
        }

        if self.last_pressed.is_some() {
            self.last_pressed_timer -= timing_manager.get_delta_time();

            if self.last_pressed_timer < 0. {
                self.last_pressed_timer = 0.06;
                self.impulsed_key_buffer = self.last_pressed;
            }
        }
    }

    pub fn get_last_pressed(&self) -> Option<Key> {
        for k in all::<Key>() {
            if self.is_just_pressed(k) {
                return Some(k);
            }
        }
        None
    }

    pub fn is_impulsed_key(&self, key: Key) -> bool {
        return self.impulsed_key_buffer.is_some_and(|v| v == key)
    }

    pub fn is_just_pressed(&self, key: Key) -> bool {
        self.just_pressed.key_down(key)
    }

    pub fn is_keydown(&self, key: Key) -> bool {
        self.keyboard_state.key_down(key)
    }

    pub fn wait_delay_or_ok(&mut self, delay_ms: usize) {
        while self.is_keydown(eadk::input::Key::Ok) {
            self.update();
            eadk::timing::usleep(100);
        }
        for _ in 0..delay_ms / 50 {
            self.update();
            if self.is_just_pressed(eadk::input::Key::Ok) {
                break;
            }
            eadk::timing::msleep(50);
        }
    }
}
