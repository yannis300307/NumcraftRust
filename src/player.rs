use core::f32::consts::PI;

use libm::sincosf;
#[allow(unused_imports)]
use nalgebra::{ComplexField, Vector3};

use crate::{
    camera::Camera,
    constants::{
        BlockType, EntityType,
        player::{FLY_SPEED, JUMP_FORCE, MAX_WALKING_VELOCITY, WALK_FORCE},
    },
    eadk,
    entity::{Entity, item::ItemEntityCustomData},
    game::GameMode,
    hud::Hud,
    input_manager::InputManager,
    inventory::{Inventory, ItemStack},
    physic::PhysicEngine,
    renderer::mesh::{Mesh, Quad, QuadDir},
    world::World,
};

#[cfg(target_os = "none")]
use alloc::boxed::Box;

pub struct Player {
    ray_cast_result: Option<RaycastResult>,
    pub inventory: Inventory,
    breaking_state_timer: f32,
    breaking_block_pos: Option<Vector3<isize>>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            ray_cast_result: None,
            inventory: Inventory::new(24),
            breaking_state_timer: 0.,
            breaking_block_pos: None,
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

    pub fn get_block_breaking_progress(&self) -> Option<f32> {
        if self.breaking_block_pos.is_none() {
            return None;
        }
        let hardness = self.ray_cast_result.as_ref()?.block_type.get_hardness();
        if hardness <= 0. {
            return None;
        }

        Some((hardness - self.breaking_state_timer) / hardness)
    }

    pub fn sync_with_camera(&self, camera: &mut Camera, player_entity: &mut Entity) {
        camera.update_pos(player_entity.pos + Vector3::new(0., 1.2, 0.));
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
        game_mode: GameMode,
        physic_engine: &PhysicEngine,
        delta_time: f32,
    ) {
        self.ray_cast_result = self.ray_cast(camera, world, 10);

        let player_entity = world.get_player_entity_mut();

        self.sync_with_camera(camera, player_entity);
        player_entity.rotation = *camera.get_rotation();

        // Movements
        if input_manager.is_keydown(eadk::input::Key::Toolbox) {
            // Forward
            let translation = sincosf(player_entity.rotation.y);
            if game_mode == GameMode::Creative {
                player_entity.pos.x += translation.0 * delta * FLY_SPEED;
                player_entity.pos.z += translation.1 * delta * FLY_SPEED;
            } else {
                player_entity.velocity.x += translation.0 * delta * WALK_FORCE;
                player_entity.velocity.z += translation.1 * delta * WALK_FORCE;
            }
        }
        if input_manager.is_keydown(eadk::input::Key::Comma) {
            // Backward
            let translation = sincosf(player_entity.rotation.y);
            if game_mode == GameMode::Creative {
                player_entity.pos.x -= translation.0 * delta * FLY_SPEED;
                player_entity.pos.z -= translation.1 * delta * FLY_SPEED;
            } else {
                player_entity.velocity.x -= translation.0 * delta * WALK_FORCE;
                player_entity.velocity.z -= translation.1 * delta * WALK_FORCE;
            }
        }
        if input_manager.is_keydown(eadk::input::Key::Imaginary) {
            // Left
            let translation = sincosf(player_entity.rotation.y + PI / 2.0);
            if game_mode == GameMode::Creative {
                player_entity.pos.x += translation.0 * delta * FLY_SPEED;
                player_entity.pos.z += translation.1 * delta * FLY_SPEED;
            } else {
                player_entity.velocity.x += translation.0 * delta * WALK_FORCE;
                player_entity.velocity.z += translation.1 * delta * WALK_FORCE;
            }
        }
        if input_manager.is_keydown(eadk::input::Key::Power) {
            // Right
            let translation = sincosf(player_entity.rotation.y + PI / 2.0);
            if game_mode == GameMode::Creative {
                player_entity.pos.x -= translation.0 * delta * FLY_SPEED;
                player_entity.pos.z -= translation.1 * delta * FLY_SPEED;
            } else {
                player_entity.velocity.x -= translation.0 * delta * WALK_FORCE;
                player_entity.velocity.z -= translation.1 * delta * WALK_FORCE;
            }
        }
        if input_manager.is_keydown(eadk::input::Key::Shift) {
            // Up
            if game_mode == GameMode::Creative {
                player_entity.pos.y += delta * FLY_SPEED;
            } else if player_entity.is_on_floor {
                player_entity.velocity.y += JUMP_FORCE;
            }
        }
        if input_manager.is_keydown(eadk::input::Key::Exp) {
            // Down
            if game_mode == GameMode::Creative {
                player_entity.pos.y -= delta * FLY_SPEED;
            }
        }

        // Limit speed
        if player_entity.velocity.xz().norm() > MAX_WALKING_VELOCITY {
            let max_velocity = player_entity.velocity.xz().normalize() * MAX_WALKING_VELOCITY;
            player_entity.velocity.x = max_velocity.x;
            player_entity.velocity.z = max_velocity.y;
        }

        // Break Block
        if game_mode == GameMode::Creative {
            if input_manager.is_just_pressed(eadk::input::Key::Back) {
                if let Some(result) = &self.ray_cast_result {
                    world
                        .chunks_manager
                        .set_block_in_world(result.block_pos, BlockType::Air);
                }
            }
        } else {
            if input_manager.is_keydown(eadk::input::Key::Back) {
                if let Some(ray_cast) = &self.ray_cast_result {
                    if self
                        .breaking_block_pos
                        .is_some_and(|pos| pos == ray_cast.block_pos)
                    {
                        self.breaking_state_timer -= delta_time;
                        if self.breaking_state_timer <= 0. {
                            world.replace_block_and_drop_item(ray_cast.block_pos, BlockType::Air);
                            self.breaking_block_pos = None;
                            self.breaking_state_timer = 0.;
                        }
                    } else {
                        let hardness = ray_cast.block_type.get_hardness();
                        if hardness >= 0. {
                            self.breaking_block_pos = Some(ray_cast.block_pos);
                            self.breaking_state_timer = ray_cast.block_type.get_hardness();
                        }
                    }
                } else {
                    self.breaking_block_pos = None;
                    self.breaking_state_timer = 0.;
                }
            } else {
                self.breaking_block_pos = None;
                self.breaking_state_timer = 0.;
            }
        }

        if input_manager.is_just_pressed(eadk::input::Key::Ok) {
            // Place Block
            if let Some(result) = &self.ray_cast_result {
                let block_pos = result.block_pos + result.face_dir.get_normal_vector();
                if world
                    .chunks_manager
                    .get_block_in_world(block_pos)
                    .is_some_and(|b| b.is_air())
                    && physic_engine.can_place_block(world, block_pos)
                    && let Some(item_type) = self.inventory.take_one(0 + hud.selected_slot)
                    && let Some(block_type) = item_type.get_matching_block_type()
                {
                    world
                        .chunks_manager
                        .set_block_in_world(block_pos, block_type);
                }
            }
        }

        let player_entity = world.get_player_entity();

        if let Some(player_bbox) = player_entity.get_bbox() {
            world.get_all_entities_mut().retain_mut(|entity| {
                if let EntityType::Item { .. } = entity.get_type()
                    && entity
                        .get_bbox()
                        .is_some_and(|entity_bbox| entity_bbox.is_coliding(&player_bbox))
                {
                    // Recover the item_stack data from the item entity
                    let item_data = ItemEntityCustomData::get_item_data(&entity)
                        .expect("Item Entity must have ItemData as custom data.");

                    let item_stack = item_data.item_stack;

                    let remain = self.inventory.add_item_stack(item_stack.clone());

                    if remain != 0 {
                        entity.custom_data = Some(Box::new(ItemEntityCustomData {
                            item_stack: ItemStack::new(item_stack.get_item_type(), remain, false),
                        }));
                        return true;
                    }
                    false
                } else {
                    true
                }
            });
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
            let result = world
                .chunks_manager
                .get_block_in_world(current_voxel_pos_isize);
            if let Some(block_type) = result
                && !block_type.is_air()
            {
                let voxel_normal = if step_dir == 0 {
                    if dx < 0. {
                        QuadDir::Left
                    } else {
                        QuadDir::Right
                    }
                } else if step_dir == 1 {
                    if dy < 0. {
                        QuadDir::Top
                    } else {
                        QuadDir::Bottom
                    }
                } else if dz < 0. {
                    QuadDir::Back
                } else {
                    QuadDir::Front
                };

                return Some(RaycastResult {
                    block_pos: current_voxel_pos_isize,
                    face_dir: voxel_normal,
                    block_type: block_type,
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
    pub block_type: BlockType,
}
