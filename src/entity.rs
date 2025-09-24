use nalgebra::Vector3;

use crate::{
    constants::{
        EntityType,
        physic::{GRAVITY_FACTOR, MAX_VELOCITY},
    },
    physic::BoundingBox,
};

#[derive(Clone)]
pub struct Entity {
    id: usize,
    entity_type: EntityType,
    pub gravity: bool,
    pub pos: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub is_on_floor: bool,
}

impl Entity {
    pub fn new(id: usize, entity_type: EntityType) -> Self {
        Entity {
            id,
            entity_type,
            gravity: true,
            velocity: Vector3::zeros(),
            pos: Vector3::zeros(),
            rotation: Vector3::zeros(),
            is_on_floor: false,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
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

    pub fn get_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn get_bbox(&self) -> Option<BoundingBox> {
        Some(self.entity_type.get_bbox()?.transform(self.pos))
    }
}
