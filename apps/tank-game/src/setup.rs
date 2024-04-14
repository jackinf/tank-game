use std::collections::HashMap;

use crate::building::building_type::BuildingType;
use crate::building::managers::building_spawn_manager::BuildingSpawnManager;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res, ResMut};

use crate::common::constants::{RawGrid, TileGrid, OFFSET_X, OFFSET_Y, TILE_SIZE};

use crate::common::player::Player;
use crate::common::resources::game_map::GameMap;
use crate::common::utils::enum_helpers::EnumHelpers;

use crate::common::utils::file_helpers::FileHelpers;
use crate::tank::managers::tank_spawn_manager::TankSpawnManager;
use crate::tile::managers::tile_manager::TileManager;
use crate::tile::managers::tile_spawn_manager::TileSpawnManager;
use crate::tile::tile_type::TileType;
use crate::unit::managers::unit_spawn_manager::UnitSpawnManager;
use crate::unit::resources::unit_id_counter::UnitIdCounter;
use crate::unit::unit_type::UnitType;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tank_id_counter: ResMut<UnitIdCounter>,
    mut game_map: ResMut<GameMap>,
) {
    let tile_map = FileHelpers::read_map_from_file("apps/tank-game/assets/map2.txt");
    let p1_unit_map = FileHelpers::read_map_from_file("apps/tank-game/assets/map2_p1_units.txt");
    let p2_unit_map = FileHelpers::read_map_from_file("apps/tank-game/assets/map2_p2_units.txt");
    let all_unit_maps: Vec<(RawGrid, Player)> =
        vec![(p1_unit_map, Player::P1), (p2_unit_map, Player::P2)];

    // Reuse unit map for now
    let p1_building_map =
        FileHelpers::read_map_from_file("apps/tank-game/assets/map2_p1_units.txt");
    let p2_building_map =
        FileHelpers::read_map_from_file("apps/tank-game/assets/map2_p2_units.txt");
    let all_building_maps: Vec<(RawGrid, Player)> =
        vec![(p1_building_map, Player::P1), (p2_building_map, Player::P2)];

    let mut grid_to_tilemap = HashMap::new();

    let calculate_world_position = |row_index: usize, col_index: usize| {
        let x = row_index as f32 * TILE_SIZE + OFFSET_X;
        let y = col_index as f32 * TILE_SIZE + OFFSET_Y;
        Vec2::new(x, y)
    };

    let grid = TileSpawnManager::spawn_tiles(
        &mut commands,
        &asset_server,
        tile_map,
        &mut grid_to_tilemap,
        calculate_world_position,
    );

    UnitSpawnManager::spawn_units(
        &mut commands,
        &asset_server,
        &mut tank_id_counter,
        all_unit_maps,
        calculate_world_position,
    );

    BuildingSpawnManager::spawn_buildings(
        &mut commands,
        &asset_server,
        all_building_maps,
        calculate_world_position,
    );

    game_map.set_map(grid, grid_to_tilemap);
}
