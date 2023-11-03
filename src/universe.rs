use bevy::prelude::Resource;

use crate::cell::{Cell, State};

pub const UNIVERSE_SIZE: i32 = 1;

#[derive(Resource)]
pub struct Universe {
    pub cells: Vec<State>,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            cells: vec![State::Dead; (UNIVERSE_SIZE * UNIVERSE_SIZE) as usize],
        }
    }

    pub fn snapshot<'a>(&mut self, cells: impl Iterator<Item = &'a Cell>) {
        cells.for_each(|cell| {
            let index = self.get_index(cell.x, cell.y);
            self.cells[index] = cell.state;
        });
    }

    pub fn get_alive_neighbours(&self, cell: &Cell) -> u32 {
        let mut count = 0;
        for xx in [cell.x - 1, cell.x, cell.x + 1] {
            for yy in [cell.y - 1, cell.y, cell.y + 1] {
                if xx == cell.x && yy == cell.y {
                    continue;
                }

                if State::Alive.eq(self.get(xx, yy)) {
                    count += 1;
                }
            }
        }
        count
    }

    fn get(&self, x: i32, y: i32) -> &State {
        self.cells.get(self.get_index(x, y)).unwrap_or(&State::Dead)
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (y * UNIVERSE_SIZE + x) as usize
    }
}
