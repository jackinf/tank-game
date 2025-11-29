use crate::features::cursor::resources::CursorCoordinates;
use crate::features::tile::{find_accessible_tile_coord, Tile};
use bevy::prelude::{Camera, GlobalTransform, Query, ResMut, Vec3Swizzles, Window, With};
use bevy::window::PrimaryWindow;

pub fn sys_convert_cursor_to_world_position(
    mut my_world_coords: ResMut<CursorCoordinates>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap();

    if let Some(cursor) = window.cursor_position() {
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) {
            my_world_coords.set_world(ray.origin.xy());
        }
    }
}
