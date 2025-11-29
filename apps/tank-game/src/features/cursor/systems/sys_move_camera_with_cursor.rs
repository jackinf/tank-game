use crate::constants::{CAMERA_SPEED_DYNAMIC, CAMERA_SPEED_STATIC, SIDE_MARGIN_PERCENTAGE};
use crate::features::con_menu::MenuInfo;
use crate::features::cursor::resources::ClickInfo;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    Camera, EventReader, MouseButton, Query, Res, ResMut, Time, Transform, Vec2, Vec3, Window, With,
};
use bevy::window::PrimaryWindow;

pub fn sys_move_camera_with_cursor(
    time: Res<Time>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
    q_menu_info: Query<&MenuInfo>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut click_info: ResMut<ClickInfo>,
) {
    let dt = time.delta_secs();

    let mut transform = q_camera.single_mut().unwrap();
    let menu_info = q_menu_info.single().unwrap();
    if menu_info.is_hovered() {
        // Don't move the camera if the cursor is over the UI
        return;
    }

    // TODO: stop moving when on the edge of the map
    // let (min_x, max_x, min_y, max_y) = game_map.get_min_max();

    let window = q_window.single().unwrap();
    if let None = window.cursor_position() {
        return;
    }

    let cursor = window.cursor_position().unwrap();
    let cursor_x = cursor.x;
    let cursor_y = cursor.y;
    let max_width = window.width();
    let max_height = window.height();
    let side_margin_x = max_width * SIDE_MARGIN_PERCENTAGE;
    let side_margin_y = max_height * SIDE_MARGIN_PERCENTAGE;

    /*
       Priority 1: Moving camera when the right mouse button is pressed
    */
    for mouse_button_event in mouse_button_events.read() {
        match (
            mouse_button_event.button,
            mouse_button_event.state,
            click_info.get_translation(),
        ) {
            (MouseButton::Right, ButtonState::Pressed, None) => {
                click_info.set_translation(Some(Vec2::new(cursor_x, cursor_y)));
            }
            (MouseButton::Right, ButtonState::Released, Some(_)) => {
                click_info.set_translation(None);
            }
            _ => {}
        }
    }

    if let Some(tr) = click_info.get_translation() {
        let delta = tr - Vec2::new(cursor_x, cursor_y);
        let delta = Vec3::new(-delta.x, delta.y, 0.0);
        transform.translation += delta * dt * CAMERA_SPEED_DYNAMIC;
        return;
    }

    /*
       Alternative priority: Moving camera when the cursor is on the edge of the screen
    */

    let delta_x = if cursor_x < side_margin_x {
        -1.0
    } else if cursor_x > max_width - side_margin_x {
        1.0
    } else {
        0.0
    };

    let delta_y = if cursor_y < side_margin_y {
        1.0
    } else if cursor_y > max_height - side_margin_y {
        -1.0
    } else {
        0.0
    };

    let delta = Vec3::new(delta_x, delta_y, 0.0);
    transform.translation += delta * dt * CAMERA_SPEED_STATIC;
}
