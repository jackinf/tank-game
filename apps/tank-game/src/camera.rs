//! The RTS camera: a 2D camera that pans with the keyboard / screen edges and
//! zooms with the keyboard.

use crate::config::*;
use crate::grid::GameMap;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (pan_camera, zoom_camera));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn pan_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    map: Option<Res<GameMap>>,
    mut cam: Query<(&mut Transform, &Projection), With<MainCamera>>,
) {
    let Ok((mut transform, projection)) = cam.single_mut() else {
        return;
    };

    let mut dir = Vec2::ZERO;
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        dir.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        dir.x += 1.0;
    }
    if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        dir.y += 1.0;
    }
    if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        dir.y -= 1.0;
    }

    // Screen-edge scrolling.
    if let Ok(window) = windows.single() {
        if let Some(pos) = window.cursor_position() {
            let w = window.width();
            let h = window.height();
            if pos.x < CAMERA_EDGE_MARGIN {
                dir.x -= 1.0;
            }
            if pos.x > w - CAMERA_EDGE_MARGIN {
                dir.x += 1.0;
            }
            // Screen-space y is top-down; world y is bottom-up.
            if pos.y < CAMERA_EDGE_MARGIN {
                dir.y += 1.0;
            }
            if pos.y > h - CAMERA_EDGE_MARGIN {
                dir.y -= 1.0;
            }
        }
    }

    let scale = match projection {
        Projection::Orthographic(o) => o.scale,
        _ => 1.0,
    };

    if dir != Vec2::ZERO {
        let delta = dir.normalize() * CAMERA_PAN_SPEED * scale * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }

    // Clamp to map bounds.
    if let Some(map) = map {
        let (min, max) = map.world_bounds();
        let margin = TILE * 4.0;
        transform.translation.x = transform
            .translation
            .x
            .clamp(min.x - margin, max.x + margin);
        transform.translation.y = transform
            .translation
            .y
            .clamp(min.y - margin, max.y + margin);
    }
}

fn zoom_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cam: Query<&mut Projection, With<MainCamera>>,
) {
    let Ok(mut projection) = cam.single_mut() else {
        return;
    };
    let Projection::Orthographic(ref mut ortho) = *projection else {
        return;
    };

    let mut delta = 0.0;
    if keys.any_pressed([KeyCode::Minus, KeyCode::NumpadSubtract]) {
        delta += 1.0;
    }
    if keys.any_pressed([KeyCode::Equal, KeyCode::NumpadAdd]) {
        delta -= 1.0;
    }
    if delta != 0.0 {
        ortho.scale = (ortho.scale + delta * CAMERA_ZOOM_SPEED * time.delta_secs())
            .clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);
    }
}
