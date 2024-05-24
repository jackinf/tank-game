use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct PowerTimer(pub Timer);
