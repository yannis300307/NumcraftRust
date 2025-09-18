use core::f32::consts::PI;

use libm::sincosf;
use nalgebra::{ComplexField, Vector3};

use crate::{
    camera::Camera, constants::{player::MOVEMENT_SPEED, BlockType}, eadk, entity::Entity, hud::Hud, input_manager::InputManager, inventory::Inventory, physic::BoundingBox, renderer::mesh::{Mesh, Quad, QuadDir}, world::World
};

pub struct Player {
    ray_cast_result: Option<RaycastResult>,
    pub inventory: Inventory,
}

impl Player {
    pub fn new(player_entity: &mut Entity) -> Self {
        player_entity.bbox = Some(BoundingBox {
            offset: Vector3::new(-0.3, 0., -0.3),
            size: Vector3::new(0.6, 1.8, 0.6),
        });
        
        Player {
            ray_cast_result: None,
            inventory: Inventory::new(24),
        }
    }

    pub fn get_block_marker(&self) -> (Mesh, Vector3<isize>) {
        let mut mesh = Mesh::new();

        if let Some(result) = &self.ray_cast_result {
            mesh.quads
                .push(Quad::new(Vector3::new(0, 0, 0), result.face_dir, 255, 0));
            (mesh, result.block_pos)
        } else {
            (mesh, Vector3::repeat(0))
        }
    }

    pub fn sync_with_camera(&self, camera: &mut Camera, player_entity: &mut Entity) {
        camera.update_pos(player_entity.pos - Vector3::new(0., 1.70, 0.));
        player_entity.rotation = *camera.get_rotation();
    }

    pub fn set_inventory(&mut self, inventory: Inventory) {
        self.inventory = inventory
    }

    pub fn update(
        &mut self,
        delta: f32,
        input_manager: &InputManager,
        world: &mut World,
        camera: &mut Camera,
        hud: &Hud,
    ) {
        self.ray_cast_result = self.ray_cast(camera, world, 10);

        let player_entity = world.get_player_entity_mut();

        self.sync_with_camera(camera, player_entity);
        player_entity.rotation = *camera.get_rotation();

        // Movements
        if input_manager.is_keydown(eadk::input::Key::Toolbox) {
            // Forward
            let translation = sincosf(player_entity.rotation.y);
            player_entity.pos.x += translation.0 * delta * MOVEMENT_SPEED;
            player_entity.pos.z += translation.1 * delta * MOVEMENT_SPEED;
        }
        if input_manager.is_keydown(eadk::input::Key::Comma) {
            // Backward
            let translation = sincosf(player_entity.rotation.y);
            player_entity.pos.x -= translation.0 * delta * MOVEMENT_SPEED;
            player_entity.pos.z -= translation.1 * delta * MOVEMENT_SPEED;
        }
        if input_manager.is_keydown(eadk::input::Key::Imaginary) {
            // Left
            let translation = sincosf(player_entity.rotation.y + PI / 2.0);
            player_entity.pos.x -= translation.0 * delta * MOVEMENT_SPEED;
            player_entity.pos.z -= translation.1 * delta * MOVEMENT_SPEED;
        }
        if input_manager.is_keydown(eadk::input::Key::Power) {
            // Right
            let translation = sincosf(player_entity.rotation.y + PI / 2.0);
            player_entity.pos.x += translation.0 * delta * MOVEMENT_SPEED;
            player_entity.pos.z += translation.1 * delta * MOVEMENT_SPEED;
        }
        if input_manager.is_keydown(eadk::input::Key::Shift) {
            // Up
            player_entity.pos.y -= delta * MOVEMENT_SPEED;
        }
        if input_manager.is_keydown(eadk::input::Key::Exp) {
            // Down
            player_entity.pos.y += delta * MOVEMENT_SPEED;
        }

        if input_manager.is_just_pressed(eadk::input::Key::Back) {
            // Break Block
            if let Some(result) = &self.ray_cast_result {
                world.set_block_in_world(result.block_pos, BlockType::Air);
            }
        }

        if input_manager.is_just_pressed(eadk::input::Key::Ok) {
            // Place Block
            if let Some(result) = &self.ray_cast_result {
                let block_pos = result.block_pos + result.face_dir.get_normal_vector();
                if world
                    .get_block_in_world(block_pos)
                    .is_some_and(|b| b.is_air())
                // Just in case
                    && let Some(item_type) = self.inventory.take_one(18 + hud.selected_slot)
                        && let Some(block_type) = item_type.get_matching_block_type()
                {
                    world.set_block_in_world(block_pos, block_type);
                }
            }
        }
    }

    fn ray_cast(&self, camera: &Camera, world: &World, max_lenght: usize) -> Option<RaycastResult> {
        let start_pos = *camera.get_pos();
        let forward_vector = camera.get_forward_vector();

        let end_pos = start_pos + forward_vector.normalize() * (max_lenght as f32);

        let mut current_voxel_pos = start_pos;
        let mut step_dir = -1;

        let dx = (end_pos.x - start_pos.x).signum();
        let delta_x = if dx != 0. {
            (dx / (end_pos.x - start_pos.x)).min(10000000.0)
        } else {
            10000000.0
        };
        let mut max_x = if dx > 0. {
            delta_x * (1.0 - start_pos.x.fract())
        } else {
            delta_x * start_pos.x.fract()
        };

        let dy = (end_pos.y - start_pos.y).signum();
        let delta_y = if dy != 0. {
            (dy / (end_pos.y - start_pos.y)).min(10000000.0)
        } else {
            10000000.0
        };
        let mut max_y = if dy > 0. {
            delta_y * (1.0 - start_pos.y.fract())
        } else {
            delta_y * start_pos.y.fract()
        };

        let dz = (end_pos.z - start_pos.z).signum();
        let delta_z = if dz != 0. {
            (dz / (end_pos.z - start_pos.z)).min(10000000.0)
        } else {
            10000000.0
        };
        let mut max_z = if dz > 0. {
            delta_z * (1.0 - start_pos.z.fract())
        } else {
            delta_z * start_pos.z.fract()
        };

        while !(max_x > 1.0 && max_y > 1.0 && max_z > 1.0) {
            let current_voxel_pos_isize = current_voxel_pos.map(|x| x as isize);
            let result = world.get_block_in_world(current_voxel_pos_isize);
            if !result.is_none_or(|b| b == BlockType::Air) {
                let voxel_normal = if step_dir == 0 {
                    if dx < 0. {
                        QuadDir::Right
                    } else {
                        QuadDir::Left
                    }
                } else if step_dir == 1 {
                    if dy < 0. {
                        QuadDir::Bottom
                    } else {
                        QuadDir::Top
                    }
                } else if dz < 0. {
                    QuadDir::Back
                } else {
                    QuadDir::Front
                };
                return Some(RaycastResult {
                    block_pos: current_voxel_pos_isize,
                    face_dir: voxel_normal,
                });
            }

            if max_x < max_y {
                if max_x < max_z {
                    current_voxel_pos.x += dx;
                    max_x += delta_x;
                    step_dir = 0;
                } else {
                    current_voxel_pos.z += dz;
                    max_z += delta_z;
                    step_dir = 2;
                }
            } else if max_y < max_z {
                current_voxel_pos.y += dy;
                max_y += delta_y;
                step_dir = 1;
            } else {
                current_voxel_pos.z += dz;
                max_z += delta_z;
                step_dir = 2
            }
        }
        None
    }
}

struct RaycastResult {
    pub block_pos: Vector3<isize>,
    pub face_dir: QuadDir,
}
