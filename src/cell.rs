use bevy::prelude::{Component, Reflect};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Copy, Clone, Debug, Reflect)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub state: State,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Reflect)]
pub enum State {
    Dead = 0,
    Alive = 1,
}
