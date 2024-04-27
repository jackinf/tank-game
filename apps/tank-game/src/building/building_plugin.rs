use crate::building::managers::building_spawn_manager::BuildingSpawnManager;
use bevy::app::{App, FixedUpdate};
use bevy::prelude::Plugin;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, BuildingSpawnManager::draw_construction_tiles);
    }
}
