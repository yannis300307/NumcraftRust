use core::f32::{self, consts::PI};

use nalgebra::{Matrix4, UnitQuaternion, Vector3};

use crate::{constants::player::ROTATION_SPEED, eadk};

pub struct Camera {
    pos: Vector3<f32>,
    rotation: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
        }
    }

    pub fn update(
        &mut self,
        delta: f32,
        keyboard_state: eadk::input::KeyboardState,
        position: Vector3<f32>,
    ) {
        // Rotation
        if keyboard_state.key_down(eadk::input::Key::Right) {
            self.rotation.y += delta * ROTATION_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Left) {
            self.rotation.y -= delta * ROTATION_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Up) {
            self.rotation.x += delta * ROTATION_SPEED;
            if self.rotation.x >= PI / 2.0 - 0.0001 {
                self.rotation.x = PI / 2.0 - 0.0001
            }
        }
        if keyboard_state.key_down(eadk::input::Key::Down) {
            self.rotation.x -= delta * ROTATION_SPEED;

            if self.rotation.x <= -PI / 2.0 + 0.0001 {
                self.rotation.x = -PI / 2.0 + 0.0001
            }
        }

        self.pos = position; // Updated from player
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

    pub fn get_rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }
}
