use crate::features::cursor::resources::CursorCoordinates;
use crate::features::tile::{find_accessible_tile_coord, Tile};
use bevy::prelude::{Camera, GlobalTransform, Query, ResMut, Vec3Swizzles, Window, With};
use bevy::window::PrimaryWindow;

pub fn convert_cursor_to_world_position(
    mut my_world_coords: ResMut<CursorCoordinates>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.xy())
    {
        my_world_coords.set_world(world_position);
    }
}
