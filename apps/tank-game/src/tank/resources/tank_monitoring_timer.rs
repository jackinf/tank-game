use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct TankMonitoringTimer(pub Timer);
