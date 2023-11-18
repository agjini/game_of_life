mod cell;
mod system;
mod timer;
mod universe;

#[cfg(test)]
mod test;

use crate::cell::Cell;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::input::Input;
use bevy::prelude::{ClearColor, Color, KeyCode, Res, ResMut, Resource};
use bevy::time::{Timer, TimerMode};
use bevy::window::close_on_esc;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::system::{click_on_cell, create_universe, entropy, update_cells};
use crate::timer::StepTimer;

pub struct GameOfLifePlugin;

#[derive(Resource, Eq, PartialEq, Default)]
struct Settings {
    display: bool,
}

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(
        //     WorldInspectorPlugin::new().run_if(|settings: Res<Settings>| settings.display),
        // )
        app.insert_resource(Settings::default())
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(StepTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
            .add_systems(Startup, create_universe)
            .add_systems(Update, (update_cells, click_on_cell, entropy))
            .add_systems(Update, (toggle_settings, close_on_esc))
            .register_type::<Cell>();
    }
}

fn toggle_settings(mut settings: ResMut<Settings>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::F12) {
        settings.display = !settings.display;
    }
}
