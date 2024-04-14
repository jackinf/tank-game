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
