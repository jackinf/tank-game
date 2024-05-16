use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct HarvesterTimer(pub Timer);
