#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub rgb565: u16,
}

pub const COLOR_BLACK: Color = Color::from_888(0, 0, 0);

impl Color {
    #[inline]
    pub const fn from_components(r: u16, g: u16, b: u16) -> Self {
        Color {
            rgb565: r << 11 | g << 5 | b,
        }
    }
    pub const fn from_888(r: u16, g: u16, b: u16) -> Self {
        Color {
            rgb565: ((r & 0b11111000) << 8) | ((g & 0b11111100) << 3) | (b >> 3),
        }
    }

    pub fn apply_light(&self, light_level: u8) -> Self {
        let light_level = light_level as u16;
        let rgb = self.get_components();
        Color::from_components(
            rgb.0 * light_level / 255,
            rgb.1 * light_level / 255,
            rgb.2 * light_level / 255,
        )
    }

    pub const fn get_components(&self) -> (u16, u16, u16) {
        let r = self.rgb565 >> 11;
        let g = (self.rgb565 & 0b0000011111100000) >> 5;
        let b = self.rgb565 & 0b0000000000011111;

        (r, g, b)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub const SCREEN_RECT: Rect = Rect {
    x: 0,
    y: 0,
    width: 320,
    height: 240,
};

#[repr(C)]
#[derive(Clone, Copy)]
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

    unsafe extern "C" {
        fn eadk_backlight_set_brightness(brightness: u8);
        fn eadk_backlight_brightness() -> u8;
    }
}

pub mod display {
    use super::Color;
    use super::Point;
    use super::Rect;

    #[cfg(target_os = "none")]
    use alloc::vec::Vec;

    #[cfg(target_os = "none")]
    use alloc::ffi::CString;

    #[cfg(not(target_os = "none"))]
    use std::ffi::CString;

    use core::ffi::c_char;

    pub fn push_rect(rect: Rect, pixels: &[Color]) {
        unsafe {
            eadk_display_push_rect(rect, pixels.as_ptr());
        }
    }

    pub fn pull_rect(rect: Rect) -> Vec<Color> {
        let size = rect.width as usize * rect.height as usize;
        let mut vec: Vec<Color> = Vec::with_capacity(size);
        for _ in 0..size {
            use crate::eadk::COLOR_BLACK;

            vec.push(COLOR_BLACK);
        }

        unsafe {
            eadk_display_pull_rect(rect, vec.as_mut_slice().as_mut_ptr());
        }

        vec
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
        let c_string =
            CString::new(text).expect("Can't convert str to C_String. Maybe invalid caracter.");
        unsafe {
            eadk_display_draw_string(
                c_string.as_ptr(),
                point,
                large_font,
                text_color,
                background_color,
            )
        }
    }

    unsafe extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
        fn eadk_display_push_rect(rect: Rect, color: *const Color);
        fn eadk_display_wait_for_vblank();
        fn eadk_display_pull_rect(rect: Rect, color: *mut Color);
        fn eadk_display_draw_string(
            text: *const c_char,
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

    unsafe extern "C" {
        fn eadk_timing_usleep(us: u32);
        fn eadk_timing_msleep(us: u32);
        fn eadk_timing_millis() -> u64;
    }
}

pub fn random() -> u32 {
    unsafe { eadk_random() }
}

unsafe extern "C" {
    fn eadk_random() -> u32;
}

pub mod input {
    use enum_iterator::Sequence;

    type EadkKeyboardState = u64;

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq, Sequence, Debug)]
    #[repr(u8)]
    pub enum Key {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Home = 6,
        OnOff = 8,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LeftParenthesis = 33,
        RightParenthesis = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Multiplication = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52,
    }

    unsafe extern "C" {
        fn eadk_keyboard_scan() -> EadkKeyboardState;
    }

    #[derive(Clone, Copy)]
    pub struct KeyboardState(EadkKeyboardState);

    impl Default for KeyboardState {
        fn default() -> Self {
            Self::new()
        }
    }

    impl KeyboardState {
        pub fn scan() -> Self {
            Self::from_raw(unsafe { eadk_keyboard_scan() })
        }

        pub fn new() -> Self {
            KeyboardState(0)
        }

        pub fn from_raw(state: EadkKeyboardState) -> Self {
            Self(state)
        }

        pub fn key_down(&self, key: Key) -> bool {
            (self.0 >> (key as u8)) & 1 != 0
        }

        pub fn get_just_pressed(&self, old: KeyboardState) -> Self {
            KeyboardState(self.0 & (!old.0))
        }

        pub fn get_just_realeased(&self, old: KeyboardState) -> Self {
            KeyboardState((!self.0) & old.0)
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u16)]
    pub enum Event {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LeftParenthesis = 33,
        RightParenthesis = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Multiplication = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52,
        ShiftLeft = 54,
        ShiftUp = 55,
        ShiftDown = 56,
        ShiftRight = 57,
        AlphaLock = 67,
        Cut = 68,
        Copy = 69,
        Paste = 70,
        Clear = 71,
        LeftBracket = 72,
        RightBracket = 73,
        LeftBrace = 74,
        RightBrace = 75,
        Underscore = 76,
        Sto = 77,
        Arcsine = 78,
        Arccosine = 79,
        Arctangent = 80,
        Equal = 81,
        Lower = 82,
        Greater = 83,
        Colon = 122,
        Semicolon = 123,
        DoubleQuotes = 124,
        Percent = 125,
        LowerA = 126,
        LowerB = 127,
        LowerC = 128,
        LowerD = 129,
        LowerE = 130,
        LowerF = 131,
        LowerG = 132,
        LowerH = 133,
        LowerI = 134,
        LowerJ = 135,
        LowerK = 136,
        LowerL = 137,
        LowerM = 138,
        LowerN = 139,
        LowerO = 140,
        LowerP = 141,
        LowerQ = 142,
        LowerR = 144,
        LowerS = 145,
        LowerT = 146,
        LowerU = 147,
        LowerV = 148,
        LowerW = 150,
        LowerX = 151,
        LowerY = 152,
        LowerZ = 153,
        Space = 154,
        Question = 156,
        Exclamation = 157,
        UpperA = 180,
        UpperB = 181,
        UpperC = 182,
        UpperD = 183,
        UpperE = 184,
        UpperF = 185,
        UpperG = 186,
        UpperH = 187,
        UpperI = 188,
        UpperJ = 189,
        UpperK = 190,
        UpperL = 191,
        UpperM = 192,
        UpperN = 193,
        UpperO = 194,
        UpperP = 195,
        UpperQ = 196,
        UpperR = 198,
        UpperS = 199,
        UpperT = 200,
        UpperU = 201,
        UpperV = 202,
        UpperW = 204,
        UpperX = 205,
        UpperY = 206,
        UpperZ = 207,
    }

    impl Event {
        pub fn is_digit(&self) -> bool {
            matches!(
                self,
                Event::Zero
                    | Event::One
                    | Event::Two
                    | Event::Three
                    | Event::Four
                    | Event::Five
                    | Event::Six
                    | Event::Seven
                    | Event::Eight
                    | Event::Nine
            )
        }

        pub fn to_digit(&self) -> Option<u8> {
            match self {
                Event::Zero => Some(0),
                Event::One => Some(1),
                Event::Two => Some(2),
                Event::Three => Some(3),
                Event::Four => Some(4),
                Event::Five => Some(5),
                Event::Six => Some(6),
                Event::Seven => Some(7),
                Event::Eight => Some(8),
                Event::Nine => Some(9),
                _ => None,
            }
        }
    }

    unsafe extern "C" {
        fn eadk_event_get(timeout: &i32) -> Event;
    }

    pub fn event_get(timeout: i32) -> Event {
        unsafe { eadk_event_get(&timeout) }
    }
}

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
use alloc::string::String;

#[cfg(target_os = "none")]
fn write_wrapped(text: &str, limit: usize) {
    let mut line_count = 0;

    let mut line = String::new();
    for i in 0..text.len() {
        line.push(text.as_bytes()[i] as char);

        if line.len() >= limit || text.as_bytes()[i] as char == '\n' || i >= text.len() - 1 {
            display::draw_string(
                line.as_str(),
                Point {
                    x: 10,
                    y: (10 + 20 * line_count) as u16,
                },
                false,
                Color { rgb565: 65503 },
                Color { rgb565: 63488 },
            );
            line.clear();
            line_count += 1;
        }
    }
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    use alloc::format;

    display::push_rect_uniform(
        Rect {
            x: 0,
            y: 0,
            width: 320,
            height: 240,
        },
        Color { rgb565: 63488 },
    ); // Show a red screen

    write_wrapped(format!("{}", panic).as_str(), 42);

    loop {
        timing::msleep(50);
    } // FIXME: Do something better. Exit the app maybe?
}

pub fn debug_info(text: &str, wait: usize) {
    display::draw_string(
        text,
        Point { x: 10, y: 30 },
        false,
        Color::from_888(0, 0, 0),
        Color::from_888(255, 255, 255),
    );
    timing::msleep(wait as u32);
}

unsafe extern "C" {
    pub static mut _heap_start: u8;
    pub static mut _heap_end: u8;
}

pub static mut HEAP_START: *mut u8 = core::ptr::addr_of_mut!(_heap_start);
pub static mut HEAP_END: *mut u8 = core::ptr::addr_of_mut!(_heap_end);

pub fn heap_size() -> usize {
    (unsafe { HEAP_END.offset_from(HEAP_START) }) as usize
}
