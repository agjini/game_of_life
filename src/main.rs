use bevy::prelude::*;

use universe::plugin::GameOfLifePlugin;

pub mod universe;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameOfLifePlugin))
        .run();
}
