use bevy::prelude::*;

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
            GameOfLifePlugin, /*, WorldInspectorPlugin::new()*/
        ))
        .run();
}
