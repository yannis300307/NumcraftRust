use libm::{cosf, sinf};
use nalgebra::{ Matrix3, Vector3};

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

    pub fn get_x_rotation_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
            1.0, 0.0,                   0.0,
            0.0, cosf(self.rotation.x), -sinf(self.rotation.x),
            0.0, sinf(self.rotation.x), cosf(self.rotation.x)
        )
    }

    pub fn get_y_rotation_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
             cosf(self.rotation.y), 0.0, sinf(self.rotation.y),
             0.0,                   1.0, 0.0,
             -sinf(self.rotation.y), 0.0, cosf(self.rotation.y)
        )
    }

    pub fn get_z_rotation_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
            cosf(self.rotation.z), -sinf(self.rotation.z), 0.0,
            sinf(self.rotation.z), cosf(self.rotation.z),  0.0,
            0.0,                   0.0,                    1.0
        )
    }

    pub fn get_pos(&self) -> &Vector3<f32> {
        &self.pos
    }

    pub fn get_fov(&self) -> &f32 {
        &self.fov
    }
}
