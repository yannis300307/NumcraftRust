#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]

#[allow(unused_imports)]
#[cfg(target_os = "none")]
use cortex_m;

#[cfg(target_os = "none")]
use eadk::adresses::heap_size;
#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
#[cfg(target_os = "none")]
static HEAP: Heap = Heap::empty();

#[cfg(target_os = "none")]
extern crate alloc;

mod camera;
pub mod constants;
pub mod eadk;
mod game;
mod renderer;
mod world;
use game::Game;

mod entity;
mod game_ui;
mod hud;
mod input_manager;
mod inventory;
mod menu;
pub mod misc;
mod physic;
mod player;
mod save_manager;
mod settings;
mod timing;

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_app_name")]
pub static EADK_APP_NAME: [u8; 9] = *b"Numcraft\0";

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_api_level")]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_app_icon")]
pub static EADK_APP_ICON: [u8; 3437] = *include_bytes!("../target/assets/icon.nwi");

#[unsafe(no_mangle)]
fn main() -> isize {
    // Init the heap
    #[cfg(target_os = "none")]
    {
        let heap_size: usize = heap_size();
        unsafe { HEAP.init(eadk::adresses::HEAP_START as usize, heap_size) }
    }

    while eadk::keyboard::KeyboardState::scan().key_down(eadk::keyboard::Key::Ok) {
        // Avoid instant click on Ok
        eadk::time::wait_milliseconds(50);
    }

    let mut game = Game::new();

    game.main_loop();

    0
}
