#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[allow(unused_imports)]
#[cfg(target_os = "none")]
use cortex_m;

use eadk::heap_size;
#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
#[cfg(target_os = "none")]
static HEAP: Heap = Heap::empty();

#[cfg(target_os = "none")]
extern crate alloc;

mod camera;
mod chunk;
pub mod constants;
pub mod eadk;
mod game;
pub mod mesh;
mod renderer;
mod world;
use game::Game;

mod frustum;
mod menu;
mod player;
mod storage_lib;
mod storage_manager;

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
pub static EADK_APP_ICON: [u8; 3437] = *include_bytes!("../target/icon.nwi");

#[unsafe(no_mangle)]
fn main() -> isize {
    // Init the heap
    #[cfg(target_os = "none")]
    {
        let heap_size: usize = heap_size();
        unsafe { HEAP.init(eadk::HEAP_START as usize, heap_size) }
    }

    while eadk::input::KeyboardState::scan().key_down(eadk::input::Key::Ok) {
        // Avoid instant click on Ok
        eadk::timing::msleep(50);
    }

    let mut game = Game::new();

    game.main_loop();

    0
}
