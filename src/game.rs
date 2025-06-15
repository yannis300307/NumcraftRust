// In src/game.rs

use crate::{
    constants::{
        rendering::{RENDER_DISTANCE, SCREEN_HEIGHT, SCREEN_WIDTH},
        UI_BLACK, // Import UI_BLACK for screen clearing
    },
    eadk::{self, input::KeyboardState, display, Rect}, // Corrected: import `Rect` struct
    player::Player,
    renderer::Renderer,
    world::World,
};

pub struct Game {
    renderer: Renderer,
    world: World,
    player: Player,
    last_keyboard_state: KeyboardState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: Renderer::new(),
            world: World::new(),
            player: Player::new(),
            last_keyboard_state: KeyboardState::new(),
        }
    }

    pub fn start(&mut self) {
        // We no longer create a `Display` instance. Drawing functions are called directly via `eadk::display::...`
        // Initial scene clear (e.g., loading screen, initial black screen)
        display::push_rect_uniform( // Corrected: use `display::push_rect_uniform`
            Rect {
                x: 0,
                y: 0,
                width: SCREEN_WIDTH as u16,
                height: SCREEN_HEIGHT as u16,
            },
            UI_BLACK,
        );

        let mut last_frame_time_millis = eadk::timing::millis();

        // Main game loop
        loop {
            let current_time_millis = eadk::timing::millis();
            let delta_time = (current_time_millis - last_frame_time_millis) as f32 / 1000.0; // Delta in seconds
            last_frame_time_millis = current_time_millis;

            // Call the update function; if it returns false, break the loop (e.g., Home key pressed)
            // The `update` function no longer receives a `display` parameter
            if !self.update(delta_time) {
                break;
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool { // Corrected: removed `display` parameter
        // Input processing
        let keyboard_state = eadk::input::KeyboardState::scan();
        // Get keys that were just pressed since the last frame
        let just_pressed_keyboard_state = keyboard_state.get_just_pressed(self.last_keyboard_state);
        // Store the current keyboard state for the next frame
        self.last_keyboard_state = keyboard_state;

        // Check for exit condition (Home key)
        if keyboard_state.key_down(eadk::input::Key::Home) {
            return false; // Signal to exit the game loop
        }

        // --- Game Logic Update ---
        // Update player state, including movement, inventory interaction, and block placement/breaking.
        // The player's update method will also update the camera's position and rotation within the renderer.
        self.player.update(
            delta_time,
            keyboard_state,
            just_pressed_keyboard_state,
            &mut self.world,
            &mut self.renderer.camera, // Pass the camera by mutable reference
        );

        // Regenerate or load world chunks around the player's position
        self.world
            .generate_world_around_pos(*self.renderer.camera.get_pos(), RENDER_DISTANCE as isize);

        // Update the renderer (this typically involves drawing the 3D world)
        // You might need to adjust the `1.0 / delta_time` if your renderer expects FPS directly.
        self.renderer.update(&self.world, &self.player, 1.0 / delta_time);

        // --- UI Rendering ---
        // Render the player's inventory (hotbar) on top of the 3D world
        self.player.inventory.draw(); // No longer passing display instance

        // Render any temporary debug messages on top of the UI
        self.player.draw_debug_message(); // No longer passing display instance

        // Wait for vertical blanking to synchronize with the display refresh rate
        eadk::display::wait_for_vblank(); // `eadk::display` module is still correctly used here

        true // Continue the game loop
    }
}
