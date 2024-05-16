use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res, ResMut};
use iyes_perf_ui::PerfUiCompleteBundle;

use crate::constants::{TileCoord, TileGrid, OFFSET_X, OFFSET_Y, TILE_SIZE};
use crate::features::building::managers::building_spawn_manager::BuildingSpawnManager;
use crate::features::preparation::mission_info_resource::MissionInfoResource;
use crate::features::tile::managers::tile_spawn_manager::TileSpawnManager;
use crate::features::unit::managers::unit_spawn_manager::UnitSpawnManager;
use crate::features::unit::resources::unit_id_counter::UnitIdCounter;
use crate::resources::game_map::GameMap;

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
