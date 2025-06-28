use libm::roundf;

use crate::chunk::{self, Chunk};
use crate::constants::BlockType;
use crate::constants::world::CHUNK_SIZE;
use crate::mesh::{Mesh, Quad};
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use fastnoise_lite::FastNoiseLite;
use nalgebra::Vector3;

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct World {
    pub chunks: Vec<chunk::Chunk>,
    gen_noise: FastNoiseLite,
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
        };

        world
            .gen_noise
            .set_noise_type(Some(fastnoise_lite::NoiseType::OpenSimplex2));

        world
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
}
