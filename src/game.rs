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
        display::push_rect_uniform(
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
            if !self.update(delta_time) {
                break;
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
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

        self.player.update(
            delta_time,
            keyboard_state,
            just_pressed_keyboard_state,
            &mut self.world,
            &mut self.renderer.camera, // Pass the camera by mutable reference
        );

        self.world
            .generate_world_around_pos(*self.renderer.camera.get_pos(), RENDER_DISTANCE as isize);

        self.renderer.update(&self.world, &self.player, 1.0 / delta_time);

        eadk::display::wait_for_vblank();

        true // Continue the game loop
    }
}
