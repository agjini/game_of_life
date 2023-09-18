use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use universe::plugin::GameOfLifePlugin;

mod test;
pub mod universe;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GameOfLifePlugin, /*, WorldInspectorPlugin::new()*/
        ))
        .run();
}
