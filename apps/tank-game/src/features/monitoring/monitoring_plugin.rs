use crate::features::monitoring::resources::PowerTimer;
use crate::features::monitoring::systems::monitor_power;
use bevy::prelude::{App, Plugin, Resource, Timer, TimerMode, Update};

pub struct MonitoringPlugin;

impl Plugin for MonitoringPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PowerTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, monitor_power);
    }
}
