use libm::{cosf, sinf};
use nalgebra::{Matrix3, Vector3};

use crate::eadk;

const PI: f32 = 3.14159265358979323846264338327950288419716939937510582;

const ROTATION_SPEED: f32 = PI; // rad / sec

pub struct Camera {
    pos: Vector3<f32>,
    rotation: Vector3<f32>,
    fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
            fov: 0.,
        }
    }

    pub fn update(&mut self, delta: f32, keyboard_state: eadk::input::KeyboardState) {
        if keyboard_state.key_down(eadk::input::Key::Right) {
            self.rotation.y -= delta * ROTATION_SPEED;
        }

        if keyboard_state.key_down(eadk::input::Key::Left) {
            self.rotation.y += delta * ROTATION_SPEED;
        }
    }

    pub fn get_x_rotation_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
            1.0,
            0.0,
            0.0,
            0.0,
            cosf(self.rotation.x),
            sinf(self.rotation.x),
            0.0,
            -sinf(self.rotation.x),
            cosf(self.rotation.x),
        )
    }

    pub fn get_y_rotation_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
            cosf(self.rotation.y),
            0.0,
            -sinf(self.rotation.y),
            0.0,
            1.0,
            0.0,
            sinf(self.rotation.y),
            0.0,
            cosf(self.rotation.y),
        )
    }

    pub fn get_z_rotation_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
            cosf(self.rotation.z),
            sinf(self.rotation.z),
            0.0,
            -sinf(self.rotation.z),
            cosf(self.rotation.z),
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn get_pos(&self) -> &Vector3<f32> {
        &self.pos
    }

    pub fn get_fov(&self) -> &f32 {
        &self.fov
    }
}
