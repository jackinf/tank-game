use crate::actions::calculate_bfs::calculate_bfs;
use crate::actions::calculate_tile_world_position::calculate_world_to_tile_position;
use crate::actions::get_all_blocking_cells::get_all_blocking_cells;
use crate::constants::TileCoord;
use crate::features::building::components::Building;
use crate::features::harvester::components::Harvester;
use crate::features::tile::{Gold, Tile};
use crate::resources::mission_info_resource::MissionInfoResource;
use bevy::prelude::{Query, Res, Transform, Vec3Swizzles, With};
use std::collections::{HashSet, VecDeque};

pub fn find_gold_for_hungry_harvester(
    mission_info_resource: Res<MissionInfoResource>,
    mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
    q_golds: Query<&Gold>,
    q_tiles: Query<&Tile>,
    q_buildings: Query<&Building>,
) {
    let grid_size = mission_info_resource.get_grid_size();
    let blocking = get_all_blocking_cells(&q_tiles, &q_buildings);
    let golds = q_golds
        .iter()
        .map(|gold| gold.at())
        .collect::<HashSet<TileCoord>>();

    q_harvesters
        .iter_mut()
        .filter(|(harvester, _)| harvester.is_searching_for_gold())
        .for_each(|(mut harvester, transform)| {
            let start = calculate_world_to_tile_position(&transform.translation.xy());

            if let Some(path) = calculate_bfs(grid_size, start, &golds, &blocking) {
                harvester.set_movement_path(VecDeque::from(path));
                harvester.set_moving_to_gold();
            } else {
                println!("Cannot find gold for harvester");
            }
        });
}
