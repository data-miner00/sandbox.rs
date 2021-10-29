#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub fn tick(&mut self) {
    let mut next = self.cells.clone();

    for row in 0..self.height {
        for col in 0..self.width {
            let idx = self.get_index(row, col);
            let cell = self.cells[idx];
            let live_neighbors = self.live_neighbor_count(row, col);

            let next_cell = match (cell, live_neighbors) {
                (Cell::Alive, x) if x < 2 => Cell::Dead,
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                (Cell::Alive, x) if x > 3 => Cell::Dead,
                (Cell::Dead, 3) => Cell::Alive,
                (otherwise, _) => otherwise,
            };

            next[idx] = next_cell;
        }
    }
    self.cells = next;
}