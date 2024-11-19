pub struct Window {
    window: minifb::Window
}

pub struct FrameBuffer {
    data: Vec<u32>,
    width: usize,
    height: usize
}

impl Window {
    pub fn new(name: &str, width: usize, height: usize)  -> Self {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };

        let window = minifb::Window::new(name, width, height, options).expect("Error");

        Window {
            window
        }

    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn update(&mut self) {
        self.window.update();
    }
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        FrameBuffer {
            data: vec![0; width*height],
            width,
            height
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}