use crate::features::tank::managers::tank_health_manager::TankHealthManager;
use crate::features::tank::managers::tank_movement_manager::TankMovementManager;
use crate::features::tank::managers::tank_shooting_manager::TankShootingManager;
use crate::features::tank::managers::tank_spawn_manager::TankSpawnManager;
use crate::features::tank::resources::tank_monitoring_timer::TankMonitoringTimer;
use bevy::prelude::*;

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankMonitoringTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, TankHealthManager::update_health_bar)
        .add_systems(Update, TankSpawnManager::despawn_tanks_with_zero_health)
        .add_systems(
            Update,
            TankMovementManager::set_tank_target_position_to_move,
        )
        .add_systems(FixedUpdate, TankMovementManager::move_tanks_towards_target)
        .add_systems(FixedUpdate, TankShootingManager::move_bullets)
        .add_systems(Update, TankShootingManager::periodic_shooting)
        .add_systems(Update, TankShootingManager::monitor_for_enemies);
    }
}