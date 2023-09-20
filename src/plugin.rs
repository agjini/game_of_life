use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{ClearColor, Color};
use bevy::time::{Timer, TimerMode};
use bevy::window::close_on_esc;

use crate::system::{click_on_cell, create_universe, entropy, update_cells};
use crate::timer::StepTimer;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(StepTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
            .add_systems(Startup, create_universe)
            .add_systems(Update, update_cells)
            .add_systems(Update, (click_on_cell, entropy))
            .add_systems(Update, close_on_esc);
    }
}