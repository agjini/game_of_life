use bevy::prelude::Resource;

use crate::cell::{Cell, State};
use crate::cell_buffer::CellBuffer;

#[derive(Resource)]
pub struct Universe {
    buffer1: CellBuffer,
    buffer2: CellBuffer,
    use_buffer1: bool,
}

impl Universe {
    pub fn with_entropy(size: usize) -> Universe {
        let buffer1 = CellBuffer::with_entropy(size);
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
            let state = cell.state.update(cell.live_neighbors);
            write.set(cell.x, cell.y, state);
        }
        self.use_buffer1 = !self.use_buffer1;
    }

    pub fn iter(&self) -> impl Iterator<Item = Cell> + '_ {
        if self.use_buffer1 {
            self.buffer1.iter()
        } else {
            self.buffer2.iter()
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

#[cfg(test)]
impl Universe {
    pub fn from_string(map: &str) -> Self {
        let buffer1 = CellBuffer::from_string(map);
        let buffer2 = buffer1.clone();
        Universe {
            buffer1,
            buffer2,
            use_buffer1: true,
        }
    }
}
