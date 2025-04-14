use core::f32;

use libm::sincosf;
use nalgebra::Vector3;

use crate::eadk;

const PI: f32 = f32::consts::PI;

const ROTATION_SPEED: f32 = PI/4.0; // rad / sec
const MOVEMENT_SPEED: f32 = 1.0;

pub struct Camera {
    pos: Vector3<f32>,
    rotation: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Vector3::new(0., 0., -5.0),
            rotation: Vector3::new(0., 0.0, 0.),
        }
    }

    pub fn rotate(&mut self, rotation_vector: Vector3<f32>) {
        self.rotation += rotation_vector;
    }
    pub fn translate(&mut self, translation_vector: Vector3<f32>) {
        self.pos += translation_vector;
    }

    pub fn update(&mut self, delta: f32, keyboard_state: eadk::input::KeyboardState) {
        // Rotation
        if keyboard_state.key_down(eadk::input::Key::Right) {
            self.rotation.y -= delta * ROTATION_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Left) {
            self.rotation.y += delta * ROTATION_SPEED;
        }

        // Movements
        if keyboard_state.key_down(eadk::input::Key::Toolbox) { // Forward
            let translation = sincosf(self.rotation.y);
            self.pos.x += translation.0*delta*MOVEMENT_SPEED;
            self.pos.z += translation.1*delta*MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Comma) { // Backward
            let translation = sincosf(self.rotation.y);
            self.pos.x -= translation.0*delta*MOVEMENT_SPEED;
            self.pos.z -= translation.1*delta*MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Imaginary) { // Left
            let translation = sincosf(self.rotation.y+PI/2.0);
            self.pos.x += translation.0*delta*MOVEMENT_SPEED;
            self.pos.z += translation.1*delta*MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Power) { // Right
            let translation = sincosf(self.rotation.y+PI/2.0);
            self.pos.x -= translation.0*delta*MOVEMENT_SPEED;
            self.pos.z -= translation.1*delta*MOVEMENT_SPEED;
        }
    }

    pub fn get_rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }

    pub fn get_pos(&self) -> &Vector3<f32> {
        &self.pos
    }
}
