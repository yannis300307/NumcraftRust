#![cfg_attr(target_os = "none", no_std)]
#![no_main]


#[allow(unused_imports)]
#[cfg(target_os = "none")]
use cortex_m;

#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
#[cfg(target_os = "none")]
static HEAP: Heap = Heap::empty();

#[cfg(target_os = "none")]
extern crate alloc;

pub mod eadk;
pub mod constants;
pub mod mesh;
mod camera;
mod chunk;
mod game;
mod renderer;
mod world;
use game::Game;

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
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");


#[unsafe(no_mangle)]
fn main() {
    #[cfg(target_os = "none")]
    {
        const HEAP_SIZE: usize = 100000;
        unsafe { HEAP.init(eadk::HEAP_START as usize, HEAP_SIZE) }
    }

    let mut game = Game::new();

    #[cfg(not(target_os = "none"))]
    eadk::init_window();

    game.start();
}
