use crate::cursor::cursor_coordinates::WorldCoordinates;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCoordinates(Vec2::new(0.0, 0.0)))
            .add_systems(Update, (track_cursor, move_camera));
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

fn move_camera(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    // q_camera: Query<(&Camera, &mut GlobalTransform)>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    let (mut camera_transform) = q_camera.single_mut();

    if keyboard.just_pressed(KeyCode::KeyA) {
        camera_transform.translation.x -= 100.0;
    }
    if keyboard.just_pressed(KeyCode::KeyD) {
        camera_transform.translation.x += 100.0;
    }
    if keyboard.just_pressed(KeyCode::KeyW) {
        camera_transform.translation.y += 100.0;
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        camera_transform.translation.y -= 100.0;
    }
}