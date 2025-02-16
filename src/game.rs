use crate::renderer::Renderer;

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: Renderer::new(),
        }
    }

    pub fn update(&self) {
        self.renderer.update();
    }
}
