use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct TankUngroupTimer(pub Timer);
