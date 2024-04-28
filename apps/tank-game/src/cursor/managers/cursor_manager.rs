use crate::cursor::resources::cursor_coordinates::CursorCoordinates;
use crate::tile::components::tile::Tile;
use crate::tile::tile_queries::TileQueries;
use bevy::prelude::{Camera, GlobalTransform, Query, Res, ResMut, Vec3Swizzles, Window, With};
use bevy::window::PrimaryWindow;

pub struct CursorManager;

impl CursorManager {
    pub fn convert_cursor_to_world_position(
        mut my_world_coords: ResMut<CursorCoordinates>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
        q_tiles: Query<&Tile>,
    ) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.xy())
        {
            my_world_coords.set_world(world_position);

            TileQueries::find_accessible(&q_tiles, &world_position)
                .map(|tile_coord| {
                    my_world_coords.set_tile(Some(tile_coord));
                })
                .or_else(|| {
                    my_world_coords.set_tile(None);
                    None
                });
        }
    }
}
