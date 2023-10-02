mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Cell)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WasmCell {
    Dead = 0,
    Alive = 1,
}

impl From<rusty_game_of_life::Cell> for WasmCell {
    fn from(value: rusty_game_of_life::Cell) -> Self {
        match value {
            rusty_game_of_life::Cell::Alive => WasmCell::Alive,
            rusty_game_of_life::Cell::Dead => WasmCell::Dead,
        }
    }
}

#[wasm_bindgen(js_name = Universe)]
pub struct WasmUniverse(rusty_game_of_life::Universe);

#[wasm_bindgen(js_class = Universe)]
impl WasmUniverse {
    pub fn new() -> WasmUniverse {
        WasmUniverse(rusty_game_of_life::Universe::new(256, 256, |i| {
            if i % 2 == 0 || i % 7 == 0 {
                rusty_game_of_life::Cell::Alive
            } else {
                rusty_game_of_life::Cell::Dead
            }
        }))
    }

    pub fn tick(&mut self) {
        self.0.tick();
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }

    pub fn height(&self) -> u32 {
        self.0.height()
    }

    pub fn set_width(&mut self, width: u32) {
        self.0.set_width(width);
    }

    pub fn set_height(&mut self, height: u32) {
        self.0.set_height(height);
    }

    pub fn cells(&self) -> *const WasmCell {
        self.0.cells() as *const WasmCell
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        self.0.toggle_cell(row, column);
    }
}
