use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res, ResMut};
use iyes_perf_ui::PerfUiCompleteBundle;

use crate::constants::TileGrid;
use crate::features::building::actions::spawn_buildings::spawn_buildings;
use crate::features::tile::{create_tile_to_world_coordinates, spawn_tiles};
use crate::features::unit::{spawn_units, UnitIdCounter};
use crate::resources::building_map::BuildingMap;
use crate::resources::ground_map::GroundMap;
use crate::resources::map_trait::MapTrait;
use crate::resources::mission_info_resource::MissionInfoResource;
use crate::resources::resource_map::ResourceMap;
use crate::resources::unit_map::UnitMap;

pub fn setup_mission(
    mission_info_resource: Res<MissionInfoResource>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tank_id_counter: ResMut<UnitIdCounter>,
    mut game_map: ResMut<GroundMap>,
    mut gold_map: ResMut<ResourceMap>,
    mut unit_map: ResMut<UnitMap>,
    mut building_map: ResMut<BuildingMap>,
) {
    if mission_info_resource.is_empty() {
        panic!("Mission info is not loaded");
    }

    /*
       Prepare Ground layer
    */

    let ground_layer = mission_info_resource.get_ground_layer();
    spawn_tiles(&mut commands, &asset_server, &ground_layer);

    let ground_grid: TileGrid = ground_layer.to_2d_grid();
    let ground_tile_to_world = create_tile_to_world_coordinates(&ground_layer);
    game_map.set_map(ground_grid, ground_tile_to_world);

    /*
       Prepare Resource layer
    */

    let resource_layer = mission_info_resource.get_resource_layer();
    spawn_tiles(&mut commands, &asset_server, &resource_layer);

    let gold_grid: TileGrid = resource_layer.to_2d_grid();
    let gold_tile_to_world = create_tile_to_world_coordinates(&resource_layer);
    gold_map.set_map(gold_grid, gold_tile_to_world);

    /*
       Prepare Units layer
    */

    let units_layer = mission_info_resource.get_units_layer();
    spawn_units(
        &mut commands,
        &asset_server,
        &mut tank_id_counter,
        units_layer,
    );

    let unit_grid: TileGrid = resource_layer.to_2d_grid();
    let unit_tile_to_world = create_tile_to_world_coordinates(&resource_layer);
    unit_map.set_map(unit_grid, unit_tile_to_world);

    /*
       Prepare Buildings layer
    */

    let buildings_layer = mission_info_resource.get_buildings_layer();
    spawn_buildings(&mut commands, &asset_server, buildings_layer);

    // let building_grid: TileGrid = buildings_layer.to_2d_grid();
    // let building_tile_to_world = create_tile_to_world_coordinates(&resource_layer);
    // building_map.set_map(building_grid, building_tile_to_world);

    commands.spawn(PerfUiCompleteBundle::default());
}
