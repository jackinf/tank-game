use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res, ResMut};
use iyes_perf_ui::PerfUiCompleteBundle;

use crate::building::managers::building_spawn_manager::BuildingSpawnManager;
use crate::common::constants::{TileCoord, TileGrid, OFFSET_X, OFFSET_Y, TILE_SIZE};
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

    let calculate_world_position = |coord: &TileCoord| {
        let x = coord.0 as f32 * TILE_SIZE + OFFSET_X;
        let y = coord.1 as f32 * TILE_SIZE + OFFSET_Y;
        Vec2::new(x, y)
    };

    let ground_layer = mission_info_resource.get_ground_layer();
    TileSpawnManager::spawn_tiles(
        &mut commands,
        &asset_server,
        &ground_layer,
        calculate_world_position,
    );
    let grid: TileGrid = ground_layer.to_2d_grid();
    let tile_to_world =
        TileSpawnManager::create_tile_to_world_coordinates(&ground_layer, calculate_world_position);
    game_map.set_map(grid, tile_to_world);

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

    commands.spawn(PerfUiCompleteBundle::default());
}
