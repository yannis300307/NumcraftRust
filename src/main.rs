#![cfg_attr(target_os = "none", no_std)]

#![no_main]

#[cfg_attr(target_os = "none", global_allocator)]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

extern crate alloc;

pub mod eadk;
mod game;
mod camera;
mod renderer;
use game::Game;

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 10] = *b"HelloRust\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

#[no_mangle]
fn main() {
    let mut game = Game::new();

    #[cfg(target_os = "windows")]
    eadk::init_window();

    game.start();
}
