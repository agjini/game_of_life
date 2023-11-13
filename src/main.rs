use bevy::prelude::*;
use game_of_life::GameOfLifePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameOfLifePlugin))
        .run();
}
