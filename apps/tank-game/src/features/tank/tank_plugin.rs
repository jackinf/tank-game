use crate::features::tank::resources::{TankMonitoringTimer, TankUngroupTimer};
use crate::features::tank::systems::{
    despawn_tanks_with_zero_health, monitor_for_enemies, move_bullets, move_tanks_towards_target,
    periodic_shooting, set_tank_target_position_to_move, ungroup_tanks, update_health_bar,
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
        .add_systems(Update, despawn_tanks_with_zero_health)
        .add_systems(Update, set_tank_target_position_to_move)
        .add_systems(FixedUpdate, move_tanks_towards_target)
        .add_systems(FixedUpdate, move_bullets)
        .add_systems(Update, periodic_shooting)
        .add_systems(Update, monitor_for_enemies)
        .add_systems(Update, ungroup_tanks);
    }
}
