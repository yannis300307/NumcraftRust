use core::f32::consts::PI;

use libm::sincosf;
use nalgebra::Vector3;

use crate::{constants::{world, player::MOVEMENT_SPEED}, eadk, world::World};

pub struct Player {
    pub pos: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl Player {
    pub fn new() -> Self {
        Player { pos: Vector3::new(0., 0., 0.), rotation: Vector3::new(0., 0., 0.) }
    }
    pub fn update(&mut self, delta: f32, keyboard_state: eadk::input::KeyboardState, world: &World) {
        // Movements
        if keyboard_state.key_down(eadk::input::Key::Toolbox) {
            // Forward
            let translation = sincosf(self.rotation.y);
            self.pos.x += translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z += translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Comma) {
            // Backward
            let translation = sincosf(self.rotation.y);
            self.pos.x -= translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z -= translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Imaginary) {
            // Left
            let translation = sincosf(self.rotation.y + PI / 2.0);
            self.pos.x -= translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z -= translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Power) {
            // Right
            let translation = sincosf(self.rotation.y + PI / 2.0);
            self.pos.x += translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z += translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Shift) {
            // Up
            self.pos.y -= delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Exp) {
            // Down
            self.pos.y += delta * MOVEMENT_SPEED;
        }
    }
}