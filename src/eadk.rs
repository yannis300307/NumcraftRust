#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub rgb565: u16,
}

#[repr(C)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[repr(C)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub mod backlight {
    pub fn set_brightness(brightness: u8) {
        unsafe {
            eadk_backlight_set_brightness(brightness);
        }
    }
    pub fn brightness() -> u8 {
        unsafe { eadk_backlight_brightness() }
    }

    extern "C" {
        fn eadk_backlight_set_brightness(brightness: u8);
        fn eadk_backlight_brightness() -> u8;
    }
}

pub mod display {
    use super::Color;
    use super::Point;
    use super::Rect;

    pub fn push_rect(rect: Rect, pixels: &[Color]) {
        unsafe {
            eadk_display_push_rect(rect, pixels.as_ptr());
        }
    }

    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    pub fn wait_for_vblank() {
        unsafe {
            eadk_display_wait_for_vblank();
        }
    }

    pub fn draw_string(
        text: &str,
        point: Point,
        large_font: bool,
        text_color: Color,
        background_color: Color,
    ) {
        let mut buf = [0u8; 256]; // MAXIMUM is 256 chars
        let s = format_no_std::show(
            &mut buf,
            format_args!("{}\0", text),
        ).unwrap();
        unsafe { eadk_display_draw_string(s.as_ptr(), point, large_font, text_color, background_color) }
    }

    extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
        fn eadk_display_push_rect(rect: Rect, color: *const Color);
        fn eadk_display_wait_for_vblank();
        fn eadk_display_draw_string(
            text: *const u8,
            point: Point,
            large_font: bool,
            text_color: Color,
            background_color: Color,
        );
    }
}

pub mod timing {
    pub fn usleep(us: u32) {
        unsafe {
            eadk_timing_usleep(us);
        }
    }

    pub fn msleep(ms: u32) {
        unsafe {
            eadk_timing_msleep(ms);
        }
    }

    pub fn millis() -> u64 {
        unsafe { eadk_timing_millis() }
    }

    extern "C" {
        fn eadk_timing_usleep(us: u32);
        fn eadk_timing_msleep(us: u32);
        fn eadk_timing_millis() -> u64;
    }
}

pub fn random() -> u32 {
    unsafe { eadk_random() }
}

extern "C" {
    fn eadk_random() -> u32;
}

use core::panic::PanicInfo;

use core::cmp::min;

fn write_wrapped(text: &str, limit: usize) {
    let lines = (text.len() + limit - 1) / limit; // Calcul du nombre de lignes

    if !text.is_empty() {
        for i in 0..lines {
            let start = i * limit;
            let end = min(start + limit, text.len());
            let tt = &text[start..end];
            display::draw_string(tt, Point { x: 10, y: (10 + 20 * i) as u16 }, false, Color { rgb565: 65503 }, Color { rgb565: 63488 });
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    display::push_rect_uniform(Rect{x: 0, y: 0, width: 340, height: 240}, Color {rgb565: 63488}); // Show a red screen

    if let Some(message) = _panic.message().as_str() {
        if let Some(loc) = _panic.location() {
            let mut buf = [0u8; 512];
            let f = format_no_std::show(
                &mut buf,
                format_args!("Error occured at {} line {} col {} : {}", loc.file(), loc.line(), loc.column(), message),
            ).unwrap();

            write_wrapped(f, 42);
        } else {
            write_wrapped(message, 42);
        };
    } else {
        display::draw_string("Unknow Error Occured", Point {x: 10, y: 10}, false, Color { rgb565: 65503 }, Color { rgb565: 63488 });
    }
    loop {} // FIXME: Do something better. Exit the app maybe?
}
