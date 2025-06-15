use core::f32::consts::PI;

use libm::{cosf, sincosf, sinf};
use nalgebra::{ComplexField, Vector3};

use crate::{
    camera::Camera,
    constants::{
        player::MOVEMENT_SPEED, BlockType,
        rendering::SCREEN_WIDTH,
        UI_BLACK,
        QuadDir,
    },
    eadk::{self, Color, Point, Rect, display},
    inventory::Inventory,
    mesh::{Mesh, Quad},
    world::{get_chunk_local_coords, get_chunk_pos_from_block, World},
};

pub struct Player {
    pub pos: Vector3<f32>,
    pub rotation: Vector3<f32>,
    ray_cast_result: Option<RaycastResult>,
    pub inventory: Inventory,
    debug_message: Option<(&'static str, u64)>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
            ray_cast_result: None,
            inventory: Inventory::new(),
            debug_message: None,
        }
    }

    pub fn get_block_marker(&self) -> (Mesh, Vector3<isize>) {
        let mut mesh = Mesh::new();

        if let Some(result) = &self.ray_cast_result {
            mesh.quads.push(Quad::new(
                get_chunk_local_coords(result.block_pos).map(|x| x as u16),
                result.face_dir,
                255,
                0,
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
        camera.update(delta, keyboard_state, self.pos - Vector3::new(0., 1.70, 0.));
        self.rotation = *camera.get_rotation();

        self.ray_cast_result = self.ray_cast(camera, world, 10);

        if keyboard_state.key_down(eadk::input::Key::Toolbox) {
            let translation = sincosf(self.rotation.y);
            self.pos.x += translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z += translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Comma) {
            let translation = sincosf(self.rotation.y);
            self.pos.x -= translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z -= translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Imaginary) {
            let translation = sincosf(self.rotation.y + PI / 2.0);
            self.pos.x -= translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z -= translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Power) {
            let translation = sincosf(self.rotation.y + PI / 2.0);
            self.pos.x += translation.0 * delta * MOVEMENT_SPEED;
            self.pos.z += translation.1 * delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Shift) {
            self.pos.y -= delta * MOVEMENT_SPEED;
        }
        if keyboard_state.key_down(eadk::input::Key::Exp) {
            self.pos.y += delta * MOVEMENT_SPEED;
        }

        if just_pressed_keyboard_state.key_down(eadk::input::Key::Plus) {
            self.inventory.select_next_slot();
        }
        if just_pressed_keyboard_state.key_down(eadk::input::Key::Minus) {
            self.inventory.select_previous_slot();
        }

        if just_pressed_keyboard_state.key_down(eadk::input::Key::Back) {
            if let Some(result) = &self.ray_cast_result {
                if result.block_type != BlockType::Air {
                    world.set_block_in_world(result.block_pos, BlockType::Air);
                }
            }
        }

        if just_pressed_keyboard_state.key_down(eadk::input::Key::Ok) {
            if let Some(result) = &self.ray_cast_result {
                let block_to_place = self.inventory.get_selected_block_type();

                if block_to_place != BlockType::Air {
                    let block_pos = result.block_pos + result.face_dir.get_normal_vector();

                    if world.get_block_in_world(block_pos).is_some_and(|b| b.is_air()) {
                        let player_bbox_min = self.pos - Vector3::new(0.4, 0.0, 0.4);
                        let player_bbox_max = self.pos + Vector3::new(0.4, 1.8, 0.4);

                        let block_collides_with_player =
                            ((block_pos.x as f32) + 1.0 > player_bbox_min.x && (block_pos.x as f32) < player_bbox_max.x) &&
                            ((block_pos.y as f32) + 1.0 > player_bbox_min.y && (block_pos.y as f32) < player_bbox_max.y) &&
                            ((block_pos.z as f32) + 1.0 > player_bbox_min.z && (block_pos.z as f32) < player_bbox_max.z);

                        if !block_collides_with_player {
                            world.set_block_in_world(block_pos, block_to_place);
                        } else {
                            self.set_debug_message("Cannot place block inside player!", 1500);
                        }
                    }
                } else {
                    self.set_debug_message("Selected slot is empty, cannot place!", 1500);
                }
            }
        }

        if let Some((_, end_time)) = self.debug_message {
            if eadk::timing::millis() >= end_time {
                self.debug_message = None;
            }
        }
    }

    pub fn set_debug_message(&mut self, message: &'static str, duration_ms: u32) {
        self.debug_message = Some((message, eadk::timing::millis() + duration_ms as u64));
    }

    pub fn draw_debug_message(&self) {
        if let Some((message, _)) = self.debug_message {
            let text_x = (SCREEN_WIDTH as u16 / 2) - (message.len() as u16 * 3);
            let text_y = 5;

            display::push_rect_uniform(
                Rect { x: text_x - 5, y: text_y - 2, width: (message.len() * 6 + 10) as u16, height: 12 },
                UI_BLACK
            );

            display::draw_string(
                message,
                Point { x: text_x, y: text_y },
                false,
                Color::from_components(255, 255, 255),
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
            (dx / (end_pos.x - start_pos.x)).abs().min(10000000.0)
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
            (dy / (end_pos.y - start_pos.y)).abs().min(10000000.0)
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
            (dz / (end_pos.z - start_pos.z)).abs().min(10000000.0)
        } else {
            10000000.0
        };
        let mut max_z = if dz > 0. {
            delta_z * (1.0 - start_pos.z.fract())
        } else {
            delta_z * start_pos.z.fract()
        };

        while !(max_x > (max_lenght as f32) && max_y > (max_lenght as f32) && max_z > (max_lenght as f32)) {
            let current_voxel_pos_isize = current_voxel_pos.map(|x| x.floor() as isize);
            let result = world.get_block_in_world(current_voxel_pos_isize);

            if !result.is_none_or(|b| b == BlockType::Air) {
                let block_type = result.unwrap();

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
                } else {
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
                step_dir = 2;
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
