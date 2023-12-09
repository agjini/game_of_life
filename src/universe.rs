use bevy::prelude::Resource;
use itertools::Itertools;
use rand::Rng;

use crate::cell::State::{Alive, Dead};
use crate::cell::{Cell, State};

#[derive(Resource)]
pub struct Universe {
    size: usize,
    cells: Vec<State>,
}

impl Universe {
    fn with_size(size: usize) -> Self {
        Self {
            size,
            cells: vec![Dead; size * size],
        }
    }

    pub fn with_entropy(size: usize) -> Self {
        let mut buffer = Self::with_size(size);
        let mut rng = rand::thread_rng();
        for x in 0..size {
            for y in 0..size {
                let alive = rng.gen_ratio(7, 60);
                if alive {
                    buffer.set(x, y, Alive);
                };
            }
        }
        buffer
    }

    pub fn iter(&self) -> impl Iterator<Item = Cell> + '_ {
        self.cells.iter().enumerate().map(|(i, &state)| {
            let (x, y) = self.reverse_index(i);
            let live_neighbors = self.get_alive_neighbours(x, y);
            Cell {
                x,
                y,
                state,
                live_neighbors,
            }
        })
    }

    fn get(&self, x: usize, y: usize) -> State {
        let index = self.get_index(x, y);
        if index >= self.cells.len() {
            Dead
        } else {
            self.cells[index]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, state: State) {
        let index = self.get_index(x, y);
        if index >= self.cells.len() {
            return;
        }
        self.cells[index] = state;
    }

    fn get_alive_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for yy in y.saturating_sub(1)..=(y + 1) {
            for xx in x.saturating_sub(1)..=(x + 1) {
                if xx == x && yy == y {
                    continue;
                }

                if let Alive = self.get(xx, yy) {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    fn reverse_index(&self, i: usize) -> (usize, usize) {
        (i % self.size, i / self.size)
    }

    pub fn update_life(&mut self) {
        let cells = self
            .iter()
            .filter_map(|cell| {
                cell.state
                    .update(cell.live_neighbors)
                    .map(|state| Cell { state, ..cell })
            })
            .collect_vec();
        for cell in cells {
            self.set(cell.x, cell.y, cell.state);
        }
    }
}

#[cfg(test)]
impl Universe {
    pub fn from_string(map: &str) -> Self {
        let mut size = 0;
        let mut rows: Vec<Vec<State>> = vec![];
        for line in map.split("\\s+") {
            let row = line
                .bytes()
                .map(|b| if b == b'*' { Alive } else { Dead })
                .collect_vec();
            size = std::cmp::max(size, row.len());
            rows.push(row);
        }
        size = std::cmp::max(size, rows.len());
        let mut cells: Vec<State> = Vec::with_capacity(size * size);
        for y in 0..size {
            let row = rows.get(y);
            for _ in 0..size {
                let state = row.and_then(|r| r.get(y)).unwrap_or(&Dead);
                cells.push(*state);
            }
        }
        Self { size, cells }
    }
}
