use crate::eadk::{display, keyboard, time};
calc_use!(alloc::string::String);
calc_use!(alloc::vec::Vec);

/// Wait until Ok is released.
/// This function should be added at the beginning of the program because your it could handle the "Ok" that started the app. 
pub fn wait_ok_released() {
    while keyboard::KeyboardState::scan().key_down(keyboard::Key::Ok) {
        time::wait_milliseconds(50);
    }
}

/// Write debugging code to the screen.
pub fn log(text: &[&str]) {
    for i in 0..text.len() {
        display::draw_string(
            text[i],
            display::ScreenPoint::new(5, 5 + 14 * i as u16 as u16),
            false,
            display::Color565::from_rgb888(255, 255, 255),
            display::Color565::from_rgb888(0, 0, 0),
        );
    }
}

/// Refresh the simulator screen and prevent it from craching.
/// Only use this function if you DON'T use the keyboard. The keyboard scan will refresh the simulator automatically.
#[inline(always)]
pub fn refresh_simulator() {
    #[cfg(not(target_os = "none"))]
    crate::eadk::keyboard::KeyboardState::scan();
}
