use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::Input;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Camera, Camera2dBundle, Changed, Color, Commands, KeyCode, MouseButton, Query, Res, ResMut, Transform, With};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::time::Time;
use bevy::utils::default;
use bevy::window::{PrimaryWindow, Window};
use rand::Rng;

use crate::universe::components::{Cell, State, Universe};
use crate::universe::resources::StepTimer;

const UNIVERSE_SIZE: u32 = 100;
const CELL_SIZE: f32 = 8.;
const GAP: f32 = 0.;

pub fn create_universe(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings {
            intensity: 0.25,
            ..default()
        },
    ));
    let mut rng = rand::thread_rng();
    for x in 0..UNIVERSE_SIZE {
        for y in 0..UNIVERSE_SIZE {
            let state = if rng.gen_ratio(3, 60) {
                State::Alive
            } else {
                State::Dead
            };
            commands
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(
                        (x as f32) * (CELL_SIZE + GAP) - (UNIVERSE_SIZE as f32 / 2. * (CELL_SIZE + GAP)),
                        (y as f32) * (CELL_SIZE + GAP) - (UNIVERSE_SIZE as f32 / 2. * (CELL_SIZE + GAP)),
                        0.,
                    ),
                    sprite: Sprite {
                        color: Color::BLACK,
                        flip_x: false,
                        flip_y: false,
                        custom_size: None,
                        rect: Some(Rect {
                            min: Vec2::new(0., 0.),
                            max: Vec2::new(CELL_SIZE, CELL_SIZE),
                        }),
                        anchor: Default::default(),
                    },
                    ..default()
                })
                .insert(Cell {
                    x: x as i32,
                    y: y as i32,
                    state,
                });
        }
    }
}

pub fn click_on_cell(input: Res<Input<MouseButton>>,
                     q_windows: Query<&Window, With<PrimaryWindow>>,
                     mut query: Query<&mut Cell>) {
    if !input.pressed(MouseButton::Left) {
        return;
    }
    let position = q_windows.single().cursor_position();
    if position.is_none() {
        return;
    }
    // get cell under mouser position
    let cell_x = ((position.unwrap().x + (UNIVERSE_SIZE as f32 / 2. * (CELL_SIZE + GAP))) / (CELL_SIZE + GAP)) as i32 / CELL_SIZE as i32;
    let cell_y = ((position.unwrap().y + (UNIVERSE_SIZE as f32 / 2. * (CELL_SIZE + GAP))) / (CELL_SIZE + GAP)) as i32 / CELL_SIZE as i32;
    for mut cell in &mut query {
        if cell.x == cell_x && cell.y == cell_y {
            cell.state = State::Alive;
        }
    }
}

pub fn entropy(input: Res<Input<KeyCode>>, mut query: Query<&mut Cell>) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    
    let mut rng = rand::thread_rng();
    for mut cell in &mut query {
        cell.state = if rng.gen_ratio(3, 60) {
            State::Alive
        } else {
            State::Dead
        };
    }
}

pub fn update_cells(time: Res<Time>, mut timer: ResMut<StepTimer>, mut query: Query<&mut Cell>) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let universe = Universe::snapshot(query.iter());
    for mut cell in &mut query {
        let live_neighbors = universe.get_alive_neighbours(&cell);
        // if live_neighbors > 0 {
        //     println!("{},{} has {} live neighbours", cell.x, cell.y, live_neighbors);
        // }
        match (cell.state, live_neighbors) {
            // Rule 1: Any live cell with fewer than two live neighbours
            // dies, as if caused by underpopulation.
            (State::Alive, x) if x < 2 => cell.state = State::Dead,
            // Rule 2: Any live cell with two or three live neighbours
            // lives on to the next generation.
            (State::Alive, 2) | (State::Alive, 3) => {}
            // Rule 3: Any live cell with more than three live
            // neighbours dies, as if by overpopulation.
            (State::Alive, x) if x > 3 => cell.state = State::Dead,
            // Rule 4: Any dead cell with exactly three live neighbours
            // becomes a live cell, as if by reproduction.
            (State::Dead, 3) => cell.state = State::Alive,
            // All other cells remain in the same state.
            (_, _) => {}
        };
    }
}

pub fn update_colors(mut query: Query<(&mut Sprite, &Cell), Changed<Cell>>) {
    for (mut sprite, cell) in &mut query {
        if let State::Alive = cell.state {
            sprite.color = Color::PURPLE;
        } else {
            sprite.color = Color::BLACK;
        }
    }
}
