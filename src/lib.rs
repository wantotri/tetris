pub mod tetris;

use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use crate::tetris::*;

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
}

#[wasm_bindgen(js_name = gamePrint)]
pub fn game_print() -> String {
    GAME.with(|game| game.borrow().to_string())
}

#[wasm_bindgen(js_name = gamePrintNextfield)]
pub fn game_print_nextfield() -> String {
    GAME.with(|game| game.borrow().get_nextfield_str())
}

#[wasm_bindgen(js_name = gameTick)]
pub fn game_tick() {
    GAME.with(|game| game.borrow_mut().tick());
}

#[wasm_bindgen(js_name = gameScore)]
pub fn game_score() -> String {
    GAME.with(|game| game.borrow().get_score().to_string())
}

#[wasm_bindgen(js_name = gameOver)]
pub fn game_over() -> bool {
    GAME.with(|game| game.borrow().over)
}

#[wasm_bindgen(js_name = moveDown)]
pub fn move_down() {
    GAME.with(|game| {
        if game.borrow().can_move_down() {
            game.borrow_mut().move_down();
        }
    });
}

#[wasm_bindgen(js_name = moveRight)]
pub fn move_right() {
    GAME.with(|game| {
        if game.borrow().can_move_right() {
            game.borrow_mut().move_right();
        }
    });
}

#[wasm_bindgen(js_name = moveLeft)]
pub fn move_left() {
    GAME.with(|game| {
        if game.borrow().can_move_left() {
            game.borrow_mut().move_left();
        }
    });
}

#[wasm_bindgen(js_name = rotate)]
pub fn rotate() {
    GAME.with(|game| {
        if game.borrow_mut().can_rotate() {
            game.borrow_mut().rotate();
        }
    })
}
