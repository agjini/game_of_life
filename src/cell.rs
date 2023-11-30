use crate::cell::State::{Alive, Dead};
use bevy::prelude::{Color, Component, Reflect};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Copy, Clone, Debug, Reflect)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub state: State,
    pub live_neighbors: u8,
}

impl Cell {
    pub fn color(&self) -> Color {
        match self.state {
            Dead => Color::BLACK,
            Alive => Color::Hsla {
                hue: 122.,
                saturation: 1.,
                lightness: self.live_neighbors as f32 / 4.,
                alpha: 1.,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Reflect)]
pub enum State {
    Dead = 0,
    Alive = 1,
}

impl State {
    pub fn update(&self, live_neighbors: u8) -> Option<State> {
        match (self, live_neighbors) {
            // Rule 1: Any live cell with fewer than two live neighbours
            // dies, as if caused by underpopulation.
            (Alive, x) if x < 2 => Some(Dead),
            // Rule 2: Any live cell with two or three live neighbours
            // lives on to the next generation.
            (Alive, 2) | (Alive, 3) => None,
            // Rule 3: Any live cell with more than three live
            // neighbours dies, as if by overpopulation.
            (Alive, x) if x > 3 => Some(Dead),
            // Rule 4: Any dead cell with exactly three live neighbours
            // becomes a live cell, as if by reproduction.
            (Dead, 3) => Some(Alive),
            // All other cells remain in the same state.
            (_, _) => None,
        }
    }
}
