use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::plugin::GameOfLifePlugin;

mod universe;
mod cell;
mod system;
mod plugin;
mod timer;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GameOfLifePlugin,
            WorldInspectorPlugin::new().run_if(on_key_press),
        ))
        .run();
}

fn on_key_press(input: Res<Input<KeyCode>>) -> bool {
    input.pressed(KeyCode::F12)
}