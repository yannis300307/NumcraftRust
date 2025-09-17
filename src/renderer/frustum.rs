use libm::tanf;
use nalgebra::Vector3;

use crate::camera::Camera;

struct Plane {
    normal: Vector3<f32>,
    pos: Vector3<f32>,
}

pub struct Frustum {
    top_plane: Plane,
    bottom_plane: Plane,

    right_plane: Plane,
    left_plane: Plane,

    far_plane: Plane,
    near_plane: Plane,
}

impl Frustum {
    pub fn new(camera: &Camera, aspect: f32, fov_y: f32, z_near: f32, z_far: f32) -> Self {
        let half_vside = z_far * tanf(fov_y * 0.5);
        let half_hside = half_vside * aspect;
        let front = camera.get_forward_vector();
        let up = camera.get_up_vector();
        let right = camera.get_right_vector();
        let front_mult_far = z_far * front;

        let cam_pos = *camera.get_pos();

        Frustum {
            near_plane: Plane {
                pos: cam_pos + z_near * front,
                normal: front,
            },
            far_plane: Plane {
                pos: cam_pos + front_mult_far,
                normal: -front,
            },
            right_plane: Plane {
                pos: cam_pos,
                normal: up.cross(&(front_mult_far - right * half_hside)),
            },
            left_plane: Plane {
                pos: cam_pos,
                normal: (front_mult_far + right * half_hside).cross(&up),
            },
            top_plane: Plane {
                pos: cam_pos,
                normal: (front_mult_far - up * half_vside).cross(&right),
            },
            bottom_plane: Plane {
                pos: cam_pos,
                normal: right.cross(&(front_mult_far + up * half_vside)),
            },
        }
    }

    pub fn is_aabb_in_frustum(&self, min: Vector3<f32>, max: Vector3<f32>) -> bool {
        let corners = [
            Vector3::new(min.x, min.y, min.z),
            Vector3::new(max.x, min.y, min.z),
            Vector3::new(min.x, max.y, min.z),
            Vector3::new(max.x, max.y, min.z),
            Vector3::new(min.x, min.y, max.z),
            Vector3::new(max.x, min.y, max.z),
            Vector3::new(min.x, max.y, max.z),
            Vector3::new(max.x, max.y, max.z),
        ];
        for plane in [
            &self.far_plane,
            &self.near_plane,
            &self.right_plane,
            &self.left_plane,
            &self.top_plane,
            &self.bottom_plane,
        ] {
            if corners
                .iter()
                .all(|c| (c - plane.pos).dot(&plane.normal) < 0.0)
            {
                return false;
            }
        }
        true
    }
}
