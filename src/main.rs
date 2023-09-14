use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use universe::plugin::GameOfLifePlugin;

pub mod universe;
mod test;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameOfLifePlugin, WorldInspectorPlugin::new()))
        .run();
}
