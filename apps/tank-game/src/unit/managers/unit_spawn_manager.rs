use crate::harvester::managers::harvester_spawn_manager::HarvesterSpawnManager;
use crate::tank::managers::tank_spawn_manager::TankSpawnManager;
use crate::unit::resources::unit_id_counter::UnitIdCounter;
use crate::unit::unit_tile::{UnitTile, UnitTileType};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Res, ResMut};
use crate::preparation::load_mission::{UnitsLayer};

pub struct UnitSpawnManager;

impl UnitSpawnManager {
    pub fn spawn_units(
        mut commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        mut unit_id_counter: &mut ResMut<UnitIdCounter>,
        layer: UnitsLayer,
        calculate_world_position: fn(usize, usize) -> Vec2,
    ) {
        layer
                .get_grid()
                .into_iter()
                .enumerate()
                .for_each(|(row_index, row_on_row)| {
                    row_on_row.into_iter().enumerate().for_each(|(col_index, cell)| {
                        let pos = calculate_world_position(row_index, col_index);

                        match cell.get_unit_type() {
                            UnitTileType::Tank => {
                                TankSpawnManager::spawn_tank(
                                    &mut commands,
                                    &asset_server,
                                    pos,
                                    &mut unit_id_counter,
                                    cell.get_player(),
                                );
                            }
                            UnitTileType::Harvester => {
                                HarvesterSpawnManager::spawn_harvester(
                                    &mut commands,
                                    &asset_server,
                                    pos,
                                    &mut unit_id_counter,
                                    cell.get_player(),
                                );
                            }
                            _ => {}
                        }
                    });
                });
    }
}
