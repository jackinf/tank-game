use crate::cursor::managers::camera_manager::CameraManager;
use crate::cursor::managers::cursor_manager::CursorManager;
use crate::cursor::resources::click_info::ClickInfo;
use crate::cursor::resources::cursor_coordinates::CursorCoordinates;
use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorCoordinates(Vec2::new(0.0, 0.0)))
            .insert_resource(ClickInfo::new(None))
            .add_systems(Update, CursorManager::convert_cursor_to_world_position)
            .add_systems(PreStartup, CameraManager::spawn_camera)
            .add_systems(Update, CameraManager::move_camera_with_keys)
            .add_systems(FixedUpdate, CameraManager::move_camera_with_cursor);
    }
}
