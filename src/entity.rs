use nalgebra::Vector3;

use crate::{constants::physic::{GRAVITY_FACTOR, MAX_VELOCITY}, world::World};

#[derive(Clone)]
pub struct BoundingBox {
    pub offset: Vector3<f32>,
    pub size: Vector3<f32>,
}

#[derive(Clone)]
pub struct Entity {
    id: usize,
    pub bbox: Option<BoundingBox>,
    gravity: bool,
    pub pos: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub velocity: Vector3<f32>,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Entity {
            id,
            bbox: None,
            gravity: true,
            velocity: Vector3::zeros(),
            pos: Vector3::zeros(),
            rotation: Vector3::zeros(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.gravity {
            self.velocity.y += GRAVITY_FACTOR * delta_time;
        }

        if self.velocity.x > MAX_VELOCITY.x {
            self.velocity.x = MAX_VELOCITY.x;
        }
        if self.velocity.y > MAX_VELOCITY.y {
            self.velocity.y = MAX_VELOCITY.y;
        }
        if self.velocity.z > MAX_VELOCITY.z {
            self.velocity.z = MAX_VELOCITY.z;
        }

        self.pos += self.velocity * delta_time;
    }

    pub fn set_boundingbox(&mut self, bbox: BoundingBox) {
        self.bbox = Some(bbox);
    }
}
