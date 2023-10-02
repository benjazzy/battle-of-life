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

impl From<battle_of_life::Cell> for WasmCell {
    fn from(value: battle_of_life::Cell) -> Self {
        match value {
            battle_of_life::Cell::Alive => WasmCell::Alive,
            battle_of_life::Cell::Dead => WasmCell::Dead,
        }
    }
}

#[wasm_bindgen(js_name = Universe)]
pub struct WasmUniverse(battle_of_life::Universe);

#[wasm_bindgen(js_class = Universe)]
impl WasmUniverse {
    pub fn new() -> WasmUniverse {
        WasmUniverse(battle_of_life::Universe::new(256, 256, |i| {
            if i % 2 == 0 || i % 7 == 0 {
                battle_of_life::Cell::Alive
            } else {
                battle_of_life::Cell::Dead
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

impl WasmUniverse {
    pub fn get_cells(&self) -> &[WasmCell] {
        use std::mem::size_of;
        // Check WasmCell and Cell are the same size. They should be because they are both u8
        // I have no idea what would happen if they are not the same size.
        assert_eq!(size_of::<WasmCell>(), size_of::<battle_of_life::Cell>());

        let n_bytes = self.0.get_cells().len() * std::mem::size_of::<battle_of_life::Cell>();
        let cells = unsafe {
            std::slice::from_raw_parts(self.0.get_cells().as_ptr() as *const WasmCell, n_bytes)
        };

        cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        self.0.set_cells(cells)
    }
}
