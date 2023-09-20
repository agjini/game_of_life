use bevy::prelude::Component;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Copy, Clone, Debug)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub state: State,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    Dead = 0,
    Alive = 1,
}
