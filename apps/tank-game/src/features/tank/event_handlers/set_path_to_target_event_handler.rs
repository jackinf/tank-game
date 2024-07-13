use crate::actions::calculate_astar_path::calculate_astar_path;
use crate::actions::calculate_tile_world_position::{
    calculate_tile_to_world_position, calculate_world_to_tile_position,
};
use crate::actions::get_all_blocking_cells::get_all_blocking_cells;
use crate::features::building::components::Building;
use crate::features::tank::events::SetPathToTargetEvent;
use crate::features::tank::Tank;
use crate::features::tile::Tile;
use crate::resources::mission_info_resource::MissionInfoResource;
use bevy::prelude::{EventReader, Query, Res, Transform, Vec2, Vec3Swizzles};
use std::collections::VecDeque;

pub fn set_path_to_target_event_handler(
    q_tiles: Query<&Tile>,
    mut q_buildings: Query<&Building>,
    mut q_tanks: Query<(&mut Tank, &Transform)>,
    mut set_path_to_target_event_reader: EventReader<SetPathToTargetEvent>,
    mission_info_resource: Res<MissionInfoResource>,
) {
    for set_path_to_target_event in set_path_to_target_event_reader.read() {
        let source_id = set_path_to_target_event.source.clone();
        let target_id = set_path_to_target_event.target.clone();

        let start = q_tanks
            .iter()
            .find(|(tank, _)| tank.get_id() == source_id)
            .map(|(_, tr)| calculate_world_to_tile_position(&tr.translation.xy()));
        let goal = q_tanks
            .iter()
            .find(|(tank, _)| tank.get_id() == target_id)
            .map(|(_, tr)| calculate_world_to_tile_position(&tr.translation.xy()));

        if start.is_none() || goal.is_none() {
            continue;
        }
        let start = start.unwrap();
        let goal = goal.unwrap();

        if let Some((mut source_tank, _)) = q_tanks
            .iter_mut()
            .find(|(tank, _)| tank.get_id() == source_id)
        {
            let grid_size = mission_info_resource.get_grid_size();
            let all_blocking_cells = get_all_blocking_cells(&q_tiles, &q_buildings);

            let world_path: VecDeque<Vec2> =
                calculate_astar_path(grid_size, start, goal, &all_blocking_cells)
                    .iter()
                    .map(|&key| calculate_tile_to_world_position(&key))
                    .collect();

            source_tank.set_movement_path(world_path);
            source_tank.set_stop_when_target_in_range(true);
        }
    }
}
