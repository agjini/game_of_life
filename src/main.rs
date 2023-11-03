use bevy::prelude::*;

use crate::plugin::GameOfLifePlugin;

mod cell;
mod plugin;
mod system;
mod timer;
mod universe;

#[cfg(test)]
mod test;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameOfLifePlugin))
        .run();
}
