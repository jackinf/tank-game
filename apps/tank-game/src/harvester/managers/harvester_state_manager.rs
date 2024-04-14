use crate::common::constants::TileCoord;
use crate::common::resources::game_map::GameMap;
use crate::common::utils::astar::find_path;
use crate::harvester::components::harvester::Harvester;
use crate::harvester::resources::harvester_timer::HarvesterTimer;
use crate::tile::components::gold::Gold;
use crate::tile::components::tile::Tile;
use crate::tile::tile_queries::TileQueries;
use crate::tile::tile_type::TileType;
use bevy::prelude::{Query, Res, ResMut, Time, Transform, Vec2, Vec3Swizzles, With};
use bevy::utils::dbg;
use std::collections::VecDeque;

pub struct HarvesterStateManager;

impl HarvesterStateManager {
    pub fn run_state_machine_step(
        mut timer: ResMut<HarvesterTimer>,
        time: Res<Time>,
        mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
    ) {
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        q_harvesters
            .iter_mut()
            .for_each(|(mut harvester, transform)| {
                if harvester.is_forced_by_player() {
                    // player command overrides everything
                    return;
                }

                if harvester.is_idle() {
                    if harvester.is_full() {
                        harvester.set_returning();
                    } else {
                        harvester.set_harvesting();
                    }
                    return;
                }

                if harvester.is_harvesting() && harvester.is_returning() {
                    // carry on ;)
                    return;
                }
            });
    }

    pub fn find_gold_for_hungry_harvester(
        mut timer: ResMut<HarvesterTimer>,
        time: Res<Time>,
        mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
        tile_query: Query<&Tile>,
        q_gold: Query<&Transform, With<Gold>>,
        game_map: Res<GameMap>,
    ) {
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        q_harvesters
            .iter_mut()
            .for_each(|(mut harvester, transform)| {
                if harvester.is_harvesting() && !harvester.has_movement_path() {
                    let start =
                        TileQueries::find_accessible(&tile_query, &transform.translation.xy())
                            .unwrap();
                    let found_option = find_first_gold(&game_map.get_tile_type_grid_i32(), start);

                    if let Some(goal) = found_option {
                        let path = find_path(&game_map.get_tile_type_grid(), start, goal);
                        dbg!(goal);
                        harvester.set_movement_path(path);
                    }
                }
            });
    }

    pub fn move_harvester_towards_path(
        time: Res<Time>,
        mut q_harvesters: Query<(&mut Harvester, &mut Transform), With<Harvester>>,
        game_map: Res<GameMap>,
    ) {
        let dt = time.delta_seconds();

        q_harvesters
            .iter_mut()
            .for_each(|(mut harvester, mut transform)| {
                if !harvester.has_movement_path() {
                    return;
                }

                let current_pos = transform.translation.xy();
                let last_tile = harvester.get_movement_path().into_iter().next().unwrap();
                let mut last_world_pos = game_map
                    .get_tile_to_world_coordinates()
                    .get(&last_tile)
                    .unwrap();
                let mut last_world_pos = Vec2::new(last_world_pos.0, last_world_pos.1);

                let direction = last_world_pos - current_pos;
                let distance_to_move = harvester.get_speed() * dt;

                // Smooth movement
                if direction.length() > distance_to_move {
                    let new_pos = current_pos + direction.normalize() * distance_to_move;
                    transform.translation = new_pos.extend(transform.translation.z);
                } else {
                    transform.translation = last_world_pos.extend(transform.translation.z);
                    harvester.try_take_next_position_in_path();
                }
            });
    }
}

// TODO: Move this to a common module and make it more readable
fn find_first_gold(grid: &Vec<Vec<i32>>, start: TileCoord) -> Option<TileCoord> {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // Up, down, left, right
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    // Start BFS from the starting position
    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some((x, y)) = queue.pop_front() {
        // Check if current cell contains gold
        if grid[x][y] == TileType::Gold as i32 {
            return Some((x, y));
        }

        // Explore the four possible directions
        for &(dx, dy) in &directions {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if !visited[nx][ny]
                    && grid[nx][ny] != TileType::Wall as i32
                    && grid[nx][ny] != TileType::Water as i32
                {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    None
}
