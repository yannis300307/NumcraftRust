use core::f32::consts::PI;

use libm::{cosf, sincosf, sinf};
use nalgebra::{ComplexField, Vector3};

use crate::{
    camera::Camera,
    constants::{
        player::MOVEMENT_SPEED, BlockType,
        rendering::SCREEN_WIDTH, // Import SCREEN_WIDTH for display debug. SCREEN_HEIGHT is unused here.
        UI_BLACK, // For debug text background
        QuadDir, // Import QuadDir directly from constants (re-exported from mesh)
    },
    eadk::{self, Color, Point, Rect, display}, // Corrected: import `Point` and `Rect` struct, and `display` module
    inventory::Inventory, // Import Inventory
    mesh::{Mesh, Quad}, // QuadDir is imported via constants now
    world::{get_chunk_local_coords, get_chunk_pos_from_block, World},
};

pub struct Player {
    pub pos: Vector3<f32>,
    pub rotation: Vector3<f32>,
    ray_cast_result: Option<RaycastResult>,
    pub inventory: Inventory, // Player's inventory
    debug_message: Option<(&'static str, u64)>, // Changed u32 to u64 for end_time (matching eadk::timing::millis)
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
            ray_cast_result: None,
            inventory: Inventory::new(), // Initialize the player's inventory
            debug_message: None,
        }
    }

    pub fn get_block_marker(&self) -> (Mesh, Vector3<isize>) {
        let mut mesh = Mesh::new();

        if let Some(result) = &self.ray_cast_result {
            // Use 255 as texture_id for the block outline
            // Make sure get_quad_color_from_texture_id handles ID 255 correctly
            mesh.quads.push(Quad::new(
                get_chunk_local_coords(result.block_pos).map(|x| x as u16),
                result.face_dir,
                255, // Use 255 for the marker (outline)
                0,   // Alpha, if used for the marker
            ));
            (mesh, get_chunk_pos_from_block(result.block_pos))
        } else {
            (mesh, Vector3::repeat(0))
        }
    }

    pub fn update(
        &mut self,
        delta: f32,
        keyboard_state: eadk::input::KeyboardState,
        just_pressed_keyboard_state: eadk::input::KeyboardState,
        world: &mut World,
        camera: &mut Camera,
    ) {
        // Update camera and player rotation
        camera.update(delta, keyboard_state, self.pos - Vector3::new(0., 1.70, 0.));
        self.rotation = *camera.get_rotation();

        // Raycast for block selection
        self.ray_cast_result = self.ray_cast(camera, world, 10);

        // Player movements
        if keyboard_state.key_down(eadk::input::Key::Toolbox) {
            // Forward
            let translation = sincosf(self.rotation.y);
            self.pos.x += translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z += translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Comma) {
            // Backward
            let translation = sincosf(self.rotation.y);
            self.pos.x -= translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z -= translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Imaginary) {
            // Left
            let translation = sincosf(self.rotation.y + PI / 2.0);
            self.pos.x -= translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z -= translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Power) {
            // Right
            let translation = sincosf(self.rotation.y + PI / 2.0);
            self.pos.x += translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z += translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Shift) {
            // Up
            self.pos.y -= delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Exp) {
            // Down
            self.pos.y += delta * MOVEMENT_SPEED;
        }

        // Inventory selection
        if just_pressed_keyboard_state.key_down(eadk::input::Key::Plus) {
            self.inventory.select_next_slot();
        }
        if just_pressed_keyboard_state.key_down(eadk::input::Key::Minus) {
            self.inventory.select_previous_slot();
        }

        // Block interaction (break/place)
        if just_pressed_keyboard_state.key_down(eadk::input::Key::Back) {
            // Break a block
            if let Some(result) = &self.ray_cast_result {
                if result.block_type != BlockType::Air { // Ensure we don't break air
                    world.set_block_in_world(result.block_pos, BlockType::Air);
                    // For a full game, you would add the broken block to the inventory here.
                    // self.inventory.add_block(result.block_type);
                }
            }
        }

        if just_pressed_keyboard_state.key_down(eadk::input::Key::Ok) {
            // Place a block
            if let Some(result) = &self.ray_cast_result {
                let block_to_place = self.inventory.get_selected_block_type();

                // Only place the block if the selected slot is not "Air"
                if block_to_place != BlockType::Air {
                    let block_pos = result.block_pos + result.face_dir.get_normal_vector();

                    // Check if the target position is air (empty)
                    if world.get_block_in_world(block_pos).is_some_and(|b| b.is_air()) {
                        // Simple player collision check
                        // Adjust player bounding box dimensions if necessary
                        let player_bbox_min = self.pos - Vector3::new(0.4, 0.0, 0.4);
                        let player_bbox_max = self.pos + Vector3::new(0.4, 1.8, 0.4);

                        // Fix: Add parentheses around the `as f32` casts to resolve parsing ambiguity
                        let block_collides_with_player =
                            ((block_pos.x as f32) + 1.0 > player_bbox_min.x && (block_pos.x as f32) < player_bbox_max.x) &&
                            ((block_pos.y as f32) + 1.0 > player_bbox_min.y && (block_pos.y as f32) < player_bbox_max.y) &&
                            ((block_pos.z as f32) + 1.0 > player_bbox_min.z && (block_pos.z as f32) < player_bbox_max.z);

                        if !block_collides_with_player {
                            world.set_block_in_world(block_pos, block_to_place);
                            // To consume the block from inventory after placing:
                            // self.inventory.remove_block(block_to_place); // Implement this method if needed
                        } else {
                            self.set_debug_message("Cannot place block inside player!", 1500); // Display message for 1.5s
                        }
                    }
                } else {
                    self.set_debug_message("Selected slot is empty, cannot place!", 1500);
                }
            }
        }

        // Update debug message timer
        if let Some((_, end_time)) = self.debug_message {
            // Fix: No explicit cast needed if `debug_message` end_time is already `u64` and `duration_ms` is cast to `u64`
            if eadk::timing::millis() >= end_time {
                self.debug_message = None;
            }
        }
    }

    // Function to set a temporary debug message
    pub fn set_debug_message(&mut self, message: &'static str, duration_ms: u32) {
        // Fix: Add duration_ms as u64 to millis() result, then store in u64
        self.debug_message = Some((message, eadk::timing::millis() + duration_ms as u64));
    }

    // Function to draw the debug message
    // Removed `display: &mut Display` parameter
    pub fn draw_debug_message(&self) {
        if let Some((message, _)) = self.debug_message {
            // Position the message at the top of the screen, centered
            let text_x = (SCREEN_WIDTH as u16 / 2) - (message.len() as u16 * 3); // Rough estimation of text width
            let text_y = 5; // A few pixels from the top

            // Draw a semi-transparent or opaque background for the text using `eadk::display::push_rect_uniform`
            display::push_rect_uniform(
                Rect { x: text_x - 5, y: text_y - 2, width: (message.len() * 6 + 10) as u16, height: 12 },
                UI_BLACK
            );

            // Draw the message text using `eadk::display::draw_string`
            // Fix: Use Color::from_888 instead of non-existent Color::from_rgb
            display::draw_string(
                message,
                Point { x: text_x, y: text_y },
                false, // Assuming false for small font
                Color::from_888(255, 255, 255),
                UI_BLACK
            );
        }
    }


    fn ray_cast(&self, camera: &Camera, world: &World, max_lenght: usize) -> Option<RaycastResult> {
        let start_pos = *camera.get_pos();
        let cam_rot = camera.get_rotation();
        let forward_vector = Vector3::new(
            cosf(cam_rot.x) * sinf(cam_rot.y),
            -sinf(cam_rot.x),
            cosf(cam_rot.x) * cosf(cam_rot.y),
        );

        let end_pos = start_pos + forward_vector.normalize() * (max_lenght as f32);

        let mut current_voxel_pos = start_pos;
        let mut step_dir = -1;

        let dx = (end_pos.x - start_pos.x).signum();
        let delta_x = if dx != 0. {
            (dx / (end_pos.x - start_pos.x)).abs().min(10000000.0) // Use abs() for delta_x
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
            (dy / (end_pos.y - start_pos.y)).abs().min(10000000.0) // Use abs() for delta_y
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
            (dz / (end_pos.z - start_pos.z)).abs().min(10000000.0) // Use abs() for delta_z
        } else {
            10000000.0
        };
        let mut max_z = if dz > 0. {
            delta_z * (1.0 - start_pos.z.fract())
        } else {
            delta_z * start_pos.z.fract()
        };

        // Raycasting loop
        while !(max_x > (max_lenght as f32) && max_y > (max_lenght as f32) && max_z > (max_lenght as f32)) { // Stop if beyond max length
            let current_voxel_pos_isize = current_voxel_pos.map(|x| x.floor() as isize); // Use floor() for precise integer coords
            let result = world.get_block_in_world(current_voxel_pos_isize);

            // If a block is found and it's not air
            if !result.is_none_or(|b| b == BlockType::Air) {
                let block_type = result.unwrap();

                // Determine the face of the block that was hit
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
                } else { // step_dir == 2
                    if dz < 0. {
                        QuadDir::Back
                    } else {
                        QuadDir::Front
                    }
                };
                return Some(RaycastResult {
                    block_pos: current_voxel_pos_isize,
                    face_dir: voxel_normal,
                    block_type,
                });
            }

            // Advance the ray to the next voxel
            if max_x < max_y {
                if max_x < max_z {
                    current_voxel_pos.x += dx;
                    max_x += delta_x;
                    step_dir = 0; // X axis
                } else {
                    current_voxel_pos.z += dz;
                    max_z += delta_z;
                    step_dir = 2; // Z axis
                }
            } else if max_y < max_z {
                current_voxel_pos.y += dy;
                max_y += delta_y;
                step_dir = 1; // Y axis
            } else {
                current_voxel_pos.z += dz;
                max_z += delta_z;
                step_dir = 2; // Z axis
            }
        }
        None // No block found within max_length
    }
}

struct RaycastResult {
    pub block_pos: Vector3<isize>,
    pub face_dir: QuadDir,
    pub block_type: BlockType,
}
