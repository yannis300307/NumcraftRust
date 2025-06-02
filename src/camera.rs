use core::f32::{self, consts::PI};

use libm::sincosf;
use nalgebra::{Matrix4, UnitQuaternion, Vector3};

use crate::eadk;

const ROTATION_SPEED: f32 = PI / 3.0; // rad / sec
const MOVEMENT_SPEED: f32 = 2.0;

pub struct Camera {
    pos: Vector3<f32>,
    rotation: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Vector3::new(0., 0., -5.0),
            rotation: Vector3::new(0., 0., 0.),
        }
    }

    pub fn update(&mut self, delta: f32, keyboard_state: eadk::input::KeyboardState) {
        // Rotation
        if keyboard_state.key_down(eadk::input::Key::Right) {
            self.rotation.y += delta * ROTATION_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Left) {
            self.rotation.y -= delta * ROTATION_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Up) {
            self.rotation.x += delta * ROTATION_SPEED;
            if self.rotation.x >= PI / 2.0 {
                self.rotation.x = PI / 2.0
            }
        }
        if keyboard_state.key_down(eadk::input::Key::Down) {
            self.rotation.x -= delta * ROTATION_SPEED;
        }

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

    pub fn get_rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }

    pub fn get_rotation_matrix(&self) -> Matrix4<f32> {
        let yaw = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.rotation.y);
        let pitch = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.rotation.x);
        let orientation = yaw * pitch;
        orientation.to_homogeneous()
    }

    pub fn get_pos(&self) -> &Vector3<f32> {
        &self.pos
    }
}
