use crate::actions::calculate_tile_world_position::{
    calculate_tile_to_world_position, calculate_world_to_tile_position,
};
use crate::actions::get_all_blocking_cells::get_all_blocking_cells;
use crate::constants::{GridSize, TileCoord};
use crate::features::building::components::Building;
use crate::features::tank::resources::TankUngroupTimer;
use crate::features::tank::Tank;
use crate::features::tile::Tile;
use crate::resources::mission_info_resource::MissionInfoResource;
use bevy::audio::AudioBundle;
use bevy::prelude::{
    default, AssetServer, Commands, Entity, Query, Res, ResMut, Time, Transform, Vec3Swizzles, With,
};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn sys_ungroup_tanks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut tank_ungroup_timer: ResMut<TankUngroupTimer>,
    mut q_tanks: Query<(Entity, &mut Tank, &Transform), With<Tank>>,
    mission_info_resource: Res<MissionInfoResource>,
    q_tiles: Query<&Tile>,
    q_buildings: Query<&Building>,
) {
    if !tank_ungroup_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let grid_size = mission_info_resource.get_grid_size();
    let blocking = get_all_blocking_cells(&q_tiles, &q_buildings);

    let groups: HashMap<TileCoord, Vec<Entity>> = q_tanks
        .iter()
        .filter(|(_, tank, _)| !tank.is_moving())
        .map(|(entity, _, transform)| {
            (
                calculate_world_to_tile_position(&transform.translation.xy()),
                entity,
            )
        })
        .fold(HashMap::new(), |mut acc, (key, entity)| {
            acc.entry(key).or_insert_with(Vec::new).push(entity);
            acc
        });

    let mut occupied_tiles: HashSet<_> = groups.keys().cloned().collect();

    // find a new destination tile for each group that has more than one tank in the group
    let mut at_least_one_conflict = false;
    for (tile, entities) in groups {
        if entities.len() > 1 {
            for (i, entity) in entities.iter().enumerate() {
                if i == 0 {
                    // Keep the first tank in the original tile
                    continue;
                }

                // Find a new vacant neighboring tile for each remaining tank
                if let Some(new_tile) =
                    find_vacant_neighbor(tile, &occupied_tiles, &grid_size, &blocking)
                {
                    if let Ok((_, mut tank, _)) = q_tanks.get_mut(*entity) {
                        at_least_one_conflict = true;
                        let world_coord = calculate_tile_to_world_position(&new_tile);
                        let path = VecDeque::from(vec![world_coord]);
                        tank.set_movement_path(path);
                    }
                    occupied_tiles.insert(new_tile);
                }
            }
        }
    }

    if at_least_one_conflict {
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/hugh.ogg"),
            ..default()
        });
    }
}

fn find_vacant_neighbor(
    tile: TileCoord,
    occupied_tiles: &HashSet<TileCoord>,
    grid_size: &GridSize,
    blocking: &HashSet<TileCoord>,
) -> Option<TileCoord> {
    let directions = [
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
        (-1, 0), // Up
    ];

    let (width, height) = grid_size;

    for &(dx, dy) in &directions {
        let (tx, ty) = (tile.0 as isize, tile.1 as isize);
        let (nx, ny) = (tx + dx, ty + dy);

        if nx >= 0 && nx < *width as isize && ny >= 0 && ny < *height as isize {
            let neighbor = (nx as usize, ny as usize);
            if !occupied_tiles.contains(&neighbor) && !blocking.contains(&neighbor) {
                return Some(neighbor);
            }
        }
    }
    None
}
