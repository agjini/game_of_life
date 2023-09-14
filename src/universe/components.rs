use bevy::prelude::Component;
use bevy::utils::HashMap;

#[derive(Component, Copy, Clone, Debug)]
pub struct Cell {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) state: State,
}

#[derive(Copy, Clone, Debug)]
pub enum State {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    pub cells: HashMap<(i32, i32), State>,
}

impl Universe {
    pub(crate) fn snapshot<'a>(cells: impl Iterator<Item=&'a Cell>) -> Universe {
        let cells: HashMap<(i32, i32), State> = cells
            .map(|cell| ((cell.x, cell.y), cell.state))
            .collect();
        Universe { cells }
    }

    pub(crate) fn get_alive_neighbours(&self, cell: &Cell) -> u32 {
        let mut count = 0;
        for xx in [cell.x - 1, cell.x, cell.x + 1] {
            for yy in [cell.y - 1, cell.y, cell.y + 1] {
                if xx == cell.x && yy == cell.y {
                    continue;
                }

                if let Some(State::Alive) = self.cells.get(&(xx, yy)) {
                    count += 1;
                }
            }
        }
        count
    }
}