use libm::sincosf;
use nalgebra::Vector3;

use crate::{
    constants::physic::{BLOCK_COLLISION_SCANNING_RADIUS, GRAVITY_FACTOR, ON_FLOOR_FRICTION},
    entity::{self, Entity},
    world::World,
};

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub offset: Vector3<f32>,
    pub size: Vector3<f32>,
}

pub struct PhysicEngine {}

impl PhysicEngine {
    pub fn new() -> Self {
        PhysicEngine {}
    }

    pub fn process(&self, world: &mut World, delta_time: f32) {
        for entity in world.get_all_entities_mut().iter_mut() {
            if entity.gravity {
                entity.velocity.y += GRAVITY_FACTOR * delta_time;
            }
        }

        for index in 0..world.get_all_entities().len() {
            let entity = &world.get_all_entities_mut()[index];
            let id = entity.get_id();

            let movement = entity.velocity * delta_time;
            self.move_entity(world, id, Vector3::new(0., movement.y, 0.));
            self.move_entity(world, id, Vector3::new(movement.x, 0., 0.));
            self.move_entity(world, id, Vector3::new(0., 0., movement.z));
        }

        // Friction
        for entity in world.get_all_entities_mut().iter_mut() {
            if entity.velocity.x < 0. {
                entity.velocity.x += (ON_FLOOR_FRICTION * delta_time).min(-entity.velocity.x);
            } else if entity.velocity.x > 0. {
                entity.velocity.x -= (ON_FLOOR_FRICTION * delta_time).min(entity.velocity.x);
            }
            if entity.velocity.z < 0. {
                entity.velocity.z += (ON_FLOOR_FRICTION * delta_time).min(-entity.velocity.z);
            } else if entity.velocity.z > 0. {
                entity.velocity.z -= (ON_FLOOR_FRICTION * delta_time).min(entity.velocity.z);
            }
        }
    }

    pub fn move_entity(&self, world: &mut World, entity_id: usize, movement: Vector3<f32>) {
        if let Some(state) = self.is_entity_colliding_world(entity_id, world, movement)
            && let Some(entity) = world.get_entity_by_id_mut(entity_id)
        {
            if state {
                if movement.y > 0. {
                    entity.is_on_floor = true;
                }

                if movement.y.abs() > 0. {
                    entity.velocity.y = 0.;
                }
                if movement.x.abs() > 0. {
                    entity.velocity.x = 0.;
                }
                if movement.z.abs() > 0. {
                    entity.velocity.z = 0.;
                }
            } else {
                entity.pos += movement;

                if movement.y.abs() > 0. {
                    entity.is_on_floor = false;
                }
            }
        }
    }

    pub fn is_entity_colliding_world(
        &self,
        entity_id: usize,
        world: &World,
        movement: Vector3<f32>,
    ) -> Option<bool> {
        let entity = world.get_entity_by_id(entity_id)?;

        let entity_block_pos = (entity.pos + movement).map(|v| v as isize);

        if let Some(entity_bbox) = entity.get_bbox() {
            for bx in (entity_block_pos.x - BLOCK_COLLISION_SCANNING_RADIUS)
                ..(entity_block_pos.x + BLOCK_COLLISION_SCANNING_RADIUS)
            {
                for by in (entity_block_pos.y - BLOCK_COLLISION_SCANNING_RADIUS)
                    ..(entity_block_pos.y + BLOCK_COLLISION_SCANNING_RADIUS)
                {
                    for bz in (entity_block_pos.z - BLOCK_COLLISION_SCANNING_RADIUS)
                        ..(entity_block_pos.z + BLOCK_COLLISION_SCANNING_RADIUS)
                    {
                        if let Some(block) = world.get_block_in_world(Vector3::new(bx, by, bz))
                            && !block.is_air()
                        {
                            let block_bbox = BoundingBox {
                                offset: Vector3::new(bx as f32, by as f32, bz as f32),
                                size: Vector3::repeat(1.0),
                            };

                            if block_bbox.is_coliding(&entity_bbox.transform(movement)) {
                                return Some(true);
                            }
                        }
                    }
                }
            }
        }
        Some(false)
    }
}

impl BoundingBox {
    pub fn is_coliding(&self, other: &BoundingBox) -> bool {
        println!("{:?}, {:?}", self, other);
        self.offset.x < other.offset.x + other.size.x
            && self.offset.x + self.size.x > other.offset.x
            && self.offset.y < other.offset.y + other.size.y
            && self.offset.y + self.size.y > other.offset.y
            && self.offset.z < other.offset.z + other.size.z
            && self.offset.z + self.size.z > other.offset.z
    }

    pub fn transform(&self, vector: Vector3<f32>) -> BoundingBox {
        BoundingBox {
            offset: self.offset + vector,
            size: self.size,
        }
    }
}
