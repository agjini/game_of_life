use bevy::app::AppExit;
use bevy::input::Input;
use bevy::prelude::{Commands, EventWriter, KeyCode, Query, Res, ResMut};
use bevy::time::Time;

use crate::universe::components::{Cell, Universe};
use crate::universe::resources::StepTimer;

pub fn create_universe(mut commands: Commands) {
    commands.spawn((Universe {
        width: 640,
        height: 480,
        cells: vec![Cell::Dead; 64 * 64],
    }));
}

pub fn update_universe(time: Res<Time>, mut timer: ResMut<StepTimer>, query: Query<&mut Universe>) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for universe in &query {
        println!("hello {}!", universe);
    }
}

fn close_on_esc(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}