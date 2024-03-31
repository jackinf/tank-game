use crate::cursor::cursor_coordinates::WorldCoordinates;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCoordinates(Vec2::new(0.0, 0.0)))
            .add_systems(PreStartup, spawn_camera)
            .add_systems(
                Update,
                (
                    move_camera_with_cursor_p1,
                    move_camera_with_keys,
                    convert_cursor_to_world_position,
                ),
            )
            .add_systems(FixedUpdate, move_camera_with_cursor_p2);
    }
}

#[derive(Component)]
struct CameraMovement {
    speed: f32,
    direction: Vec2,
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(CameraMovement {
            speed: CAMERA_SPEED,
            direction: Vec2::ZERO,
        });
}

const CAMERA_SPEED: f32 = 10.0;
const SIDE_MARGIN_PERCENTAGE: f32 = 0.2;

fn move_camera_with_cursor_p1(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_camera: Query<&mut CameraMovement, With<Camera>>,
) {
    let (mut movement) = q_camera.single_mut();
    let window = q_window.single();

    if let Some(cursor) = window.cursor_position() {
        let cursor_x = cursor.x;
        let cursor_y = cursor.y;
        let max_width = window.width();
        let max_height = window.height();
        let side_margin_x = max_width * SIDE_MARGIN_PERCENTAGE;
        let side_margin_y = max_height * SIDE_MARGIN_PERCENTAGE;

        if cursor_x < side_margin_x {
            movement.direction.x = -1.0;
        } else if cursor_x > max_width - side_margin_x {
            movement.direction.x = 1.0;
        } else {
            movement.direction.x = 0.0;
        }

        if cursor_y < side_margin_y {
            movement.direction.y = 1.0;
        } else if cursor_y > max_height - side_margin_y {
            movement.direction.y = -1.0;
        } else {
            movement.direction.y = 0.0;
        }
    } else {
        movement.direction = Vec2::ZERO;
    }
}

fn move_camera_with_cursor_p2(
    mut q_camera: Query<(&CameraMovement, &mut Transform), With<Camera>>,
) {
    let (movement, mut camera_transform) = q_camera.single_mut();
    camera_transform.translation +=
        Vec3::new(movement.direction.x, movement.direction.y, 0.0) * movement.speed;
}

fn convert_cursor_to_world_position(
    mut my_world_coords: ResMut<WorldCoordinates>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_camera: Query<(&Camera, &GlobalTransform, &mut CameraMovement), With<Camera>>,
) {
    let (camera, camera_transform, mut movement) = q_camera.single_mut();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.xy())
    {
        my_world_coords.0 = world_position;
    }
}

fn move_camera_with_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
) {
    let (mut camera_transform) = q_camera.single_mut();

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
