use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use std::collections::VecDeque;

use crate::common::constants::TILE_SIZE;
use crate::common::game_map::GameMap;
use crate::common::tile::Tile;
use crate::cursor::cursor_coordinates::WorldCoordinates;
use crate::tank::tank::Tank;
use crate::tank::tank_gun::TankGun;
use crate::utils::astar;

pub struct TankMovementPlugin;

impl Plugin for TankMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, set_tank_target_position_to_move)
            .add_systems(FixedUpdate, move_tanks_towards_target);
    }
}

fn set_tank_target_position_to_move(
    mut tank_query: Query<(&mut Tank, &mut Sprite), With<Tank>>,
    tile_query: Query<&Tile>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut key_button_events: EventReader<KeyboardInput>,
    my_world_coords: Res<WorldCoordinates>,
    game_map: Res<GameMap>,
) {
    for key_button_event in key_button_events.read() {
        if let ButtonState::Pressed = key_button_event.state {
            // selects everything
            if key_button_event.key_code == KeyCode::Space {
                for (mut tank, mut sprite) in &mut tank_query.iter_mut() {
                    select_tank(&mut tank, &mut sprite);
                }
            }

            // unselects everything
            if key_button_event.key_code == KeyCode::Escape {
                for (mut tank, mut sprite) in &mut tank_query.iter_mut() {
                    deselect_tank(&mut tank, &mut sprite);
                }
            }
        }
    }

    for mouse_button_event in mouse_button_events.read() {
        if let ButtonState::Pressed = mouse_button_event.state {
            let wx = my_world_coords.0.x;
            let wy = my_world_coords.0.y;

            let clicked_on_tank = tank_query
                .iter_mut()
                .find(|(tank, _)| is_tank_clicked_on(wx, wy, tank));

            match clicked_on_tank {
                Some((mut tank, mut sprite)) => {
                    if tank.selected {
                        deselect_tank(&mut tank, &mut sprite);
                    } else {
                        select_tank(&mut tank, &mut sprite);
                    }
                }
                None => {
                    if let Some(tile_goal) = tile_query
                        .iter()
                        .find(|tile| tile.accessible() && tile.in_range(wx, wy))
                    {
                        let goal = tile_goal.get_map_coord();

                        for (mut tank, _) in
                            &mut tank_query.iter_mut().filter(|(tank, _)| tank.selected)
                        {
                            let curr_x = tank.target_position.x;
                            let curr_y = tank.target_position.y;

                            if let Some(tile_start) = tile_query
                                .iter()
                                .find(|tile| tile.accessible() && tile.in_range(curr_x, curr_y))
                            {
                                // game_map.draw_map();

                                let start = tile_start.get_map_coord();
                                let path = astar::find_path(&game_map.0, start, goal);
                                // println!("Path: {:?}", path);

                                let path_f32: VecDeque<(f32, f32)> = path
                                    .iter()
                                    .filter_map(|&key| game_map.1.get(&key)) // Use `get` to lookup each key in the map, filter_map to ignore None results
                                    .cloned() // Clone the (f32, f32) values to move them into the Vec
                                    .collect();

                                // println!("path1 {:?}", path_f32);
                                tank.set_movement_path(path_f32);
                            }

                            // tank.start_moving_to(tile_goal.get_center());
                        }
                    }
                }
            }
        }
    }
}

fn select_tank(tank: &mut Mut<Tank>, sprite: &mut Mut<Sprite>) {
    tank.selected = true;
    sprite.color = Color::rgb(1.0, 9.0, 8.0);
}

fn deselect_tank(tank: &mut Mut<Tank>, sprite: &mut Mut<Sprite>) {
    tank.moving = false;
    tank.selected = false;
    sprite.color = Color::WHITE;
}

fn is_tank_clicked_on(wx: f32, wy: f32, tank: &Mut<Tank>) -> bool {
    let offset = TILE_SIZE / 2.0;

    let x1 = tank.target_position.x - offset;
    let x2 = tank.target_position.x + TILE_SIZE - offset;
    let in_x = x1 <= wx && wx <= x2;

    let y1 = tank.target_position.y - offset;
    let y2 = tank.target_position.y + TILE_SIZE - offset;
    let in_y = y1 <= wy && wy <= y2;

    in_x && in_y
}

fn move_tanks_towards_target(
    time: Res<Time>,
    mut tank_query: Query<(&mut Transform, &mut Tank), (With<Tank>, Without<TankGun>)>,
    mut gun_query: Query<(&mut Transform, &TankGun), (With<TankGun>, Without<Tank>)>,
) {
    for (mut transform, mut tank) in tank_query
        .iter_mut()
        .filter(|(_, tank)| tank.is_moving() && tank.selected)
    {
        let current_pos = transform.translation.xy();
        let direction = tank.target_position - current_pos;
        let distance_to_move = tank.speed * time.delta_seconds();

        // Smooth movement
        if direction.length() > distance_to_move {
            let new_pos = current_pos + direction.normalize() * distance_to_move;
            let target_vec3 = Vec3::new(new_pos.x, new_pos.y, transform.translation.z);

            // TODO: account for a bug if the speed is too high.
            // if so, use simpler:
            // transform.translation = Vec3::new(new_pos.x, new_pos.y, transform.translation.z);
            transform.translation = transform
                .translation
                .lerp(target_vec3, tank.speed / 10.0 * time.delta_seconds());
        } else {
            transform.translation.x = tank.target_position.x;
            transform.translation.y = tank.target_position.y;

            tank.try_take_next_position_in_path();
            // tank.stop();
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
