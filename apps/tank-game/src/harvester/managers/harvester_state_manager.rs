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
use crate::building::building_type::BuildingType;
use crate::building::components::building::Building;
use crate::common::player::Player;
use crate::con_menu::resources::menu_info::MenuInfo;

pub struct HarvesterStateManager;

impl HarvesterStateManager {
    pub fn begin(
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
                if harvester.is_idle() {
                    harvester.set_searching_for_gold();
                }
            });
    }

    pub fn find_gold_for_hungry_harvester(
        mut timer: ResMut<HarvesterTimer>,
        time: Res<Time>,
        mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
        tile_query: Query<&Tile>,
        game_map: Res<GameMap>,
    ) {
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        q_harvesters
            .iter_mut()
            .filter(|(harvester, _)| harvester.is_searching_for_gold())
            .for_each(|(mut harvester, transform)| {
                let start =
                    TileQueries::find_accessible(&tile_query, &transform.translation.xy()).unwrap();
                let found_option = find_first_gold(&game_map.get_tile_type_grid_i32(), start);

                if let Some(goal) = found_option {
                    let path = find_path(&game_map.get_tile_type_grid(), start, goal);
                    dbg!(goal);
                    harvester.set_movement_path(path);
                    harvester.set_moving_to_gold(goal);
                }
            });
    }

    pub fn move_harvester_towards_path(
        time: Res<Time>,
        mut q_harvesters: Query<(&mut Harvester, &mut Transform), With<Harvester>>,
        game_map: Res<GameMap>,
        mut menu_info: ResMut<MenuInfo>,
    ) {
        let dt = time.delta_seconds();

        q_harvesters
            .iter_mut()
            .filter(|(harvester, _)| harvester.has_movement_path())
            .filter(|(harvester, _)| harvester.is_moving_to_gold() || harvester.is_returning_to_base())
            .for_each(|(mut harvester, mut transform)| {
                let next_tile = harvester.get_movement_path().into_iter().next().unwrap();
                let mut next_world_pos = game_map
                    .get_tile_to_world_coordinates()
                    .get(&next_tile)
                    .unwrap();
                let mut last_world_pos = Vec2::new(next_world_pos.0, next_world_pos.1);

                let current_pos = transform.translation.xy();
                let direction_vector = last_world_pos - current_pos;
                let direction = direction_vector.normalize();
                let distance_to_move = harvester.get_speed() * dt;

                let is_close_enough = direction_vector.length() < distance_to_move;
                if is_close_enough {
                    // Made it to the temporary destination, now pick the next one, or start mining
                    transform.translation = last_world_pos.extend(transform.translation.z);
                    harvester.try_take_next_position_in_path();
                    if !harvester.has_movement_path() {
                        // Made it. Decide, what to do now based on the previous action.

                        if harvester.is_moving_to_gold() {
                            harvester.set_harvesting();
                        } else if harvester.is_returning_to_base() {
                            let unloaded_gold = harvester.unload_gold();
                            menu_info.add_money(unloaded_gold as i32);
                            harvester.set_idle();
                        }
                    }
                } else {
                    // Continue movement
                    let new_pos = current_pos + direction * distance_to_move;
                    transform.translation = new_pos.extend(transform.translation.z);
                }
            });
    }

    pub fn collect_gold(
        mut timer: ResMut<HarvesterTimer>,
        time: Res<Time>,
        mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
        mut q_gold: Query<&Gold>,
        game_map: Res<GameMap>
    ) {
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        let timestamp = time.elapsed_seconds_f64();

        q_harvesters
            .iter_mut()
            .filter(|(harvester, _)| harvester.is_harvesting())
            .filter(|(harvester, _)| harvester.is_full() == false)
            .filter(|(harvester, _)| harvester.is_cooling_down_to_harvest(timestamp) == false)
            .for_each(|(mut harvester, transform)| {
                // TODO: check if harvester is close enough to gold
                let harvester_pos = transform.translation.xy();
                let gold_res = q_gold
                    .iter_mut()
                    .find(|gold| {
                        let gold_pos = game_map.get_tile_to_world_coordinates().get(&gold.at());
                        if gold_pos.is_none() {
                            return false;
                        }
                        let (x, y) = gold_pos.unwrap();
                        let gold_pos = Vec2::new(*x, *y);
                        let distance = (harvester_pos - gold_pos).length();
                        distance < 10.0
                    });

                match gold_res {
                    None => {
                        // search for gold again
                        harvester.set_searching_for_gold();
                    },
                    Some(_) => {
                        harvester.collect_gold(10, timestamp);
                        println!("Harvester {} collected 10 gold!", harvester.get_id());

                        if harvester.is_full() {
                            harvester.set_find_base_to_return();
                        }

                        // TODO: decrease gold value somehow
                        // let mut res = q_gold.get_mut(gold_id).unwrap();
                        // let mut gold: &mut Gold = res.1;
                        // gold.reduce_value_by(10);
                    }
                }
            });
    }

    pub fn find_base_to_return(
        mut timer: ResMut<HarvesterTimer>,
        time: Res<Time>,
        mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
        game_map: Res<GameMap>,
        q_buildings: Query<&Building>,
        q_tiles: Query<&Tile>,
    ) {
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        let building_infos: Vec<(TileCoord, Player)> = q_buildings.iter()
            .filter(|building| building.get_building_type() == BuildingType::Base)
            .map(|building| {
                let tile_coord = building.get_building_tile_coord();
                (tile_coord, building.get_player().clone())
            })
            .collect();

        q_harvesters
            .iter_mut()
            .filter(|(harvester, _)| harvester.is_find_base_to_return())
            .for_each(|(mut harvester, transform)| {
                let building_res = building_infos.iter()
                    .find(|(_, player)| player == &harvester.get_player());

                if let Some((base_tile, _)) = building_res {
                    let start =
                        TileQueries::find_accessible(&q_tiles, &transform.translation.xy()).unwrap();
                    let path = find_path(&game_map.get_tile_type_grid(), start, *base_tile);
                    harvester.set_movement_path(path);
                    harvester.set_returning_to_base();
                }
            });
    }

    pub fn return_to_base(
        mut timer: ResMut<HarvesterTimer>,
        time: Res<Time>,
        mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
        game_map: Res<GameMap>
    ) {
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        q_harvesters
            .iter_mut()
            .filter(|(harvester, _)| harvester.is_returning_to_base())
            .for_each(|(mut harvester, transform)| {

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
