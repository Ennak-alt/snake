#[cfg(feature = "buddy-alloc")]
mod alloc;
mod snake;
mod snake_game;
mod utils;
mod wasm4;
use crate::snake_game::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm4::*;

lazy_static! {
    static ref SNAKE_GAME: Mutex<SnakeGame> = Mutex::new(SnakeGame::new());
}

#[no_mangle]
fn update() {
    SNAKE_GAME.lock().expect("game state").update();
}
