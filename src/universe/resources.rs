use std::fmt::Display;

use bevy::prelude::Resource;
use bevy::time::Timer;

#[derive(Resource)]
pub struct StepTimer(pub Timer);
