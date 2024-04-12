use crate::tank::managers::tank_health_manager::TankHealthManager;
use crate::tank::managers::tank_movement_manager::TankMovementManager;
use crate::tank::managers::tank_shooting_manager::TankShootingManager;
use crate::tank::managers::tank_spawn_manager::TankSpawnManager;
use bevy::prelude::*;

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, TankHealthManager::update_health_bar)
            .add_systems(Update, TankSpawnManager::despawn_tanks_with_zero_health)
            .add_systems(
                Update,
                TankMovementManager::set_tank_target_position_to_move,
            )
            .add_systems(FixedUpdate, TankMovementManager::move_tanks_towards_target)
            .add_systems(FixedUpdate, TankShootingManager::move_bullets)
            .add_systems(Update, TankShootingManager::periodic_shooting);
    }
}
