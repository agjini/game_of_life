use bevy::prelude::Resource;
use rand::Rng;

use crate::cell::State::{Alive, Dead};
use crate::cell::{Cell, State};

pub const UNIVERSE_SIZE: usize = 100;

#[derive(Clone)]
pub struct CellBuffer {
    cells: [State; UNIVERSE_SIZE * UNIVERSE_SIZE],
}

#[derive(Resource)]
pub struct Universe {
    buffer1: CellBuffer,
    buffer2: CellBuffer,
    use_buffer1: bool,
}

fn get_index(x: usize, y: usize) -> usize {
    y * UNIVERSE_SIZE + x
}

fn reverse_index(i: usize) -> (usize, usize) {
    (i % UNIVERSE_SIZE, i / UNIVERSE_SIZE)
}

impl CellBuffer {
    fn new() -> Self {
        Self {
            cells: [Dead; (UNIVERSE_SIZE * UNIVERSE_SIZE)],
        }
    }

    fn with_entropy() -> Self {
        let mut buffer = Self::new();
        let mut rng = rand::thread_rng();
        for x in 0..UNIVERSE_SIZE {
            for y in 0..UNIVERSE_SIZE {
                let alive = rng.gen_ratio(7, 60);
                if alive {
                    buffer.set(x, y, Alive);
                };
            }
        }
        buffer
    }

    pub fn iter(&self) -> impl Iterator<Item = Cell> + '_ {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, &state)| {
                let (x, y) = reverse_index(i);
                let live_neighbors = self.get_alive_neighbours(x, y);
                Cell {
                    x,
                    y,
                    state,
                    live_neighbors,
                }
            })
            .filter(|c| c.state == Alive || c.live_neighbors > 0)
    }

    fn get(&self, x: usize, y: usize) -> State {
        let index = get_index(x, y);
        if index >= self.cells.len() {
            return Dead;
        }
        self.cells[index]
    }

    fn set(&mut self, x: usize, y: usize, state: State) {
        self.cells[get_index(x, y)] = state;
    }

    fn get_alive_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for xx in x.saturating_sub(1)..x + 1 {
            for yy in y.saturating_sub(1)..y + 1 {
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
}

impl Universe {
    pub fn with_entropy() -> Universe {
        let buffer1 = CellBuffer::with_entropy();
        let buffer2 = buffer1.clone();
        Universe {
            buffer1,
            buffer2,
            use_buffer1: true,
        }
    }

    pub fn update_life(&mut self) {
        let (read, write) = self.current_buffers();
        for cell in read.iter() {
            if let Some(update) = cell.state.update(cell.live_neighbors) {
                write.set(cell.x, cell.y, update);
            }
        }
        self.use_buffer1 = !self.use_buffer1;
    }

    pub fn cells(&self) -> &CellBuffer {
        if self.use_buffer1 {
            &self.buffer1
        } else {
            &self.buffer2
        }
    }

    pub fn set(&mut self, x: usize, y: usize, state: State) {
        let (_, write) = self.current_buffers();
        write.set(x, y, state);
    }

    fn current_buffers(&mut self) -> (&CellBuffer, &mut CellBuffer) {
        if self.use_buffer1 {
            (&self.buffer1, &mut self.buffer2)
        } else {
            (&self.buffer2, &mut self.buffer1)
        }
    }
}
