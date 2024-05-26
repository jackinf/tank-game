use crate::actions::compute_astar_path::compute_astar_path;
use crate::constants::{TileCoord, WorldCoord};
use crate::features::building::components::Building;
use crate::features::cursor::CursorCoordinates;
use crate::features::tank::components::Tank;
use crate::features::tile::{find_accessible_tile_coord, Tile};
use crate::features::unit::UnitId;
use crate::resources::ground_map::GroundMap;
use crate::resources::map_trait::MapTrait;
use crate::resources::me::Me;
use crate::utils::astar;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{EventReader, KeyCode, MouseButton, Query, Res, Sprite, With};
use std::collections::{HashSet, VecDeque};

pub fn set_tank_target_position_to_move(
    mut q_tanks: Query<(&mut Tank, &mut Sprite), With<Tank>>,
    q_tiles: Query<&Tile>,
    q_buildings: Query<&Building>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut key_button_events: EventReader<KeyboardInput>,
    my_world_coords: Res<CursorCoordinates>,
    ground_map: Res<GroundMap>,
    me: Res<Me>,
) {
    for key_button_event in key_button_events.read() {
        if let ButtonState::Pressed = key_button_event.state {
            // selects everything
            if key_button_event.key_code == KeyCode::Space {
                for (mut tank, mut sprite) in &mut q_tanks.iter_mut() {
                    tank.select(&mut sprite);
                }
            }

            // unselects everything
            if key_button_event.key_code == KeyCode::Escape {
                for (mut tank, mut sprite) in &mut q_tanks.iter_mut() {
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

            let clicked_on_enemy_tank_id: Option<UnitId> = q_tanks
                .iter_mut()
                .find(|(tank, _)| tank.is_clicked_on(wx, wy) && !tank.is_mine(&me))
                .map(|(tank, _)| tank.get_id().clone());

            let grid_size = ground_map.get_grid_size();
            let building_blocking_cells: HashSet<TileCoord> = q_buildings
                .iter()
                .map(|building| building.get_building_tiles())
                .flatten()
                .collect();
            let ground_blocking_cells = ground_map.get_blocking_cells();
            let all_blocking_cells = building_blocking_cells
                .union(&ground_blocking_cells)
                .cloned()
                .collect();

            if let Some(goal) = find_accessible_tile_coord(&q_tiles, &world_coords) {
                let selected_tanks = &mut q_tanks
                    .iter_mut()
                    .filter(|(tank, _)| tank.selected)
                    .map(|(tank, _)| tank);

                for mut tank in selected_tanks {
                    if let Some(start) =
                        // TODO: why not use translation instead of target_position?
                        find_accessible_tile_coord(&q_tiles, &tank.target_position)
                    {
                        // TODO: might be expensive; optimize this
                        let world_path: VecDeque<WorldCoord> =
                            compute_astar_path(grid_size, start, goal, &all_blocking_cells)
                                .iter()
                                .filter_map(|&key| {
                                    ground_map.get_tile_to_world_coordinates().get(&key)
                                }) // Use `get` to lookup each key in the map, filter_map to ignore None results
                                .cloned() // Clone the (f32, f32) values to move them into the Vec
                                .collect();

                        tank.set_movement_path(world_path);
                    }
                }

                q_tanks
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
