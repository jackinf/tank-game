use std::collections::HashMap;

use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res, ResMut};
use iyes_perf_ui::PerfUiCompleteBundle;

use crate::building::managers::building_spawn_manager::BuildingSpawnManager;
use crate::common::constants::{OFFSET_X, OFFSET_Y, TILE_SIZE};
use crate::common::resources::game_map::GameMap;
use crate::preparation::file_helpers::{FileHelpers, MainAssetInfo};
use crate::preparation::load_mission::{load_mission, MissionInfo};
use crate::preparation::main_asset_info_resource::MainAssetInfoResource;
use crate::preparation::mission_info_resource::MissionInfoResource;
use crate::tile::managers::tile_spawn_manager::TileSpawnManager;
use crate::unit::managers::unit_spawn_manager::UnitSpawnManager;
use crate::unit::resources::unit_id_counter::UnitIdCounter;

pub fn setup1(
    mut main_asset_info_resource: ResMut<MainAssetInfoResource>,
    mut mission_info_resource: ResMut<MissionInfoResource>,
) {
    let assets_result = FileHelpers::read_assets("apps/tank-game/assets/main_assets.tsj");
    if let Err(err) = assets_result {
        panic!("Failed to read assets: {}", err);
    }
    let assets: MainAssetInfo = assets_result.unwrap();
    main_asset_info_resource.initialize(assets.get_tiles().clone());

    let mission_info = load_mission(
        &main_asset_info_resource,
        "apps/tank-game/assets/mission01.tmj",
    );
    if let Err(err) = mission_info {
        panic!("Failed to load mission: {:?}", err);
    }
    let mission_info: MissionInfo = mission_info.unwrap();
    mission_info_resource.initialize(mission_info);
}

pub fn setup2(
    mission_info_resource: Res<MissionInfoResource>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tank_id_counter: ResMut<UnitIdCounter>,
    mut game_map: ResMut<GameMap>,
) {
    if mission_info_resource.is_empty() {
        panic!("Mission info is not loaded");
    }

    let mut grid_to_tilemap = HashMap::new();

    let calculate_world_position = |row_index: usize, col_index: usize| {
        let x = row_index as f32 * TILE_SIZE + OFFSET_X;
        let y = col_index as f32 * TILE_SIZE + OFFSET_Y;
        Vec2::new(x, y)
    };

    let tile_map = mission_info_resource.get_ground_layer();
    let spawn_tiles_result = TileSpawnManager::spawn_tiles(
        &mut commands,
        &asset_server,
        tile_map,
        &mut grid_to_tilemap,
        calculate_world_position,
    );
    if let Err(err) = spawn_tiles_result {
        panic!("Failed to spawn tiles: {:?}", err);
    }
    let grid = spawn_tiles_result.unwrap();

    let units_layer = mission_info_resource.get_units_layer();
    UnitSpawnManager::spawn_units(
        &mut commands,
        &asset_server,
        &mut tank_id_counter,
        units_layer,
        calculate_world_position,
    );

    let buildings_layer = mission_info_resource.get_buildings_layer();
    BuildingSpawnManager::spawn_buildings(
        &mut commands,
        &asset_server,
        buildings_layer,
        calculate_world_position,
    );

    game_map.set_map(grid, grid_to_tilemap);

    commands.spawn(PerfUiCompleteBundle::default());
}
