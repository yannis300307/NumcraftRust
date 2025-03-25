use core::f32;

use nalgebra::Vector3;

use crate::eadk;

const PI: f32 = f32::consts::PI;

const ROTATION_SPEED: f32 = PI; // rad / sec

pub struct Camera {
    pos: Vector3<f32>,
    rotation: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Vector3::new(0., 0., -3.0),
            rotation: Vector3::new(0., 0.0, 0.),
        }
    }

    pub fn rotate(&mut self, rotation_vector: Vector3<f32>) {
        self.rotation += rotation_vector;
    }

    pub fn update(&mut self, delta: f32, keyboard_state: eadk::input::KeyboardState) {
        if keyboard_state.key_down(eadk::input::Key::Right) {
            self.rotation.y -= delta * ROTATION_SPEED;
        }

        if keyboard_state.key_down(eadk::input::Key::Left) {
            self.rotation.y += delta * ROTATION_SPEED;
        }
    }

    pub fn get_rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }

    pub fn get_pos(&self) -> &Vector3<f32> {
        &self.pos
    }
}
