use crate::features::cursor::CursorCoordinates;
use crate::features::tank::components::Tank;
use crate::features::tile::{find_accessible_tile_coord, Tile};
use crate::features::unit::UnitId;
use crate::resources::game_map::GameMap;
use crate::resources::me::Me;
use crate::utils::astar;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{EventReader, KeyCode, MouseButton, Query, Res, Sprite, With};
use std::collections::VecDeque;

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
            let world_coords = my_world_coords.get_world();
            let wx = world_coords.x;
            let wy = world_coords.y;

            let clicked_on_enemy_tank_id: Option<UnitId> = tank_query
                .iter_mut()
                .find(|(tank, _)| tank.is_clicked_on(wx, wy) && !tank.is_mine(&me))
                .map(|(tank, _)| tank.get_id().clone());

            if let Some(goal) = find_accessible_tile_coord(&tile_query, &world_coords) {
                let selected_tanks = &mut tank_query
                    .iter_mut()
                    .filter(|(tank, _)| tank.selected)
                    .map(|(tank, _)| tank);

                for mut tank in selected_tanks {
                    if let Some(start) =
                        // TODO: why not use translation instead of target_position?
                        find_accessible_tile_coord(&tile_query, &tank.target_position)
                    {
                        // TODO: expensive! optimize this
                        // TODO: consider using use bevy::utils::petgraph::algo::astar;
                        let path_f32: VecDeque<(f32, f32)> =
                            astar::find_path(&game_map.get_tile_type_grid_usize(), start, goal)
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
