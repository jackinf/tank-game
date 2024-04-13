use crate::common::components::tile::Tile;
use crate::common::components::unit_id::UnitId;
use crate::common::resources::game_map::GameMap;
use crate::common::resources::me::Me;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::utils::dbg;
use std::collections::{HashMap, VecDeque};

use crate::common::tile_queries::TileQueries;
use crate::common::utils::astar;
use crate::cursor::resources::cursor_coordinates::CursorCoordinates;
use crate::tank::components::tank::Tank;
use crate::tank::components::tank_gun::TankGun;

// TODO: rename to: TankActionManager
pub struct TankMovementManager;

impl TankMovementManager {
    pub fn set_tank_target_position_to_move(
        mut tank_query: Query<(&mut Tank, &mut Sprite), With<Tank>>,
        tile_query: Query<&Tile>,
        mut mouse_button_events: EventReader<MouseButtonInput>,
        mut key_button_events: EventReader<KeyboardInput>,
        my_world_coords: Res<CursorCoordinates>,
        game_map: Res<GameMap>,
        me: Res<Me>,
    ) {
        for key_button_event in key_button_events.read() {
            if let ButtonState::Pressed = key_button_event.state {
                // selects everything
                if key_button_event.key_code == KeyCode::Space {
                    for (mut tank, mut sprite) in &mut tank_query.iter_mut() {
                        tank.select(&mut sprite);
                    }
                }

                // unselects everything
                if key_button_event.key_code == KeyCode::Escape {
                    for (mut tank, mut sprite) in &mut tank_query.iter_mut() {
                        tank.deselect(&mut sprite);
                    }
                }
            }
        }

        for mouse_button_event in mouse_button_events.read() {
            if MouseButton::Right == mouse_button_event.button
                && mouse_button_event.state == ButtonState::Pressed
            {
                let wx = my_world_coords.0.x;
                let wy = my_world_coords.0.y;

                let clicked_on_enemy_tank_id: Option<UnitId> = tank_query
                    .iter_mut()
                    .find(|(tank, _)| tank.is_clicked_on(wx, wy) && !tank.is_mine(&me))
                    .map(|(tank, _)| tank.get_id().clone());

                if let Some(goal) = TileQueries::find_accessible(&tile_query, &my_world_coords.0) {
                    dbg!(goal);
                    let selected_tanks = &mut tank_query
                        .iter_mut()
                        .filter(|(tank, _)| tank.selected)
                        .map(|(tank, _)| tank);

                    for mut tank in selected_tanks {
                        if let Some(start) =
                            // TODO: why not use translation instead of target_position?
                            TileQueries::find_accessible(&tile_query, &tank.target_position)
                        {
                            // TODO: expensive! optimize this
                            // TODO: consider using use bevy::utils::petgraph::algo::astar;
                            let path_f32: VecDeque<(f32, f32)> =
                                astar::find_path(&game_map.get_tile_type_grid(), start, goal)
                                    .iter()
                                    .filter_map(|&key| {
                                        game_map.get_tile_to_world_coordinates().get(&key)
                                    }) // Use `get` to lookup each key in the map, filter_map to ignore None results
                                    .cloned() // Clone the (f32, f32) values to move them into the Vec
                                    .collect();

                            tank.set_movement_path(path_f32);

                            // if let Some(enemy_tank_id) = clicked_on_enemy_tank_id {
                            //     dbg!("attacking enemy tank");
                            //     tank.set_target(enemy_tank_id.clone());
                            // }
                        }
                    }

                    if let Some(enemy_tank_id) = clicked_on_enemy_tank_id {
                        tank_query
                            .iter_mut()
                            .filter(|(tank, _)| tank.is_mine(&me) && tank.selected)
                            .map(|(tank, _)| tank)
                            .for_each(|mut my_selected_tank| {
                                dbg!("attacking enemy tank");
                                my_selected_tank.set_target(enemy_tank_id.clone());
                                // refer to tank_shooting_manager.rs
                            });
                    }
                }
            }
        }
    }

    pub fn move_tanks_towards_target(
        time: Res<Time>,
        mut tank_query: Query<(&mut Transform, &mut Tank), (With<Tank>, Without<TankGun>)>,
        mut gun_query: Query<(&mut Transform, &TankGun), (With<TankGun>, Without<Tank>)>,
    ) {
        let dt = time.delta_seconds();

        let tank_id_and_positions: HashMap<UnitId, Vec2> = tank_query
            .iter()
            .map(|(transform, tank)| (tank.id.clone(), transform.translation.xy()))
            .collect();

        for (mut transform, mut tank) in tank_query.iter_mut().filter(|(_, tank)| tank.is_moving())
        {
            let current_pos = transform.translation.xy();
            let direction = tank.target_position - current_pos;
            let distance_to_move = tank.speed * dt;

            // if tank has target, check if it's close enough to stop
            if tank.has_target() {
                let target = tank_id_and_positions
                    .get(&tank.get_target().unwrap())
                    .unwrap();
                let vector = *target - current_pos;
                let total_distance = vector.length();
                dbg!(total_distance);
                if total_distance < tank.get_radius() {
                    println!("STOP!");
                    tank.stop();
                    continue;
                }
            }

            // // TODO: improve code, and move into tank component
            // // let total_distance = (tank.target_position - current_pos).length();
            // let total_distance = if tank.movement_path.len() > 0 {
            //     let xy = tank.movement_path.iter().last().unwrap().clone();
            //     let vector = Vec2::new(xy.0, xy.1) - transform.translation.truncate();
            //     let total_distance = vector.length();
            //     // dbg!(total_distance);
            //     total_distance
            // } else {
            //     let vector = tank.target_position - transform.translation.truncate();
            //     let total_distance = vector.length();
            //     // dbg!(total_distance);
            //     // dbg!(total_distance);
            //     total_distance
            // };
            // if total_distance < tank.get_radius() {
            //     tank.stop();
            //     continue;
            // }

            // Smooth movement
            if direction.length() > distance_to_move {
                let new_pos = current_pos + direction.normalize() * distance_to_move;
                // let target_vec3 = Vec3::new(new_pos.x, new_pos.y, transform.translation.z);

                // TODO: account for a bug if the speed is too high.
                // if so, use simpler:
                transform.translation = Vec3::new(new_pos.x, new_pos.y, transform.translation.z);
                // transform.translation = transform
                //     .translation
                //     .lerp(target_vec3, tank.speed / 10.0 * time.delta_seconds());
            } else {
                transform.translation.x = tank.target_position.x;
                transform.translation.y = tank.target_position.y;

                if tank.has_target() && distance_to_move < 10.0 {
                    tank.stop();
                } else {
                    tank.try_take_next_position_in_path();
                }
            }

            // Rotate tank gun smoothly
            if let Some((mut gun_transform, _)) = gun_query
                .iter_mut()
                .find(|(_, gun)| gun.parent_id.0 == tank.id.0)
            {
                let target_angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
                gun_transform.rotation = gun_transform.rotation.slerp(
                    Quat::from_rotation_z(target_angle),
                    10.0 * time.delta_seconds(),
                );
            }
        }
    }
}
