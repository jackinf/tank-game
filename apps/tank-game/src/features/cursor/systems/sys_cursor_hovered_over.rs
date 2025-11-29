use crate::constants::TILE_SIZE;
use crate::features::building::components::Building;
use crate::features::cursor::resources::CursorCoordinates;
use crate::features::tank::Tank;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn sys_cursor_hovered_over(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_buildings: Query<&Building>,
    q_tanks: Query<&Transform, With<Tank>>,
    cursor_info: Res<CursorCoordinates>,
) {
    let _primary_window = q_windows.single().unwrap();

    let cursor_world = cursor_info.get_world();
    let cursor_tile = cursor_info.get_tile();
    if cursor_tile.is_none() {
        return;
    }
    let cursor_tile = cursor_tile.unwrap();

    // check if the cursor is hovered over any building
    for building in q_buildings.iter() {
        if building.contains(cursor_tile) {
            // Building hovered - cursor change disabled for now (API changed in Bevy 0.15+)
            return;
        }
    }

    // check if the cursor is hovered over any tank
    for tank_transform in q_tanks.iter() {
        let tank_position = tank_transform.translation.truncate();
        if tank_position.distance(cursor_world) < TILE_SIZE / 2. {
            // Tank hovered - cursor change disabled for now (API changed in Bevy 0.15+)
            return;
        }
    }
}
