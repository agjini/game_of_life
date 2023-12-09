use bevy::prelude::Resource;
use rand::Rng;

use crate::cell::State::Alive;
use crate::cell::{Cell, State};
use crate::cell_buffer::CellBuffer;

#[derive(Resource)]
pub struct Universe {
    cells: CellBuffer,
    buffer: CellBuffer,
}

impl Universe {
    fn with_size(size: usize) -> Self {
        Self {
            cells: CellBuffer::with_size(size),
            buffer: CellBuffer::with_size(size),
        }
    }

    pub fn with_entropy(size: usize) -> Self {
        let mut universe = Self::with_size(size);
        let mut rng = rand::thread_rng();
        for x in 0..size {
            for y in 0..size {
                let alive = rng.gen_ratio(7, 60);
                if alive {
                    universe.set(x, y, Alive);
                };
            }
        }
        universe
    }

    pub fn iter(&self) -> impl Iterator<Item = Cell> + '_ {
        self.cells.iter()
    }

    pub fn set(&mut self, x: usize, y: usize, state: State) {
        self.cells.set(x, y, state);
    }

    pub fn update_life(&mut self) {
        self.cells.snapshot(&mut self.buffer);
        for cell in self.buffer.iter().filter_map(|cell| {
            cell.state
                .update(cell.live_neighbors)
                .map(|state| Cell { state, ..cell })
        }) {
            self.cells.set(cell.x, cell.y, cell.state);
        }
    }
}

#[cfg(test)]
impl Universe {
    pub fn from_string(map: &str) -> Self {
        let cells = CellBuffer::from_string(map);
        let buffer = cells.clone();
        Self { cells, buffer }
    }
}
