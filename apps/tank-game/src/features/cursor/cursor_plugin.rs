use crate::features::cursor::resources::ClickInfo;
use crate::features::cursor::resources::CursorCoordinates;
use crate::features::cursor::systems::{
    convert_cursor_to_world_position, cursor_debug_info, cursor_hovered_over,
    move_camera_with_cursor, move_camera_with_keys, show_cursor_coordinates_in_ui, spawn_camera,
};
use crate::AppState;
use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorCoordinates::new())
            .insert_resource(ClickInfo::new(None))
            .add_systems(PreStartup, cursor_debug_info)
            // .add_systems(PreStartup, setup_cursor)
            .add_systems(FixedUpdate, cursor_hovered_over)
            .add_systems(Update, convert_cursor_to_world_position)
            .add_systems(PreStartup, spawn_camera)
            .add_systems(Update, move_camera_with_keys)
            .add_systems(
                FixedUpdate,
                move_camera_with_cursor.run_if(in_state(AppState::Playing)),
            )
            .add_systems(Update, show_cursor_coordinates_in_ui);
    }
}
