use crate::features::tank::event_handlers::{
    spawn_harvester_event_handler, spawn_tank_event_handler,
};
use crate::features::tank::events::{SpawnHarvesterEvent, SpawnTankEvent};
use crate::features::tank::resources::{TankMonitoringTimer, TankUngroupTimer};
use crate::features::tank::systems::{
    monitor_for_enemies, move_bullets, move_tanks_towards_target, periodic_shooting,
    set_tank_target_position_to_move, ungroup_tanks, update_health_bar,
};
use bevy::prelude::*;

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankMonitoringTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(TankUngroupTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_systems(Update, update_health_bar)
        .add_systems(Update, set_tank_target_position_to_move)
        .add_systems(FixedUpdate, move_tanks_towards_target)
        .add_systems(FixedUpdate, move_bullets)
        .add_systems(Update, periodic_shooting)
        .add_systems(Update, monitor_for_enemies)
        .add_systems(Update, ungroup_tanks)
        .add_event::<SpawnTankEvent>()
        .add_event::<SpawnHarvesterEvent>()
        .add_systems(Update, spawn_tank_event_handler)
        .add_systems(Update, spawn_harvester_event_handler);
    }
}
