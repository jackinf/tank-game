use crate::features::building::components::Building;
use crate::features::con_menu::SubMenuInfo;
use crate::features::cursor::CursorCoordinates;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{EventReader, MouseButton, Query, Res, Visibility, With};

pub fn sys_interact_with_building(
    mut mouse_button_events: EventReader<MouseButtonInput>,
    q_buildings: Query<&Building>,
    mut q_sub_menu_info: Query<(&mut Visibility, &SubMenuInfo), With<SubMenuInfo>>,
    cursor_info: Res<CursorCoordinates>,
) {
    let tile_coord = cursor_info.get_tile();
    if tile_coord.is_none() {
        return;
    }

    for mouse_button_event in mouse_button_events.read() {
        if MouseButton::Left == mouse_button_event.button
            && mouse_button_event.state == ButtonState::Pressed
        {
            for (mut visibility, _) in q_sub_menu_info.iter_mut() {
                *visibility = Visibility::Hidden;
            }

            for building in q_buildings.iter() {
                let found_sub_menu_type = building.get_building_tile().get_sub_menu_type();
                if found_sub_menu_type.is_none() {
                    continue;
                }
                let found_sub_menu_type = found_sub_menu_type.unwrap();
                let tile_coord = tile_coord.unwrap(); // safe to unwrap till if-condition above
                let building_clicked = building.contains(tile_coord);

                if building_clicked {
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
