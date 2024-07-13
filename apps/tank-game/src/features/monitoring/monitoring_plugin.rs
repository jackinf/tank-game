use crate::features::monitoring::resources::{PowerTimer, StrategyMonitoringTimer};
use crate::features::monitoring::systems::{
    sys_execute_current_tank_strategy, sys_monitor_power, sys_spawn_enemy_units,
};
use bevy::prelude::{App, Plugin, Resource, Timer, TimerMode, Update};

pub struct MonitoringPlugin;

impl Plugin for MonitoringPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PowerTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(StrategyMonitoringTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_systems(Update, sys_monitor_power)
            .add_systems(Update, sys_spawn_enemy_units)
            .add_systems(Update, sys_execute_current_tank_strategy);
    }
}
