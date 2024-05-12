use crate::common::constants::TileCoord;
use crate::harvester::managers::harvester_spawn_manager::HarvesterSpawnManager;
use crate::preparation::load_mission::UnitsLayer;
use crate::tank::managers::tank_spawn_manager::TankSpawnManager;
use crate::unit::resources::unit_id_counter::UnitIdCounter;
use crate::unit::unit_tile::{UnitTile, UnitTileType};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Res, ResMut};

pub struct UnitSpawnManager;

impl UnitSpawnManager {
    pub fn spawn_units(
        mut commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        mut unit_id_counter: &mut ResMut<UnitIdCounter>,
        layer: UnitsLayer,
        calculate_world_position: fn(&TileCoord) -> Vec2,
    ) {
        layer.get_units().into_iter().for_each(|(coord, unit)| {
            let pos = calculate_world_position(coord);

            match unit.get_unit_type() {
                UnitTileType::Tank => {
                    TankSpawnManager::spawn_tank(
                        &mut commands,
                        &asset_server,
                        pos,
                        &mut unit_id_counter,
                        unit.get_player(),
                    );
                }
                UnitTileType::Harvester => {
                    HarvesterSpawnManager::spawn_harvester(
                        &mut commands,
                        &asset_server,
                        pos,
                        &mut unit_id_counter,
                        unit.get_player(),
                    );
                }
                _ => {}
            }
        });
    }
}
