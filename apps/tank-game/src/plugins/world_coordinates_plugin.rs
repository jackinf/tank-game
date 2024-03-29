use crate::resources::WorldCoordinates;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct WorldCoordinatesPlugin;

impl Plugin for WorldCoordinatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCoordinates(Vec2::new(0.0, 0.0)))
            .add_systems(Update, track_cursor);
    }
}

fn track_cursor(
    mut my_world_coords: ResMut<WorldCoordinates>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.xy())
    {
        my_world_coords.0 = world_position;
    }
}
