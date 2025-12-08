#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]

#[macro_use]
mod nadk;

mod camera;
mod constants;
mod entity;
mod game;
mod game_ui;
mod hud;
mod input_manager;
mod inventory;
mod menu;
mod misc;
mod physic;
mod player;
mod renderer;
mod save_manager;
mod settings;
mod timing;
mod world;

use game::Game;

setup_allocator!();

configure_app!(b"Numcraft\0", 9, "../target/assets/icon.nwi", 3437);

#[unsafe(no_mangle)]
fn main() {
    init_heap!();

    nadk::utils::wait_ok_released();

    let mut game = Game::new();

    game.main_loop();
}
