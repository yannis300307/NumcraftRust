use core::f32::consts::PI;

use libm::sincosf;
use nalgebra::Vector3;

use crate::{camera::Camera, constants::{player::MOVEMENT_SPEED, world, BlockType}, eadk, world::World};

pub struct Player {
    pub pos: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl Player {
    pub fn new() -> Self {
        Player { pos: Vector3::new(0., 0., 0.), rotation: Vector3::new(0., 0., 0.) }
    }
    pub fn update(&mut self, delta: f32, keyboard_state: eadk::input::KeyboardState, world: &mut World, camera: &mut Camera) {
        camera.update(delta, keyboard_state, self.pos-Vector3::new(0., 1.70, 0.));
        self.rotation = *camera.get_rotation();
        
        // Movements
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

        if keyboard_state.key_down(eadk::input::Key::Ok) {
            // Break Block
            if let Some(block_pos) = self.raycast(camera, world, 5) {
                world.set_block_in_world(block_pos, BlockType::Air);
            }
        }
    }

    pub fn raycast(&self, camera: &Camera, world: &World, max_lenght: usize) -> Option<Vector3<isize>> {
        let target: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
        let look_dir = camera.get_rotation_matrix() * target.to_homogeneous();
        let target = camera.get_pos() + look_dir.xyz();

        let mut current_pos: Vector3<f32> = *camera.get_pos();

        let forward_step = target.normalize()*0.1;

        for _ in 0..max_lenght*10 {
            current_pos += forward_step;

            let block_pos = current_pos.map(|x| x as isize);

            if world.get_block_in_world(block_pos).is_some_and(|b| b != BlockType::Air) {
                return Some(block_pos)
            }
        }
        None
    }
}