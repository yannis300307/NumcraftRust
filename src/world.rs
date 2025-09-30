use core::any::Any;

use libm::roundf;

use crate::constants::world::{
    CHUNK_SIZE, ITEM_MAGNET_FORCE, MAX_ITEM_MERGING_DISTANCE, MAX_PLAYER_ITEM_MAGNET_DISTANCE,
};
use crate::constants::{BlockType, EntityType, ItemType};
use crate::entity::Entity;
use crate::entity::item::ItemEntityCustomData;
use crate::inventory::{Inventory, ItemStack};
use crate::renderer::mesh::Mesh;
use crate::world::chunk_container::ChunksManager;
use crate::world::world_generator::WorldGenerator;

#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use fastnoise_lite::FastNoiseLite;
use nalgebra::{Vector2, Vector3};

#[cfg(target_os = "none")]
use alloc::boxed::Box;

pub mod chunk;
pub mod chunk_container;
pub mod world_generator;

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct World {
    pub chunks_manager: ChunksManager,
    gen_noise: FastNoiseLite,
    registered_inventories: Vec<Inventory>,
    loaded_entities: Vec<Entity>,
    next_available_entity_id: usize,
    world_generator: WorldGenerator,
}

pub struct RegisteredInventory {
    inventory: Inventory,
    block_pos: Option<Vector3<usize>>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the current world. Contains all the chunks
impl World {
    pub fn new() -> Self {
        let mut world = World {
            chunks_manager: ChunksManager::new(),
            gen_noise: FastNoiseLite::new(),
            registered_inventories: Vec::new(),
            loaded_entities: vec![Entity::new(0, EntityType::Player, None)], // The player entity is always loaded and id 0
            next_available_entity_id: 1,
            world_generator: WorldGenerator::new(),
        };

        world
            .gen_noise
            .set_noise_type(Some(fastnoise_lite::NoiseType::OpenSimplex2));

        world
    }

    pub fn get_player_entity_mut(&mut self) -> &mut Entity {
        &mut self.loaded_entities[0]
    }

    pub fn get_player_entity(&self) -> &Entity {
        &self.loaded_entities[0]
    }

    pub fn load_area(
        &mut self,
        x_start: isize,
        x_stop: isize,
        y_start: isize,
        y_stop: isize,
        z_start: isize,
        z_stop: isize,
    ) {
        for x in x_start..x_stop {
            for y in y_start..y_stop {
                for z in z_start..z_stop {
                    self.chunks_manager.add_chunk(Vector3::new(x, y, z));
                    let chunk = self.chunks_manager.chunks.last_mut().unwrap();
                    self.world_generator.generate_chunk(chunk);
                }
            }
        }
    }

    pub fn update_entities(&mut self, delta_time: f32) {
        // Check for item merging and player magnet
        'first_loop: for i in 0..self.loaded_entities.len() {
            if self.loaded_entities[i].get_type() == EntityType::Item {
                // Ignore items with custom data = None, they will be removed after
                if self.loaded_entities[i].custom_data.is_none() {
                    continue;
                }

                // Get the item_data from the first item
                let first_item_data = ItemEntityCustomData::get_item_data(&self.loaded_entities[i])
                    .expect("Item Entity must have ItemData as custom data.");
                let first_item_stack = first_item_data.item_stack.clone();

                let max_stack = first_item_stack.get_item_type().get_max_stack_amount();

                for j in 0..self.loaded_entities.len() {
                    if i != j
                        && self.loaded_entities[j].custom_data.is_some() // Ignore items with custom data = None, they will be removed after
                        && self.loaded_entities[j].get_type() == EntityType::Item
                        && self.loaded_entities[i]
                            .pos
                            .metric_distance(&self.loaded_entities[j].pos)
                            <= MAX_ITEM_MERGING_DISTANCE
                    {
                        // Check if the items can merge
                        let second_item_data =
                            ItemEntityCustomData::get_item_data(&self.loaded_entities[j])
                                .expect("Item Entity must have ItemData as custom data.");
                        let second_item_stack = second_item_data.item_stack.clone();

                        if second_item_stack.get_item_type() != first_item_stack.get_item_type() {
                            continue;
                        }

                        if let Some(first_bbox) = self.loaded_entities[i].get_bbox()
                            && let Some(second_bbox) = self.loaded_entities[j].get_bbox()
                            && first_bbox.is_coliding(&second_bbox)
                        {
                            if first_item_stack.get_amount() == max_stack
                                || second_item_stack.get_amount() == max_stack
                            {
                                continue;
                            }

                            let total =
                                first_item_stack.get_amount() + second_item_stack.get_amount();
                            if total <= max_stack {
                                // Merge the two items together and request the deletion of the second one
                                self.loaded_entities[i].custom_data =
                                    Some(Box::new(ItemEntityCustomData {
                                        item_stack: ItemStack::new(
                                            first_item_stack.get_item_type(),
                                            total,
                                            false,
                                        ),
                                    }));
                                self.loaded_entities[j].custom_data = None; // Yes, this should be illegal but it can also be a feature.
                                self.loaded_entities[i].velocity = Vector3::zeros();
                                continue 'first_loop;
                            } else {
                                self.loaded_entities[i].custom_data =
                                    Some(Box::new(ItemEntityCustomData {
                                        item_stack: ItemStack::new(
                                            first_item_stack.get_item_type(),
                                            max_stack,
                                            false,
                                        ),
                                    }));
                                self.loaded_entities[j].custom_data =
                                    Some(Box::new(ItemEntityCustomData {
                                        item_stack: ItemStack::new(
                                            first_item_stack.get_item_type(),
                                            total - max_stack,
                                            false,
                                        ),
                                    }));
                                self.loaded_entities[i].velocity = Vector3::zeros();
                                self.loaded_entities[j].velocity = Vector3::zeros();
                            }
                            continue;
                        }

                        // Calculate the direction to the other item
                        let direction =
                            (self.loaded_entities[j].pos - self.loaded_entities[i].pos).normalize();

                        self.loaded_entities[i].velocity +=
                            direction * ITEM_MAGNET_FORCE * delta_time;

                        // Limit the magnet speed
                        /*if self.loaded_entities[i].velocity.norm() > ITEM_MAGNET_SPEED {
                            self.loaded_entities[i].velocity =
                                self.loaded_entities[i].velocity.normalize()
                                    * ITEM_MAGNET_SPEED
                                    * delta_time;
                        }*/
                    }
                }
            }
        }

        // Player item magnet
        for i in 0..self.loaded_entities.len() {
            let distance = self.loaded_entities[i]
                .pos
                .metric_distance(&self.get_player_entity().pos);
            if self.loaded_entities[i].get_type() == EntityType::Item
                && self.loaded_entities[i].custom_data.is_some()
                && distance < MAX_PLAYER_ITEM_MAGNET_DISTANCE
            {
                let direction =
                    (self.get_player_entity().pos - self.loaded_entities[i].pos).normalize();

                self.loaded_entities[i].velocity += direction * ITEM_MAGNET_FORCE * delta_time;
            }
        }

        // Remove illegal items
        self.loaded_entities.retain(|entity| {
            entity.get_type() != EntityType::Item
                || (entity.get_type() == EntityType::Item && !entity.custom_data.is_none())
        });
    }

    /// Return the terrain height at the given world block x-z coordinates. The vector must be (x, z)!
    pub fn get_terrain_height(&self, pos: Vector2<isize>) -> isize {
        todo!("DEPRECATED: remaque this");
        let negative_1_to_1 = self.gen_noise.get_noise_2d((pos.x) as f32, (pos.y) as f32);
        roundf((negative_1_to_1 + 1.) / 2. * 14.0 - 2.0) as isize
    }

    /// Set the world generation seed
    pub fn set_seed(&mut self, seed: i32) {
        self.gen_noise.seed = seed;
    }

    /// Generate the chunks around the given position The position is in global blocks space, not world chunk space
    pub fn generate_world_around_pos(
        &mut self,
        pos: Vector3<f32>,
        render_distance: isize,
        world_generator: &mut WorldGenerator,
    ) {
        // Convert global block space coordinates to chnuk space
        let pos_chunk_coords = Vector3::new(
            roundf(pos.x / CHUNK_SIZE as f32) as isize,
            roundf(pos.y / CHUNK_SIZE as f32) as isize,
            roundf(pos.z / CHUNK_SIZE as f32) as isize,
        );

        // Unload chunks that are no longer in the view distance
        self.chunks_manager.chunks.retain(|chunk| {
            let relative_chunk_pos = chunk.get_pos() - pos_chunk_coords;
            !(relative_chunk_pos.x < -render_distance
                || relative_chunk_pos.x >= render_distance
                || relative_chunk_pos.y < -render_distance
                || relative_chunk_pos.y >= render_distance
                || relative_chunk_pos.z < -render_distance
                || relative_chunk_pos.z >= render_distance)
        });

        // Load chunks around
        for x in -render_distance..render_distance {
            for y in -render_distance..render_distance {
                for z in -render_distance..render_distance {
                    let chunk_pos: Vector3<isize> = Vector3::new(x, y, z) + pos_chunk_coords;

                    // Prevent creating chunks that already exist
                    if !self.chunks_manager.get_chunk_exists_at(chunk_pos) {
                        self.chunks_manager.add_chunk(chunk_pos);
                        let chunk = self.chunks_manager.chunks.last_mut().unwrap();

                        world_generator.generate_chunk(chunk);

                        // Reload chunks around this chunk to prevent mesh gap issues
                        self.chunks_manager
                            .request_mesh_regen_if_exists(chunk_pos + Vector3::new(-1, 0, 0));
                        self.chunks_manager
                            .request_mesh_regen_if_exists(chunk_pos + Vector3::new(1, 0, 0));
                        self.chunks_manager
                            .request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, -1, 0));
                        self.chunks_manager
                            .request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, 1, 0));
                        self.chunks_manager
                            .request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, 0, -1));
                        self.chunks_manager
                            .request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, 0, 1));
                    }
                }
            }
        }

        // Generate or regenerate mesh if needed
        self.check_mesh_regeneration();
    }

    pub fn check_mesh_regeneration(&mut self) {
        for i in 0..self.chunks_manager.chunks.len() {
            if self.chunks_manager.chunks[i].need_new_mesh {
                let new_mesh = Mesh::generate_chunk(self, &self.chunks_manager.chunks[i]);
                self.chunks_manager.chunks[i].set_mesh(new_mesh);
            }
        }
    }

    fn register_inventory(&mut self, inventory: Inventory) {
        self.registered_inventories.push(inventory);
    }

    pub fn get_all_entities_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.loaded_entities
    }
    pub fn get_all_entities(&self) -> &Vec<Entity> {
        &self.loaded_entities
    }

    pub fn get_entity_by_id_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.loaded_entities
            .iter_mut()
            .find(|entity| entity.get_id() == id)
    }

    pub fn get_entity_by_id(&self, id: usize) -> Option<&Entity> {
        self.loaded_entities
            .iter()
            .find(|entity| entity.get_id() == id)
    }

    pub fn spawn_entity(&mut self, mut entity: Entity, pos: Vector3<f32>) {
        entity.pos = pos;
        self.loaded_entities.push(entity);
    }

    pub fn get_new_entity_id(&mut self) -> usize {
        let id = self.next_available_entity_id;
        self.next_available_entity_id += 1;
        id
    }

    pub fn clear_entities(&mut self) {
        if self.loaded_entities.len() > 1 {
            for i in 1..self.loaded_entities.len() {
                self.loaded_entities.remove(i);
            }
        }
    }

    pub fn spawn_entity_auto(
        &mut self,
        entity_type: EntityType,
        pos: Vector3<f32>,
        custom_data: Option<Box<dyn Any>>,
    ) {
        let id = self.get_new_entity_id();
        self.spawn_entity(Entity::new(id, entity_type, custom_data), pos);
    }

    pub fn spawn_item_entity(&mut self, pos: Vector3<f32>, item_stack: ItemStack) {
        self.spawn_entity_auto(
            EntityType::Item,
            pos,
            Some(Box::new(ItemEntityCustomData { item_stack })),
        );
    }

    pub fn replace_block_and_drop_item(&mut self, pos: Vector3<isize>, block_type: BlockType) {
        if let Some(current_block) = self.chunks_manager.get_block_in_world(pos) {
            let drop_type = current_block.get_dropped_item_type();
            if drop_type != ItemType::Air {
                self.chunks_manager.set_block_in_world(pos, block_type);
                self.spawn_item_entity(
                    pos.map(|v| v as f32 + 0.5),
                    ItemStack::new(drop_type, 1, false),
                );
            }
        }
    }

    pub fn remove_entity(&mut self, id: usize) -> bool {
        for i in 0..self.loaded_entities.len() {
            if self.loaded_entities[i].get_id() == id {
                self.loaded_entities.remove(i);
                return true;
            }
        }
        false
    }

    /// Clear all the chunks and entities
    pub fn clear(&mut self) {
        self.chunks_manager.clear();
        self.clear_entities();
    }
}
