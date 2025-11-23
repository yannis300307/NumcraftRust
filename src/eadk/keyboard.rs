use enum_iterator::Sequence;

use crate::eadk::time;

/// A key of the calculator's keyboard.
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

impl Key {
    /// Match a character with the given key.
    /// If alpha is active and shift is not, you will get lowercase letter.
    /// With alpha active and shift inactive, you will get uppercase letters.
    /// With only shift active, you will get special characters.
    /// If none of alpha and shift are active, you will get digits.
    /// You should expect the same behavior as the Epsilon's Python app. 
    pub fn get_matching_char(&self, shift_active: bool, alpha_active: bool) -> Option<char> {
        if alpha_active && !shift_active {
            match *self {
                Key::Exp => Some('a'),
                Key::Ln => Some('b'),
                Key::Log => Some('c'),
                Key::Imaginary => Some('d'),
                Key::Comma => Some('e'),
                Key::Power => Some('f'),
                Key::Sine => Some('g'),
                Key::Cosine => Some('h'),
                Key::Tangent => Some('i'),
                Key::Pi => Some('j'),
                Key::Sqrt => Some('k'),
                Key::Square => Some('l'),
                Key::Seven => Some('m'),
                Key::Eight => Some('n'),
                Key::Nine => Some('o'),
                Key::LeftParenthesis => Some('p'),
                Key::RightParenthesis => Some('q'),
                Key::Four => Some('r'),
                Key::Five => Some('s'),
                Key::Six => Some('t'),
                Key::Multiplication => Some('u'),
                Key::Division => Some('v'),
                Key::One => Some('w'),
                Key::Two => Some('x'),
                Key::Three => Some('y'),
                Key::Plus => Some('z'),
                Key::Minus => Some(' '),
                Key::Zero => Some('?'),
                Key::Dot => Some('!'),
                _ => None,
            }
        } else if alpha_active && shift_active {
            match *self {
                Key::Exp => Some('A'),
                Key::Ln => Some('B'),
                Key::Log => Some('C'),
                Key::Imaginary => Some('D'),
                Key::Comma => Some('E'),
                Key::Power => Some('F'),
                Key::Sine => Some('G'),
                Key::Cosine => Some('H'),
                Key::Tangent => Some('I'),
                Key::Pi => Some('J'),
                Key::Sqrt => Some('K'),
                Key::Square => Some('L'),
                Key::Seven => Some('M'),
                Key::Eight => Some('N'),
                Key::Nine => Some('O'),
                Key::LeftParenthesis => Some('P'),
                Key::RightParenthesis => Some('Q'),
                Key::Four => Some('R'),
                Key::Five => Some('S'),
                Key::Six => Some('T'),
                Key::Multiplication => Some('U'),
                Key::Division => Some('V'),
                Key::One => Some('W'),
                Key::Two => Some('X'),
                Key::Three => Some('Y'),
                Key::Plus => Some('Z'),
                Key::Minus => Some(' '),
                Key::Zero => Some('?'),
                Key::Dot => Some('!'),
                _ => None,
            }
        } else if shift_active && !alpha_active {
            match *self {
                Key::Exp => Some('['),
                Key::Ln => Some(']'),
                Key::Log => Some('{'),
                Key::Imaginary => Some('}'),
                Key::Comma => Some('_'),
                Key::Pi => Some('='),
                Key::Sqrt => Some('<'),
                Key::Power => Some('>'),
                _ => None,
            }
        } else {
            match *self {
                Key::One => Some('1'),
                Key::Two => Some('2'),
                Key::Three => Some('3'),
                Key::Four => Some('4'),
                Key::Five => Some('5'),
                Key::Six => Some('6'),
                Key::Seven => Some('7'),
                Key::Eight => Some('8'),
                Key::Nine => Some('9'),
                Key::Zero => Some('0'),
                Key::Zero => Some(','),
                Key::LeftParenthesis => Some('('),
                Key::RightParenthesis => Some(')'),
                Key::Multiplication => Some('*'),
                Key::Division => Some('/'),
                Key::Plus => Some('+'),
                Key::Minus => Some('-'),
                Key::Dot => Some('.'),
                _ => None,
            }
        }
    }
}

/// Represent the state of the keyboard when `scan()` is called.
#[derive(Clone, Copy, Debug)]
pub struct KeyboardState(u64);

impl KeyboardState {
    /// Scan the keyboard.
    pub fn scan() -> Self {
        KeyboardState(unsafe { eadk_keyboard_scan() })
    }

    /// Return true if the given key was pressed when scan was called.
    pub fn key_down(&self, key: Key) -> bool {
        (self.0 >> (key as u8)) & 1 != 0
    }

    /// Return true if the given key was just pressed. If the key is still pressed in `self` and `old`, this function returns false.
    pub fn get_just_pressed(&self, old: KeyboardState) -> Self {
        KeyboardState(self.0 & (!old.0))
    }

    /// Return true if the given key was just released. If the key is still released in `self` and `old`, this function returns false.
    pub fn get_just_released(&self, old: KeyboardState) -> Self {
        KeyboardState((!self.0) & old.0)
    }
}

impl Default for KeyboardState {
    fn default() -> Self {
        KeyboardState(0)
    }
}

/// A higher level screen input handler.
pub struct InputManager {
    keyboard_state: KeyboardState,
    last_keyboard_state: KeyboardState,
    just_pressed: KeyboardState,
    just_released: KeyboardState,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            keyboard_state: KeyboardState::default(),
            last_keyboard_state: KeyboardState::default(),
            just_pressed: KeyboardState::default(),
            just_released: KeyboardState::default(),
        }
    }

    /// Update the InputManager.
    pub fn scan(&mut self) {
        self.last_keyboard_state = self.keyboard_state;
        self.keyboard_state = KeyboardState::scan();
        self.just_pressed = self
            .keyboard_state
            .get_just_pressed(self.last_keyboard_state);
        self.just_pressed = self
            .keyboard_state
            .get_just_released(self.last_keyboard_state);
    }

    /// Return one of the currently pressed keys.
    pub fn get_last_pressed(&self) -> Option<Key> {
        for k in enum_iterator::all::<Key>() {
            if self.is_just_pressed(k) {
                return Some(k);
            }
        }
        None
    }

    /// Return true if the given key has just been pressed.
    pub fn is_just_pressed(&self, key: Key) -> bool {
        self.just_pressed.key_down(key)
    }

    /// Return true if the given key has just been released.
    pub fn is_just_released(&self, key: Key) -> bool {
        self.just_released.key_down(key)
    }

    /// Return true if the key is currently pressed
    pub fn is_keydown(&self, key: Key) -> bool {
        self.keyboard_state.key_down(key)
    }
}

/// Wait for the given key to be pressed.
pub fn wait_until_pressed(key: Key) {
    loop {
        let scan = KeyboardState::scan();
        if scan.key_down(key) {
            break;
        }
        time::wait_milliseconds(50);
    }
}

unsafe extern "C" {
    fn eadk_keyboard_scan() -> u64;
}
