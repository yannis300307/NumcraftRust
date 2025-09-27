use core::any::Any;

use libm::roundf;

use crate::chunk::{self, Chunk};
use crate::constants::world::CHUNK_SIZE;
use crate::constants::{BlockType, EntityType, ItemType};
use crate::entity::item::ItemEntityCustomData;
use crate::entity::{self, Entity};
use crate::inventory::{Inventory, ItemStack};
use crate::renderer::mesh::{Mesh, Quad};

#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use fastnoise_lite::FastNoiseLite;
use nalgebra::{Vector2, Vector3};

#[cfg(target_os = "none")]
use alloc::boxed::Box;

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct World {
    pub chunks: Vec<chunk::Chunk>,
    gen_noise: FastNoiseLite,
    registered_inventories: Vec<Inventory>,
    loaded_entities: Vec<Entity>,
    next_available_entity_id: usize,
}

pub struct RegisteredInventory {
    inventory: Inventory,
    block_pos: Option<Vector3<usize>>,
}

/// Convert the block position from world space to chunk space
pub fn get_chunk_local_coords(pos: Vector3<isize>) -> Vector3<isize> {
    Vector3::new(
        (pos.x % CHUNK_SIZE_I + CHUNK_SIZE_I) % CHUNK_SIZE_I,
        (pos.y % CHUNK_SIZE_I + CHUNK_SIZE_I) % CHUNK_SIZE_I,
        (pos.z % CHUNK_SIZE_I + CHUNK_SIZE_I) % CHUNK_SIZE_I,
    )
}

fn div_floor(a: isize, b: isize) -> isize {
    let (d, r) = (a / b, a % b);
    if (r != 0) && ((r < 0) != (b < 0)) {
        d - 1
    } else {
        d
    }
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
            chunks: Vec::new(),
            gen_noise: FastNoiseLite::new(),
            registered_inventories: Vec::new(),
            loaded_entities: vec![Entity::new(0, EntityType::Player, None)], // The player entity is always loaded and id 0
            next_available_entity_id: 1,
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
                    self.add_chunk(Vector3::new(x, y, z));
                    let chunk = self.chunks.last_mut().unwrap();
                    chunk.generate_chunk(&self.gen_noise);
                }
            }
        }
    }

    pub fn push_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    /// Used for rendering priority. Return a Vector of all the loaded chunks from the nearest to the farest
    pub fn get_chunks_sorted_by_distance(&mut self, pos: Vector3<f32>) -> Vec<&mut Chunk> {
        let mut chunks: Vec<&mut Chunk> = self.chunks.iter_mut().collect();

        chunks.sort_by(|a, b| {
            let a_dist = a
                .get_pos()
                .map(|x| (x * CHUNK_SIZE_I) as f32 + CHUNK_SIZE_I as f32 / 2.)
                .metric_distance(&pos);
            let b_dist = b
                .get_pos()
                .map(|x| (x * CHUNK_SIZE_I) as f32 + CHUNK_SIZE_I as f32 / 2.)
                .metric_distance(&pos);
            b_dist.total_cmp(&a_dist)
        });

        chunks
    }

    /// Return the terrain height at the given world block x-z coordinates. The vector must be (x, z)!
    pub fn get_terrain_height(&self, pos: Vector2<isize>) -> isize {
        let negative_1_to_1 = self.gen_noise.get_noise_2d((pos.x) as f32, (pos.y) as f32);
        roundf((negative_1_to_1 + 1.) / 2. * 14.0 + 8.0) as isize
    }

    /// Add a chunk and return a reference to it as an option
    pub fn add_chunk(&mut self, pos: Vector3<isize>) {
        let chunk = Chunk::new(pos);
        self.chunks.push(chunk);
    }

    /// Return true if a chunks is loaded at the given coordinates. The position is the position of the chunk and not the position of a block
    fn get_chunk_exists_at(&self, pos: Vector3<isize>) -> bool {
        for chunk in &self.chunks {
            if *chunk.get_pos() == pos {
                return true;
            }
        }
        false
    }

    /// Return the chunk at the given position. Return an Option containing a MUTABLE reference to the chunk
    fn get_chunk_at_pos_mut(&mut self, pos: Vector3<isize>) -> Option<&mut Chunk> {
        self.chunks.iter_mut().find(|chunk| *chunk.get_pos() == pos)
    }

    /// Return the chunk at the given position. Return an Option containing a reference to the chunk
    fn get_chunk_at_pos(&self, pos: Vector3<isize>) -> Option<&Chunk> {
        self.chunks.iter().find(|&chunk| *chunk.get_pos() == pos)
    }

    /// Delete all loaded chunks
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.clear_entities();
    }

    /// Set the world generation seed
    pub fn set_seed(&mut self, seed: i32) {
        self.gen_noise.seed = seed;
    }

    /// Generate the chunks around the given position The position is in global blocks space, not world chunk space
    pub fn generate_world_around_pos(&mut self, pos: Vector3<f32>, render_distance: isize) {
        // Convert global block space coordinates to chnuk space
        let pos_chunk_coords = Vector3::new(
            roundf(pos.x / CHUNK_SIZE as f32) as isize,
            roundf(pos.y / CHUNK_SIZE as f32) as isize,
            roundf(pos.z / CHUNK_SIZE as f32) as isize,
        );

        // Unload chunks that are no longer in the view distance
        self.chunks.retain(|chunk| {
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
                    if !self.get_chunk_exists_at(chunk_pos) {
                        self.add_chunk(chunk_pos);
                        let chunk = self.chunks.last_mut().unwrap();

                        chunk.generate_chunk(&self.gen_noise);

                        // Reload chunks around this chunk to prevent mesh gap issues
                        self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(-1, 0, 0));
                        self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(1, 0, 0));
                        self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, -1, 0));
                        self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, 1, 0));
                        self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, 0, -1));
                        self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, 0, 1));
                    }
                }
            }
        }

        // Generate or regenerate mesh if needed
        self.check_mesh_regeneration();
    }

    pub fn check_mesh_regeneration(&mut self) {
        for i in 0..self.chunks.len() {
            if self.chunks[i].need_new_mesh {
                let new_mesh = Mesh::generate_chunk(self, &self.chunks[i]);
                self.chunks[i].set_mesh(new_mesh);
            }
        }
    }

    /// Return the mesh of every chunks
    pub fn get_mesh(&mut self) -> Vec<&mut Vec<Quad>> {
        let mut world_mesh = Vec::new();
        for chunk in &mut self.chunks {
            world_mesh.push(&mut chunk.get_mesh().quads);
        }

        world_mesh
    }

    /// Return the block type of the block at the given position in world blocks space
    pub fn get_block_in_world(&self, pos: Vector3<isize>) -> Option<BlockType> {
        let chunk_pos = Vector3::new(
            div_floor(pos.x, CHUNK_SIZE_I),
            div_floor(pos.y, CHUNK_SIZE_I),
            div_floor(pos.z, CHUNK_SIZE_I),
        );
        self.get_chunk_at_pos(chunk_pos)
            .map(|chunk| chunk.get_at_unchecked(get_chunk_local_coords(pos)))
    }

    /// Request the regeneration of the chunk mesh if this chunk is already loaded
    fn request_mesh_regen_if_exists(&mut self, pos: Vector3<isize>) {
        if let Some(chunk) = self.get_chunk_at_pos_mut(pos) {
            chunk.need_new_mesh = true;
        }
    }

    /// Set the block type of the block at the given position in world blocks space. Regenerate chunk mesh if needed
    pub fn set_block_in_world(&mut self, pos: Vector3<isize>, block_type: BlockType) -> bool {
        let chunk_pos = Vector3::new(
            div_floor(pos.x, CHUNK_SIZE_I),
            div_floor(pos.y, CHUNK_SIZE_I),
            div_floor(pos.z, CHUNK_SIZE_I),
        );
        if let Some(chunk) = self.get_chunk_at_pos_mut(chunk_pos) {
            let local_pos = get_chunk_local_coords(pos);
            if chunk.set_at(local_pos.map(|x| x as usize), block_type) {
                chunk.need_new_mesh = true;

                if local_pos.x == 0 {
                    self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(-1, 0, 0));
                }
                if local_pos.x == CHUNK_SIZE_I - 1 {
                    self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(1, 0, 0));
                }
                if local_pos.y == 0 {
                    self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, -1, 0));
                }
                if local_pos.y == CHUNK_SIZE_I - 1 {
                    self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, 1, 0));
                }
                if local_pos.z == 0 {
                    self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, 0, -1));
                }
                if local_pos.z == CHUNK_SIZE_I - 1 {
                    self.request_mesh_regen_if_exists(chunk_pos + Vector3::new(0, 0, 1));
                }
                true
            } else {
                false
            }
        } else {
            false
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
                self.loaded_entities.pop();
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
        if let Some(current_block) = self.get_block_in_world(pos) {
            let drop_type = current_block.get_dropped_item_type();
            if drop_type != ItemType::Air {
                self.set_block_in_world(pos, block_type);
                self.spawn_item_entity(
                    pos.map(|v| v as f32 + 0.5),
                    ItemStack::new(drop_type, 1, false),
                );
            }
        }
    }
}
