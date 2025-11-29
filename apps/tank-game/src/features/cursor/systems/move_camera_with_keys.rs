use bevy::prelude::{ButtonInput, Camera, KeyCode, Query, Res, Transform, With};

pub fn move_camera_with_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = q_camera.single_mut().unwrap();

    if keyboard.just_pressed(KeyCode::KeyA) {
        camera_transform.translation.x -= 300.0;
    }
    if keyboard.just_pressed(KeyCode::KeyD) {
        camera_transform.translation.x += 300.0;
    }
    if keyboard.just_pressed(KeyCode::KeyW) {
        camera_transform.translation.y += 300.0;
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        camera_transform.translation.y -= 300.0;
    }
}
