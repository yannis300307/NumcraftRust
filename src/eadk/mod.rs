#![allow(unused)]
#[macro_use]
/// A crate containing useful macros to write cleaner code.
pub mod macros;

/// Provides abstraction to the display.
pub mod display;

/// Provides abstraction to the backlight.
pub mod backlight;

/// Provides abstraction to the timers and some additional functions.
pub mod time;

/// Generates random numbers with the built-in random generator of the OS. Also provide some higher level functions.
pub mod random;

/// Provides abstraction to the keyboard. Also contains a high level input manager for game engines. 
pub mod keyboard;

/// A crate to handle the program's panics.
pub mod panic_handler;

/// Provide addresses to low level stuff.
pub mod adresses;

// Provide high level access to the OS' storage. Contains bindings for Yaya.cout's storage.c lib. Also works on simulator.
pub mod storage;

/// Provide some random useful functions.
pub mod utils;
