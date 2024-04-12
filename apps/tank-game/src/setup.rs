use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res, ResMut};

use crate::common::constants::{Player, RawGrid, UnitType, OFFSET_X, OFFSET_Y, TILE_SIZE};
use crate::common::managers::tile_manager::TileManager;
use crate::common::resources::game_map::GameMap;
use crate::common::resources::me::Me;
use crate::common::resources::unit_id_counter::UnitIdCounter;
use crate::common::utils::file_helpers::FileHelpers;
use crate::tank::managers::tank_spawn_manager::TankSpawnManager;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tank_id_counter: ResMut<UnitIdCounter>,
    mut game_map: ResMut<GameMap>,
    me: Res<Me>,
) {
    let tile_map = FileHelpers::read_map_from_file("apps/tank-game/assets/map1.txt");
    let p1_unit_map = FileHelpers::read_map_from_file("apps/tank-game/assets/map1_p1_units.txt");
    let p2_unit_map = FileHelpers::read_map_from_file("apps/tank-game/assets/map1_p2_units.txt");
    let all_unit_maps: Vec<(RawGrid, Player)> =
        vec![(p1_unit_map, Player::P1), (p2_unit_map, Player::P2)];

    let mut grid_to_tilemap = HashMap::new();

    let calculate_world_position = |row_index: usize, col_index: usize| {
        let x = row_index as f32 * TILE_SIZE + OFFSET_X;
        let y = col_index as f32 * TILE_SIZE + OFFSET_Y;
        Vec2::new(x, y)
    };

    let grid = tile_map
        .into_iter()
        .enumerate()
        .map(|(row_index, row_on_row)| {
            row_on_row
                .into_iter()
                .enumerate()
                .map(|(col_index, cell)| {
                    let pos = calculate_world_position(row_index, col_index);
                    let map_coord = (row_index, col_index);
                    grid_to_tilemap.insert(map_coord, (pos.x, pos.y));

                    TileManager::spawn_tile(&mut commands, &asset_server, pos, cell, map_coord)
                })
                .collect()
        })
        .collect();

    all_unit_maps.into_iter().for_each(|(unit_map, player)| {
        unit_map
            .iter()
            .enumerate()
            .for_each(|(row_index, row_on_row)| {
                row_on_row.iter().enumerate().for_each(|(col_index, cell)| {
                    let world_pos = calculate_world_position(row_index, col_index);

                    if *cell == UnitType::Tank as usize {
                        TankSpawnManager::spawn_tank(
                            &mut commands,
                            &asset_server,
                            world_pos,
                            &mut tank_id_counter,
                            player.clone(),
                        );
                    }
                });
            });
    });

    game_map.set_map(grid, grid_to_tilemap);
}
