use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::Input;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Camera, Camera2dBundle, Color, Commands, GlobalTransform, KeyCode, MouseButton, Query, Res, ResMut, Transform, With};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::time::Time;
use bevy::utils::default;
use bevy::window::{PrimaryWindow, Window};
use rand::Rng;

use crate::cell::{Cell, MainCamera, State};
use crate::timer::StepTimer;
use crate::universe::{Universe, UNIVERSE_SIZE};

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
        MainCamera,
        BloomSettings {
            intensity: 0.3,
            ..default()
        },
    ));
    commands.insert_resource(Universe::new());
    let mut rng = rand::thread_rng();
    for x in 0..UNIVERSE_SIZE {
        for y in 0..UNIVERSE_SIZE {
            let state = if rng.gen_ratio(7, 60) {
                State::Alive
            } else {
                State::Dead
            };
            commands
                .spawn(SpriteBundle {
                    transform: from_cell_to_world(x, y),
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
                    x,
                    y,
                    state,
                });
        }
    }
}

pub fn click_on_cell(camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
                     input: Res<Input<MouseButton>>,
                     window: Query<&Window, With<PrimaryWindow>>,
                     mut query: Query<&mut Cell>) {
    if !input.pressed(MouseButton::Left) {
        return;
    }

    if let Some(world_position) = get_click_world_position(camera_q, window) {
        let (cell_x, cell_y) = from_world_to_cell(world_position);
        for mut cell in &mut query {
            if cell.x == cell_x && cell.y == cell_y {
                cell.state = State::Alive;
            }
        }
    }
}

fn get_click_world_position(camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>, window: Query<&Window, With<PrimaryWindow>>) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    window.single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}

fn from_world_to_cell(world_position: Vec2) -> (i32, i32) {
    (from_world_to_cell_coordinate(world_position.x), from_world_to_cell_coordinate(world_position.y))
}

fn from_world_to_cell_coordinate(c: f32) -> i32 {
    ((c + (UNIVERSE_SIZE as f32 / 2. * (CELL_SIZE + GAP))) / (CELL_SIZE + GAP)).floor() as i32
}

fn from_cell_to_world(x: i32, y: i32) -> Transform {
    Transform::from_xyz(
        from_cell_to_world_coordinate(x),
        from_cell_to_world_coordinate(y),
        0.,
    )
}

fn from_cell_to_world_coordinate(x: i32) -> f32 {
    (x as f32) * (CELL_SIZE + GAP) - (UNIVERSE_SIZE as f32 / 2. * (CELL_SIZE + GAP))
}

pub fn entropy(input: Res<Input<KeyCode>>, mut query: Query<&mut Cell>) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let mut rng = rand::thread_rng();
    for mut cell in &mut query {
        cell.state = if rng.gen_ratio(7, 60) {
            State::Alive
        } else {
            State::Dead
        };
    }
}

pub fn update_cells(time: Res<Time>,
                    mut universe: ResMut<Universe>,
                    mut timer: ResMut<StepTimer>,
                    mut query: Query<(&mut Cell, &mut Sprite)>,
) {
    let time_to_change = timer.0.tick(time.delta()).just_finished();
    universe.snapshot(query.iter().map(|(cell, _)| cell));
    for (mut cell, mut sprite) in &mut query {
        let live_neighbors = universe.get_alive_neighbours(&cell);
        if time_to_change {
            cell.state = update_state(&cell, live_neighbors);
        }
        sprite.color = update_color(&cell, live_neighbors);
    }
}

fn update_color(cell: &Cell, live_neighbors: u32) -> Color {
    match cell.state {
        State::Dead => Color::BLACK,
        State::Alive => Color::Hsla {
            hue: 122.,
            saturation: 1.,
            lightness: live_neighbors as f32 / 4.,
            alpha: 1.,
        }
    }
}

fn update_state(cell: &Cell, live_neighbors: u32) -> State {
    match (cell.state, live_neighbors) {
        // Rule 1: Any live cell with fewer than two live neighbours
        // dies, as if caused by underpopulation.
        (State::Alive, x) if x < 2 => State::Dead,
        // Rule 2: Any live cell with two or three live neighbours
        // lives on to the next generation.
        (State::Alive, 2) | (State::Alive, 3) => cell.state,
        // Rule 3: Any live cell with more than three live
        // neighbours dies, as if by overpopulation.
        (State::Alive, x) if x > 3 => State::Dead,
        // Rule 4: Any dead cell with exactly three live neighbours
        // becomes a live cell, as if by reproduction.
        (State::Dead, 3) => State::Alive,
        // All other cells remain in the same state.
        (_, _) => cell.state,
    }
}