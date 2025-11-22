use core::panic::PanicInfo;

calc_use!(alloc::string::String);
calc_use!(alloc::format);

use crate::eadk::display::{Color565, ScreenPoint, ScreenRect, draw_string, push_rect_uniform};

#[cfg(target_os = "none")]
fn write_wrapped(text: &str, limit: usize) {
    let mut line_count = 0;

    let mut line = String::new();
    for i in 0..text.len() {
        line.push(text.as_bytes()[i] as char);

        if line.len() >= limit || text.as_bytes()[i] as char == '\n' || i >= text.len() - 1 {
            draw_string(
                line.as_str(),
                ScreenPoint {
                    x: 10,
                    y: (10 + 20 * line_count) as u16,
                },
                false,
                Color565::from_rgb888(0, 0, 0),
                Color565::from_rgb888(255, 0, 0),
            );
            line.clear();
            line_count += 1;
        }
    }
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    push_rect_uniform(
        ScreenRect {
            x: 0,
            y: 0,
            width: 320,
            height: 240,
        },
        Color565::from_rgb888(255, 0, 0),
    ); // Show a red screen

    write_wrapped(format!("{}", panic).as_str(), 42);

    loop {
        crate::eadk::time::wait_milliseconds(50);
    }
}
