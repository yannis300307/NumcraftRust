calc_use!(alloc::ffi::CString);
calc_use!(alloc::vec::Vec);

#[cfg(not(target_os = "none"))]
use std::ffi::CString;

use core::ffi::c_char;

pub const SCREEN_RECT: ScreenRect = ScreenRect {
    x: 0,
    y: 0,
    width: 320,
    height: 240,
};
pub const COLOR_BLACK: Color565 = Color565::from_rgb888(0, 0, 0);
pub const COLOR_WHITE: Color565 = Color565::from_rgb888(255, 255, 255);
pub const COLOR_RED: Color565 = Color565::from_rgb888(255, 0, 0);
pub const COLOR_GREEN: Color565 = Color565::from_rgb888(0, 255, 0);
pub const COLOR_BLUE: Color565 = Color565::from_rgb888(0, 0, 255);

/// The color format of the screen. Encoded with 16 bits (or 2 bytes). 5 bits are used for red, 6 for green and 5 for blue.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color565 {
    pub value: u16,
}

impl Color565 {
    /// Create a new Color565 object using the RGB values. The components must be valid RGB 565 values.
    #[inline]
    pub const fn new(r: u16, g: u16, b: u16) -> Self {
        Color565 {
            value: r << 11 | g << 5 | b,
        }
    }

    /// Convert a RGB 888 (standard rgb) to RGB 565.
    #[inline]
    pub const fn from_rgb888(r: u16, g: u16, b: u16) -> Color565 {
        Color565 {
            value: ((r & 0b11111000) << 8) | ((g & 0b11111100) << 3) | (b >> 3),
        }
    }

    /// Extract the red, green and blue components from the RGB 565 color.
    #[inline]
    pub const fn get_components(&self) -> (u16, u16, u16) {
        let r = self.value >> 0xB;
        let g = (self.value & 0x7E0) >> 5;
        let b = self.value & 0x1F;

        (r, g, b)
    }
}

/// A rectangle on the screen defined by its top left corner coordinates and its size.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ScreenRect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl ScreenRect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        ScreenRect {
            x,
            y,
            width,
            height,
        }
    }
}

/// A point on the screen defined by its coordinates.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ScreenPoint {
    pub x: u16,
    pub y: u16,
}

impl ScreenPoint {
    pub fn new(x: u16, y: u16) -> Self {
        ScreenPoint { x, y }
    }
}

/// Fill the screen rect defined by `rect` with the given pixels.
pub fn push_rect(rect: ScreenRect, pixels: &[Color565]) {
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_display_push_rect(rect, pixels.as_ptr());
    }

    #[cfg(feature = "upsilon")]
    unsafe {
        extapp_pushRect(rect.x, rect.y, rect.width, rect.height, pixels.as_ptr());
    }
}

/// Fetch pixels from the given rect from the screen. The size of the returned vector will be `rect.width * rect.height`
pub fn pull_rect(rect: ScreenRect) -> Vec<Color565> {
    let size = rect.width as usize * rect.height as usize;
    let mut vec: Vec<Color565> = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(COLOR_BLACK);
    }

    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_display_pull_rect(rect, vec.as_mut_slice().as_mut_ptr());
    }

    #[cfg(feature = "upsilon")]
    unsafe {
        extapp_pullRect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            vec.as_mut_slice().as_mut_ptr(),
        );
    }
    vec
}

/// Fill the screen rect defined by `rect` with the given color.
pub fn push_rect_uniform(rect: ScreenRect, color: Color565) {
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_display_push_rect_uniform(rect, color);
    }

    #[cfg(feature = "upsilon")]
    unsafe {
        extapp_pushRectUniform(rect.x, rect.y, rect.width, rect.height, color);
    }
}

/// Wait until the screen is ed. The maximum FPS is 40 on actual hardware.
pub fn wait_for_vblank() {
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_display_wait_for_vblank();
    }

    #[cfg(feature = "upsilon")]
    unsafe {
        extapp_waitForVBlank();
    }
}

/// Draw a string to the screen.
pub fn draw_string(
    text: &str,
    point: ScreenPoint,
    large_font: bool,
    text_color: Color565,
    background_color: Color565,
) -> Option<()> {
    let c_string = CString::new(text).ok()?;
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_display_draw_string(
            c_string.as_ptr(),
            point,
            large_font,
            text_color,
            background_color,
        )
    }

    #[cfg(feature = "upsilon")]
    unsafe {
        if large_font {
            extapp_drawTextLarge(
                c_string.as_ptr(),
                point.x,
                point.y,
                text_color,
                background_color,
                false,
            )
        } else {
            extapp_drawTextSmall(
                c_string.as_ptr(),
                point.x,
                point.y,
                text_color,
                background_color,
                false,
            )
        }
    }

    Some(())
}

#[cfg(feature = "epsilon")]
unsafe extern "C" {
    fn eadk_display_push_rect_uniform(rect: ScreenRect, color: Color565);
    fn eadk_display_push_rect(rect: ScreenRect, color: *const Color565);
    fn eadk_display_wait_for_vblank();
    fn eadk_display_pull_rect(rect: ScreenRect, color: *mut Color565);
    fn eadk_display_draw_string(
        text: *const c_char,
        point: ScreenPoint,
        large_font: bool,
        text_color: Color565,
        background_color: Color565,
    );
}

unsafe extern "C" {
    fn extapp_pushRectUniform(x: u16, y: u16, w: u16, h: u16, color: Color565);
    fn extapp_pushRect(x: u16, y: u16, w: u16, h: u16, color: *const Color565);
    fn extapp_waitForVBlank() -> bool;
    fn extapp_pullRect(x: u16, y: u16, w: u16, h: u16, color: *mut Color565);
    fn extapp_drawTextLarge(
        text: *const c_char,
        x: u16,
        y: u16,
        text_color: Color565,
        background_color: Color565,
        fake: bool,
    );
    fn extapp_drawTextSmall(
        text: *const c_char,
        x: u16,
        y: u16,
        text_color: Color565,
        background_color: Color565,
        fake: bool,
    );
}
