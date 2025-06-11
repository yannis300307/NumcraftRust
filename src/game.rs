use crate::{constants::rendering::RENDER_DISTANCE, eadk, player::Player, renderer::Renderer, world::World};

pub struct Game {
    renderer: Renderer,
    world: World,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: Renderer::new(),
            world: World::new(),
            player: Player::new()
        }
    }

    pub fn start(&mut self) {
        let mut last = eadk::timing::millis();

        loop {
            let current = eadk::timing::millis();
            let delta = (current - last) as f32 / 1000.0;
            last = current;
            if !self.update(delta) {
                break;
            }
        }
    }

    pub fn update(&mut self, delta: f32) -> bool {
        let keyboard_state = eadk::input::KeyboardState::scan();
        if keyboard_state.key_down(eadk::input::Key::Home) {
            return false;
        }

        self.player.update(delta, keyboard_state, &mut self.world, &mut self.renderer.camera);

        
        self.world
            .generate_world_around_pos(*self.renderer.camera.get_pos(), RENDER_DISTANCE as isize);


        self.renderer.update(&self.world, 1.0 / delta);

        //eadk::timing::msleep(20);
        true
    }
}
