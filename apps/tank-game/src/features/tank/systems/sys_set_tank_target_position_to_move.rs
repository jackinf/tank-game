use crate::actions::calculate_astar_path::calculate_astar_path;
use crate::actions::calculate_tile_world_position::{
    calculate_tile_to_world_position, calculate_world_to_tile_position,
};
use crate::actions::get_all_blocking_cells::get_all_blocking_cells;
use crate::features::building::components::Building;
use crate::features::con_menu::MenuInfo;
use crate::features::cursor::CursorCoordinates;
use crate::features::tank::components::Tank;
use crate::features::tile::{find_accessible_tile_coord, Tile};
use crate::features::unit::UnitId;
use crate::resources::mission_info_resource::MissionInfoResource;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::math::Vec2;
use bevy::prelude::{
    EventReader, KeyCode, MouseButton, Query, Res, Sprite, Transform, Vec3Swizzles, With,
};
use std::collections::{HashSet, VecDeque};

pub fn sys_set_tank_target_position_to_move(
    mut q_tanks: Query<(&mut Tank, &mut Sprite), With<Tank>>,
    q_tiles: Query<&Tile>,
    mut q_buildings: Query<&Building>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut key_button_events: EventReader<KeyboardInput>,
    my_world_coords: Res<CursorCoordinates>,
    q_menu_info: Query<&MenuInfo>,
    mission_info_resource: Res<MissionInfoResource>,
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
            let me = q_menu_info.single();
            let world_coords = my_world_coords.get_world();
            let coord = calculate_world_to_tile_position(&world_coords);
            let wx = world_coords.x;
            let wy = world_coords.y;

            let mut clicked_on_enemy_unit_id: Option<UnitId> = q_tanks
                .iter_mut()
                .find(|(tank, _)| tank.is_clicked_on(wx, wy) && !tank.is_mine(&me))
                .map(|(tank, _)| tank.get_id().clone());

            if clicked_on_enemy_unit_id.is_none() {
                clicked_on_enemy_unit_id = q_buildings
                    .iter_mut()
                    .find(|building| building.contains(coord) && !building.is_mine(&me))
                    .map(|building| building.id().clone());
            }

            let grid_size = mission_info_resource.get_grid_size();
            let all_blocking_cells = get_all_blocking_cells(&q_tiles, &q_buildings);

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
                        let world_path: VecDeque<Vec2> =
                            // TODO: might be expensive; optimize this
                            calculate_astar_path(grid_size, start, goal, &all_blocking_cells)
                                .iter()
                                .map(|&key| calculate_tile_to_world_position(&key))
                                .collect();

                        tank.set_movement_path(world_path);
                    }
                }

                q_tanks
                    .iter_mut()
                    .filter(|(tank, _)| tank.is_mine(&me) && tank.selected)
                    .map(|(tank, _)| tank)
                    .for_each(|mut my_selected_tank| {
                        match &clicked_on_enemy_unit_id {
                            Some(unit_id) => {
                                my_selected_tank.set_target(Some(unit_id.clone()));
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
