//! Tracks the mouse cursor in world space and whether it is over the UI.

use crate::camera::MainCamera;
use crate::config::{SIDEBAR_WIDTH, TOPBAR_HEIGHT};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CursorWorld {
    /// World-space position of the cursor.
    pub pos: Vec2,
    /// Whether `pos` is valid this frame (cursor inside the window).
    pub valid: bool,
    /// Whether the cursor is currently over a UI panel (sidebar / top bar).
    pub over_ui: bool,
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorld>()
            .add_systems(PreUpdate, update_cursor);
    }
}

fn update_cursor(
    mut cursor: ResMut<CursorWorld>,
    windows: Query<&Window>,
    cam: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    cursor.valid = false;
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = cam.single() else {
        return;
    };
    let Some(screen) = window.cursor_position() else {
        return;
    };

    cursor.over_ui = screen.x > window.width() - SIDEBAR_WIDTH || screen.y < TOPBAR_HEIGHT;

    if let Ok(world) = camera.viewport_to_world_2d(cam_transform, screen) {
        cursor.pos = world;
        cursor.valid = true;
    }
}
