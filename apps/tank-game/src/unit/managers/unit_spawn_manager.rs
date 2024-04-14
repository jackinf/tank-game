use crate::common::constants::RawGrid;
use crate::common::player::Player;
use crate::tank::managers::tank_spawn_manager::TankSpawnManager;
use crate::unit::resources::unit_id_counter::UnitIdCounter;
use crate::unit::unit_type::UnitType;
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Res, ResMut};

pub struct UnitSpawnManager;

impl UnitSpawnManager {
    pub fn spawn_units(
        mut commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        mut tank_id_counter: &mut ResMut<UnitIdCounter>,
        all_unit_maps: Vec<(RawGrid, Player)>,
        calculate_world_position: fn(usize, usize) -> Vec2,
    ) {
        all_unit_maps.into_iter().for_each(|(unit_map, player)| {
            unit_map
                .iter()
                .enumerate()
                .for_each(|(row_index, row_on_row)| {
                    row_on_row.iter().enumerate().for_each(|(col_index, cell)| {
                        let pos = calculate_world_position(row_index, col_index);

                        if *cell == UnitType::Tank as usize {
                            TankSpawnManager::spawn_tank(
                                &mut commands,
                                &asset_server,
                                pos,
                                &mut tank_id_counter,
                                player.clone(),
                            );
                        }
                    });
                });
        });
    }
}
