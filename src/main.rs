#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]

#[macro_use]
mod eadk;

mod game;
mod renderer;
mod world;
mod camera;
mod constants;
mod entity;
mod game_ui;
mod hud;
mod input_manager;
mod inventory;
mod menu;
mod misc;
mod physic;
mod player;
mod save_manager;
mod settings;
mod timing;

use game::Game;

setup_allocator!();

configure_app!(b"Numcraft\0", 9, "../target/assets/icon.nwi", 3437);

#[unsafe(no_mangle)]
fn main() -> isize {
    init_heap!();

   eadk::utils::wait_ok_released();

    let mut game = Game::new();

    game.main_loop();

    0
}
