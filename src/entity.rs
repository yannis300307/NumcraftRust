use core::any::Any;

use nalgebra::Vector3;

use crate::{constants::EntityType, physic::BoundingBox};

#[cfg(target_os = "none")]
use alloc::boxed::Box;

pub mod item;

pub struct Entity {
    id: usize,
    entity_type: EntityType,
    pub gravity: bool,
    pub pos: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub is_on_floor: bool,
    pub custom_data: Option<Box<dyn Any>>,
}

impl Entity {
    pub fn new(id: usize, entity_type: EntityType, custom_data: Option<Box<dyn Any>>) -> Self {
        Entity {
            id,
            entity_type,
            gravity: true,
            velocity: Vector3::zeros(),
            pos: Vector3::zeros(),
            rotation: Vector3::zeros(),
            is_on_floor: false,
            custom_data,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn get_bbox(&self) -> Option<BoundingBox> {
        Some(self.entity_type.get_bbox()?.transform(self.pos))
    }
}
