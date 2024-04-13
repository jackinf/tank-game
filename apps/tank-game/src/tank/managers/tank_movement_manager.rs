use crate::common::components::tile::Tile;
use crate::common::components::unit_id::UnitId;
use crate::common::constants::TANK_ROTATION_SPEED;
use crate::common::resources::game_map::GameMap;
use crate::common::resources::me::Me;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
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
                        }
                    }

                    tank_query
                        .iter_mut()
                        .filter(|(tank, _)| tank.is_mine(&me) && tank.selected)
                        .map(|(tank, _)| tank)
                        .for_each(|mut my_selected_tank| {
                            match &clicked_on_enemy_tank_id {
                                Some(enemy_tank_id) => {
                                    dbg!(enemy_tank_id);
                                    my_selected_tank.set_target(Some(enemy_tank_id.clone()));
                                    my_selected_tank.set_stop_when_target_in_range(true);
                                }
                                None => {
                                    // TODO: not sure if i want to unset it
                                    // my_selected_tank.set_target(None);
                                }
                            }
                        });
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

        // move all tanks via path
        for (mut transform, mut tank) in tank_query.iter_mut().filter(|(_, tank)| tank.is_moving())
        {
            let current_pos = transform.translation.xy();

            if tank.get_stop_when_target_in_range() {
                println!("STOP WHEN TARGET IN RANGE");

                // if tank has target, check if it's close enough to stop
                if let Some(target) = tank
                    .get_target()
                    .and_then(|target_id| tank_id_and_positions.get(&target_id))
                {
                    let vector = *target - current_pos;
                    let total_distance = vector.length();
                    if total_distance < tank.get_radius() {
                        tank.stop();
                        continue;
                    }
                }
            }

            let direction = tank.target_position - current_pos;
            let distance_to_move = tank.speed * dt;

            // Smooth movement
            if direction.length() > distance_to_move {
                let new_pos = current_pos + direction.normalize() * distance_to_move;
                transform.translation = new_pos.extend(transform.translation.z);
            } else {
                transform.translation = tank.target_position.extend(transform.translation.z);
                tank.try_take_next_position_in_path();
            }
        }

        // Rotate tank gun smoothly for all tanks
        for (transform, tank) in tank_query.iter().filter(|(_, tank)| tank.is_moving()) {
            if let Some((mut gun_transform, _)) = gun_query
                .iter_mut()
                .find(|(_, gun)| gun.parent_id.0 == tank.id.0)
            {
                let direction = tank.target_position - transform.translation.xy();
                let target_angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
                let quat = Quat::from_rotation_z(target_angle);

                gun_transform.rotation =
                    gun_transform.rotation.slerp(quat, TANK_ROTATION_SPEED * dt);
            }
        }
    }
}
