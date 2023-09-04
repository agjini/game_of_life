use bevy::app::{App, Plugin, Startup, Update};
use bevy::time::{Timer, TimerMode};
use bevy::window::close_on_esc;

use crate::universe::resources::StepTimer;
use crate::universe::systems::{create_universe, update_universe};

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Startup, create_universe)
            .add_systems(Update, (close_on_esc, update_universe));
    }
}