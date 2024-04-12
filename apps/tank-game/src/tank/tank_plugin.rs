use bevy::prelude::*;

use crate::tank::tank_health_manager::TankHealthManager;
use crate::tank::tank_movement_manager::TankMovementManager;
use crate::tank::tank_spawn_manager::TankSpawnManager;

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, TankHealthManager::update_health_bar)
            .add_systems(Update, TankSpawnManager::despawn_tanks_with_zero_health)
            .add_systems(
                Update,
                TankMovementManager::set_tank_target_position_to_move,
            )
            .add_systems(FixedUpdate, TankMovementManager::move_tanks_towards_target);
    }
}
