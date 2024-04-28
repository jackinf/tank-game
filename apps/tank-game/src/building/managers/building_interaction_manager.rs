use crate::building::components::building::Building;
use crate::common::resources::game_map::GameMap;
use crate::common::utils::logger::Logger;
use crate::con_menu::components::submenu_info::SubMenuInfo;
use crate::cursor::resources::cursor_coordinates::CursorCoordinates;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{EventReader, MouseButton, Query, Res, Visibility, With};

pub struct BuildingInteractionManager;

impl BuildingInteractionManager {
    pub fn new() -> Self {
        BuildingInteractionManager
    }

    pub fn interact(
        mut mouse_button_events: EventReader<MouseButtonInput>,
        q_buildings: Query<&Building>,
        mut q_sub_menu_info: Query<(&mut Visibility, &SubMenuInfo), With<SubMenuInfo>>,
        my_world_coords: Res<CursorCoordinates>,
        game_map: Res<GameMap>,
    ) {
        for mouse_button_event in mouse_button_events.read() {
            if MouseButton::Left == mouse_button_event.button
                && mouse_button_event.state == ButtonState::Pressed
            {
                for (mut visibility, _) in q_sub_menu_info.iter_mut() {
                    *visibility = Visibility::Hidden;
                }

                for building in q_buildings.iter() {
                    let found_sub_menu_type = building.get_building_type().get_sub_menu_type();
                    if found_sub_menu_type.is_none() {
                        continue;
                    }
                    let found_sub_menu_type = found_sub_menu_type.unwrap();

                    let tile_coord_start = building.get_building_tile_coord();
                    let tile_coord_end = building.get_building_tile_end_coord();
                    let world_coord_start = game_map
                        .get_tile_to_world_coordinates()
                        .get(&tile_coord_start);
                    let world_coord_end = game_map
                        .get_tile_to_world_coordinates()
                        .get(&tile_coord_end);
                    if world_coord_start.is_none() || world_coord_end.is_none() {
                        continue;
                    }
                    let world_coord_start = world_coord_start.unwrap();
                    let world_coord_end = world_coord_end.unwrap();
                    let world_coord_curr = my_world_coords.0;

                    let is_between_x = world_coord_curr.x >= world_coord_start.0
                        && world_coord_curr.x <= world_coord_end.0;
                    let is_between_y = world_coord_curr.y >= world_coord_start.1
                        && world_coord_curr.y <= world_coord_end.1;
                    println!("===========");
                    println!("world_coord_start: {:?}", world_coord_start);
                    println!("world_coord_end: {:?}", world_coord_end);
                    println!("world_coord_curr: {:?}", world_coord_curr);
                    println!(
                        "is_between_x: {}, is_between_y: {}",
                        is_between_x, is_between_y
                    );

                    // find between
                    if is_between_x && is_between_y {
                        Logger::log("BuildingInteractionManager::interact 3");
                        for (mut visibility, sub_menu_info) in q_sub_menu_info.iter_mut() {
                            if sub_menu_info.get_sub_menu_type() == found_sub_menu_type {
                                *visibility = Visibility::Visible;
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}
