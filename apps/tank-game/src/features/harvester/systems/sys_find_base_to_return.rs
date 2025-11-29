use crate::actions::calculate_astar_path::calculate_astar_path;
use crate::actions::get_all_blocking_cells::get_all_blocking_cells;
use crate::constants::TileCoord;
use crate::features::building::components::Building;
use crate::features::building::types::BuildingTileType;
use crate::features::harvester::components::Harvester;
use crate::features::tile::{find_accessible_tile_coord, Tile};
use crate::resources::mission_info_resource::MissionInfoResource;
use crate::types::player::Player;
use bevy::prelude::{Query, Res, Transform, Vec3Swizzles, With};
use std::collections::VecDeque;

pub fn sys_find_base_to_return(
    mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
    q_buildings: Query<&Building>,
    q_tiles: Query<&Tile>,
    mission_info_resource: Res<MissionInfoResource>,
) {
    let building_infos: Vec<(TileCoord, Option<Player>)> = q_buildings
        .iter()
        .filter(|building| building.get_building_tile_type() == BuildingTileType::Base)
        .map(|building| {
            let tile_coord = building.get_door();
            (tile_coord, building.get_player().clone())
        })
        .collect();

    let blocking = get_all_blocking_cells(&q_tiles, &q_buildings);
    let grid_size = mission_info_resource.get_grid_size();

    q_harvesters
        .iter_mut()
        .filter(|(harvester, _)| harvester.is_find_base_to_return())
        .for_each(|(mut harvester, transform)| {
            let building_res = building_infos
                .iter()
                .find(|(_, player)| player == &harvester.get_player());

            if let Some((base_tile, _)) = building_res {
                let start =
                    find_accessible_tile_coord(&q_tiles, &transform.translation.xy()).unwrap();
                let path = calculate_astar_path(grid_size, start, *base_tile, &blocking);
                harvester.set_movement_path(VecDeque::from(path));
                harvester.set_returning_to_base();
            }
        });
}
