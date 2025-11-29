use crate::features::cursor::resources::ClickInfo;
use crate::features::cursor::resources::CursorCoordinates;
use crate::features::cursor::systems::{
    sys_convert_cursor_to_world_position, sys_cursor_debug_info, sys_cursor_hovered_over,
    sys_move_camera_with_cursor, sys_move_camera_with_keys, sys_show_cursor_coordinates_in_ui,
    sys_spawn_camera,
};
use crate::AppState;
use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorCoordinates::new())
            .insert_resource(ClickInfo::new(None))
            .add_systems(PreStartup, sys_cursor_debug_info)
            // .add_systems(PreStartup, setup_cursor)
            .add_systems(FixedUpdate, sys_cursor_hovered_over)
            .add_systems(Update, sys_convert_cursor_to_world_position)
            .add_systems(PreStartup, sys_spawn_camera)
            .add_systems(Update, sys_move_camera_with_keys)
            .add_systems(
                FixedUpdate,
                sys_move_camera_with_cursor.run_if(in_state(AppState::Playing)),
            )
            .add_systems(Update, sys_show_cursor_coordinates_in_ui);
    }
}
