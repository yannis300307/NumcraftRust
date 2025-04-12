#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub rgb565: u16,
}

fn to_rgb888(color: &Color) -> usize {
    let r = ((color.rgb565 >> 11) as usize) << 3;
    let g = (((color.rgb565 >> 5) & 0b111111) as usize) << 2;
    let b = ((color.rgb565 & 0b11111) as usize) << 3;
    (r << 16) | (g << 8) | b
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

#[cfg(target_os = "none")]
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

#[cfg(not(target_os = "none"))]
pub mod backlight {
    pub fn set_brightness(brightness: u8) {
        println!("Brightness set to {}", brightness)
    }
    pub fn brightness() -> u8 {
        0
    }
}

pub mod display {
    use super::Color;
    use super::Point;
    use super::Rect;

    #[cfg(target_os = "none")]
    pub fn push_rect(rect: Rect, pixels: &[Color]) {
        unsafe {
            eadk_display_push_rect(rect, pixels.as_ptr());
        }
    }

    #[cfg(target_os = "windows")]
    pub fn push_rect(rect: Rect, pixels: &[Color]) {
        todo!("implement push_rect")
        //use super::WindowOperation;
        //let mut global_tx = super::WINDOW_MANAGER_TX.lock().unwrap();
        //global_tx.as_mut().unwrap().send(WindowOperation::PushRect(rect, *pixels));
    }

    #[cfg(target_os = "none")]
    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    #[cfg(target_os = "windows")]
    pub fn push_rect_uniform(rect: Rect, color: Color) {
        use super::WindowOperation;
        let mut global_tx = super::WINDOW_MANAGER_TX.lock().unwrap();
        global_tx
            .as_mut()
            .unwrap()
            .send(WindowOperation::PushRectUniform(rect, color));
    }

    #[cfg(target_os = "none")]
    pub fn wait_for_vblank() {
        unsafe {
            eadk_display_wait_for_vblank();
        }
    }

    #[cfg(target_os = "windows")]
    pub fn wait_for_vblank() {}

    #[cfg(target_os = "none")]
    pub fn draw_string(
        text: &str,
        point: Point,
        large_font: bool,
        text_color: Color,
        background_color: Color,
    ) {
        let mut buf = [0u8; 256]; // MAXIMUM is 256 chars
        let s = format_no_std::show(&mut buf, format_args!("{}\0", text)).unwrap();
        unsafe {
            eadk_display_draw_string(s.as_ptr(), point, large_font, text_color, background_color)
        }
    }

    #[cfg(target_os = "windows")]
    pub fn draw_string(
        text: &str,
        _point: Point,
        _large_font: bool,
        _text_color: Color,
        _background_color: Color,
    ) {
        println!("{}", text);
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
    #[cfg(target_os = "none")]
    pub fn usleep(us: u32) {
        unsafe {
            eadk_timing_usleep(us);
        }
    }

    #[cfg(target_os = "windows")]
    pub fn usleep(us: u32) {
        use std::{thread, time};

        thread::sleep(time::Duration::from_micros(us as u64));
    }

    #[cfg(target_os = "none")]
    pub fn msleep(ms: u32) {
        unsafe {
            eadk_timing_msleep(ms);
        }
    }

    #[cfg(target_os = "windows")]
    pub fn msleep(ms: u32) {
        use std::{thread, time};

        thread::sleep(time::Duration::from_millis(ms as u64));
    }

    #[cfg(target_os = "none")]
    pub fn millis() -> u64 {
        unsafe { eadk_timing_millis() }
    }

    #[cfg(target_os = "windows")]
    pub fn millis() -> u64 {
        use std::time::Instant;
        Instant::now().elapsed().as_millis() as u64
    }

    #[cfg(target_os = "none")]
    extern "C" {
        fn eadk_timing_usleep(us: u32);
        fn eadk_timing_msleep(us: u32);
        fn eadk_timing_millis() -> u64;
    }
}

#[cfg(target_os = "none")]
pub fn random() -> u32 {
    unsafe { eadk_random() }
}

#[cfg(target_os = "windows")]
pub fn random() -> u32 {
    rand::random_range(0..u32::MAX)
}

#[cfg(target_os = "none")]
extern "C" {
    fn eadk_random() -> u32;
}

pub mod input {
    type EadkKeyboardState = u64;

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
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

    #[cfg(target_os = "none")]
    extern "C" {
        fn eadk_keyboard_scan() -> EadkKeyboardState;
    }

    #[derive(Clone, Copy)]
    pub struct KeyboardState(EadkKeyboardState);

    impl KeyboardState {
        #[cfg(target_os = "none")]
        pub fn scan() -> Self {
            Self::from_raw(unsafe { eadk_keyboard_scan() })
        }

        #[cfg(target_os = "windows")]
        pub fn scan() -> Self {
            Self::from_raw(0) // TODO : change this
        }

        pub fn from_raw(state: EadkKeyboardState) -> Self {
            Self(state)
        }

        pub fn key_down(&self, key: Key) -> bool {
            (self.0 >> (key as u8)) & 1 != 0
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

    extern "C" {
        fn eadk_event_get(timeout: &i32) -> Event;
    }

    pub fn event_get(timeout: i32) -> Event {
        unsafe { eadk_event_get(&timeout) }
    }
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
            display::draw_string(
                tt,
                Point {
                    x: 10,
                    y: (10 + 20 * i) as u16,
                },
                false,
                Color { rgb565: 65503 },
                Color { rgb565: 63488 },
            );
        }
    }
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    display::push_rect_uniform(
        Rect {
            x: 0,
            y: 0,
            width: 320,
            height: 240,
        },
        Color { rgb565: 63488 },
    ); // Show a red screen

    if let Some(message) = _panic.message().as_str() {
        if let Some(loc) = _panic.location() {
            let mut buf = [0u8; 512];
            let f = format_no_std::show(
                &mut buf,
                format_args!(
                    "Error occured at {} line {} col {} : {}",
                    loc.file(),
                    loc.line(),
                    loc.column(),
                    message
                ),
            )
            .unwrap();

            write_wrapped(f, 42);
        } else {
            write_wrapped(message, 42);
        };
    } else {
        display::draw_string(
            "Unknow Error Occured",
            Point { x: 10, y: 10 },
            false,
            Color { rgb565: 65503 },
            Color { rgb565: 63488 },
        );
    }
    loop {} // FIXME: Do something better. Exit the app maybe?
}

#[cfg(target_os = "windows")]
enum WindowOperation<'a> {
    PushRectUniform(Rect, Color),
    PushRect(Rect, &'a [Color]),
}

#[cfg(target_os = "windows")]
static WINDOW_MANAGER_TX: std::sync::Mutex<Option<std::sync::mpsc::Sender<WindowOperation>>> =
    std::sync::Mutex::new(None);

#[cfg(target_os = "windows")]
pub fn init_window() {
    use minifb::{Key, Window, WindowOptions};
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    let mut global_tx = WINDOW_MANAGER_TX.lock().unwrap();
    *global_tx = Some(tx.clone());

    thread::spawn(move || {
        let mut buffer: Vec<u32> = vec![0; 320 * 240];

        let mut window = Window::new("Test - ESC to exit", 320, 240, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        window.set_target_fps(300);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            while true {
                let message = rx.try_recv();
                if message.is_err() {
                    break;
                }
                match message.unwrap() {
                    WindowOperation::PushRectUniform(rect, color) => {
                        for x in (rect.x as usize)..((rect.x as usize) + (rect.width as usize)) {
                            for y in (rect.y as usize)..((rect.y as usize) + (rect.height as usize))
                            {
                                if x > 0 && x < 320 && y > 0 && y < 240 {
                                    buffer[(x as usize) + (y as usize) * 320] =
                                        to_rgb888(&color) as u32;
                                }
                            }
                        }
                    }
                    WindowOperation::PushRect(_, _) => {
                        println!("push")
                    }
                }
            }

            window.update_with_buffer(&buffer, 320, 240).unwrap();
        }
    });
}

#[cfg(target_os = "windows")]
pub fn debug<T: std::fmt::Debug>(item: &T) {
    println!("{:?}", item)
}

#[cfg(target_os = "none")]
pub fn debug<T: core::fmt::Debug>(item: &T) {
    // TODO
}
