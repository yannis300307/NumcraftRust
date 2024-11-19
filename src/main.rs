mod window;
use window::Window;

fn main() {
    let mut window = Window::new("Numcraft", 512, 512);

    while !window.should_close() {
        window.update();
    }
}
