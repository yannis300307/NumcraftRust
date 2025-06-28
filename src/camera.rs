use core::f32::{self, consts::PI};

use libm::{cosf, sinf};
use nalgebra::{Matrix4, UnitQuaternion, Vector3};

use crate::{
    constants::{player::ROTATION_SPEED, rendering::FOV},
    eadk,
};

pub struct Camera {
    pos: Vector3<f32>,
    rotation: Vector3<f32>,
    has_moved: bool,
    fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
            has_moved: true,
            fov: FOV,
        }
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn set_fov(&mut self, degrees: f32) {
        self.fov = degrees * PI / 180.0;
    }

    pub fn update(
        &mut self,
        delta: f32,
        keyboard_state: eadk::input::KeyboardState,
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
    }

    pub fn update_pos(&mut self, position: Vector3<f32>) {
        self.has_moved = self.pos != position;

        self.pos = position; // Updated from player
    }

    pub fn get_forward_vector(&self) -> Vector3<f32> {
        Vector3::new(
            cosf(self.rotation.x) * sinf(self.rotation.y),
            -sinf(self.rotation.x),
            cosf(self.rotation.x) * cosf(self.rotation.y),
        )
    }

    pub fn get_right_vector(&self) -> Vector3<f32> {
        Vector3::new(cosf(self.rotation.y), 0.0, -sinf(self.rotation.y))
    }

    pub fn get_up_vector(&self) -> Vector3<f32> {
        Vector3::new(
            sinf(self.rotation.x) * sinf(self.rotation.y),
            cosf(self.rotation.x),
            sinf(self.rotation.x) * cosf(self.rotation.y),
        )
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

    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;
    }

    pub fn get_has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn get_rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }
}
