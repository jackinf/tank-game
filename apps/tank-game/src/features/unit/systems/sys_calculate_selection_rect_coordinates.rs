use crate::features::con_menu::MenuInfo;
use crate::features::cursor::CursorCoordinates;
use crate::features::tank::{deselect_all_my_units, Tank};
use crate::features::unit::components::UnitSelectionRect;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::math::Vec2;
use bevy::prelude::{EventReader, MouseButton, Query, Res, ResMut, Sprite, With};

/// while holding down left mouse button, set the start and end positions of the selection rectangle
pub fn sys_calculate_selection_rect_coordinates(
    mut q_unit_selection_rect: Query<&mut UnitSelectionRect, With<UnitSelectionRect>>,
    my_world_coords: ResMut<CursorCoordinates>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut tank_query: Query<(&mut Tank, &mut Sprite), With<Tank>>,
    q_menu_info: Query<&MenuInfo>,
) {
    for mouse_button_input_event in mouse_button_input_events.read() {
        let me = q_menu_info.single().unwrap();
        let world_coords = my_world_coords.get_world();
        let wx = world_coords.x;
        let wy = world_coords.y;

        let clicked_on_tank = tank_query
            .iter_mut()
            .find(|(tank, _)| tank.is_clicked_on(wx, wy));

        match (
            mouse_button_input_event.button,
            mouse_button_input_event.state,
            clicked_on_tank,
        ) {
            (MouseButton::Left, ButtonState::Pressed, Some((mut tank, mut sprite))) => {
                if tank.is_mine(&me) {
                    tank.toggle(&mut sprite);
                }
            }
            (MouseButton::Left, ButtonState::Pressed, None) => {
                deselect_all_my_units(&mut tank_query, &me);

                q_unit_selection_rect
                    .single_mut().unwrap()
                    .set_start(Some(Vec2::new(wx, wy)));
            }
            (MouseButton::Left, ButtonState::Released, _) => {
                let mut tank_selection_rect = q_unit_selection_rect.single_mut().unwrap();
                if tank_selection_rect.start().is_none() {
                    continue;
                }

                let sx = tank_selection_rect.start().unwrap().x;
                let sy = tank_selection_rect.start().unwrap().y;
                tank_selection_rect.set_start(None);

                // finds and selects tanks within the selection rectangle
                for (mut tank, mut sprite) in
                    &mut tank_query.iter_mut().filter(|(tank, _)| tank.is_mine(&me))
                {
                    let x1 = sx.min(wx);
                    let x2 = sx.max(wx);
                    let y1 = sy.min(wy);
                    let y2 = sy.max(wy);

                    let in_x = x1 <= tank.target_position.x && tank.target_position.x <= x2;
                    let in_y = y1 <= tank.target_position.y && tank.target_position.y <= y2;

                    if in_x && in_y {
                        tank.select(&mut sprite);
                    } else {
                        tank.deselect(&mut sprite);
                    }
                }
            }
            _ => {}
        }

        if mouse_button_input_event.button != MouseButton::Left {
            continue;
        }
    }
}
