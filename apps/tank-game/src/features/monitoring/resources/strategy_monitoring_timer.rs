use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct StrategyMonitoringTimer(pub Timer);
