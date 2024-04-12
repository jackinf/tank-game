use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct TankLogTimer(pub Timer);
