use crate::monitoring::managers::power_monitoring_manager::PowerMonitoringManager;
use bevy::prelude::{App, Plugin, Resource, Timer, TimerMode, Update};

#[derive(Resource)]
pub struct PowerTimer(pub Timer);

pub struct MonitoringPlugin;

impl Plugin for MonitoringPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PowerTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, PowerMonitoringManager::monitor_power);
    }
}
