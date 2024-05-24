use crate::constants::TILE_SIZE;
use crate::features::building::components::Building;
use crate::features::cursor::resources::CursorCoordinates;
use crate::features::tank::Tank;
use bevy::prelude::{CursorIcon, Query, Res, Transform, Window, With};
use bevy::window::PrimaryWindow;

pub fn cursor_hovered_over(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    q_buildings: Query<&Building>,
    q_tanks: Query<&Transform, With<Tank>>,
    cursor_info: Res<CursorCoordinates>,
) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.icon = CursorIcon::Default;

    let cursor_world = cursor_info.get_world();
    let cursor_tile = cursor_info.get_tile();
    if cursor_tile.is_none() {
        return;
    }
    let cursor_tile = cursor_tile.unwrap();

    // check if the cursor is hovered over any building
    for building in q_buildings.iter() {
        if building.contains(cursor_tile) {
            primary_window.cursor.icon = CursorIcon::Grabbing;
            return;
        }
    }

    // check if the cursor is hovered over any tank
    for tank_transform in q_tanks.iter() {
        let tank_position = tank_transform.translation.truncate();
        if tank_position.distance(cursor_world) < TILE_SIZE / 2. {
            primary_window.cursor.icon = CursorIcon::Grabbing;
            return;
        }
    }

    // primary_window.cursor.grab_mode = CursorGrabMode::Confined;
    // primary_window.cursor.visible = false;
}
