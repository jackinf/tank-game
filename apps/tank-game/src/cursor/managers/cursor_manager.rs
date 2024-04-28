use crate::cursor::resources::cursor_coordinates::CursorCoordinates;
use bevy::prelude::{Camera, GlobalTransform, Query, ResMut, Vec3Swizzles, Window, With};
use bevy::window::PrimaryWindow;

pub struct CursorManager;

impl CursorManager {
    pub fn convert_cursor_to_world_position(
        mut my_world_coords: ResMut<CursorCoordinates>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        mut q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    ) {
        let (camera, camera_transform) = q_camera.single_mut();
        let window = q_window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.xy())
        {
            my_world_coords.0 = world_position;
        }
    }
}
