use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::Input;
use bevy::math::{Rect, Vec2};
use bevy::prelude::MouseButton::Left;
use bevy::prelude::{
    Camera, Camera2dBundle, Commands, DetectChanges, Entity, GlobalTransform, KeyCode, MouseButton,
    Query, Res, ResMut, Touches, Transform, With,
};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::time::Time;
use bevy::utils::default;
use bevy::window::{PrimaryWindow, Window};
use itertools::Itertools;
use rand::Rng;

use crate::cell::State::{Alive, Dead};
use crate::cell::{Cell, MainCamera};
use crate::timer::StepTimer;
use crate::universe::Universe;

const UNIVERSE_SIZE: usize = 100;
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
    commands.insert_resource(Universe::with_entropy(UNIVERSE_SIZE));
}

pub fn render_cells(
    mut commands: Commands,
    universe: Res<Universe>,
    cells: Query<Entity, With<Sprite>>,
) {
    if !universe.is_changed() {
        return;
    }
    for entity in cells.iter() {
        commands.entity(entity).despawn();
    }
    for cell in universe.iter() {
        commands.spawn(SpriteBundle {
            transform: from_cell_to_world(cell.x, cell.y),
            sprite: Sprite {
                color: cell.color(),
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
        });
    }
}

pub fn click_on_cell(
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    touches: Res<Touches>,
    mouse: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut universe: ResMut<Universe>,
) {
    let (camera, camera_transform) = camera.single();
    for (x, y) in get_spawn_positions(
        touches.as_ref(),
        &mouse,
        window.single(),
        camera,
        camera_transform,
    ) {
        universe.set(x, y, Alive);
    }
}

fn get_spawn_positions<'a>(
    touches: &'a Touches,
    mouse: &'a Input<MouseButton>,
    window: &'a Window,
    camera: &'a Camera,
    camera_transform: &'a GlobalTransform,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    touches
        .iter()
        .map(|f| f.position())
        .chain(get_click_position(mouse, window.cursor_position()))
        .flat_map(|p| from_window_to_world_position(p, camera, camera_transform))
        .map(from_world_to_cell)
        .dedup()
}

fn get_click_position(mouse: &Input<MouseButton>, cursor_position: Option<Vec2>) -> Option<Vec2> {
    if !mouse.pressed(Left) {
        None
    } else {
        cursor_position
    }
}

fn from_window_to_world_position(
    position: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    camera
        .viewport_to_world(camera_transform, position)
        .map(|ray| ray.origin.truncate())
}

fn from_world_to_cell(world_position: Vec2) -> (usize, usize) {
    (
        from_world_to_cell_coordinate(world_position.x),
        from_world_to_cell_coordinate(world_position.y),
    )
}

fn from_world_to_cell_coordinate(c: f32) -> usize {
    ((c + (UNIVERSE_SIZE as f32 / 2. * (CELL_SIZE + GAP))) / (CELL_SIZE + GAP)).floor() as usize
}

fn from_cell_to_world(x: usize, y: usize) -> Transform {
    Transform::from_xyz(
        from_cell_to_world_coordinate(x),
        from_cell_to_world_coordinate(y),
        0.,
    )
}

fn from_cell_to_world_coordinate(x: usize) -> f32 {
    (x as f32) * (CELL_SIZE + GAP) - (UNIVERSE_SIZE as f32 / 2. * (CELL_SIZE + GAP))
}

pub fn entropy(input: Res<Input<KeyCode>>, mut query: Query<&mut Cell>) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let mut rng = rand::thread_rng();
    for mut cell in &mut query {
        cell.state = if rng.gen_ratio(7, 60) { Alive } else { Dead };
    }
}

pub fn update_cells(time: Res<Time>, mut universe: ResMut<Universe>, mut timer: ResMut<StepTimer>) {
    let time_to_change = timer.0.tick(time.delta()).just_finished();

    if !time_to_change {
        return;
    }

    universe.update_life();
}
