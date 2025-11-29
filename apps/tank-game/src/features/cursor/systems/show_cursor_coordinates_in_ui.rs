use crate::features::cursor::components::{TileCoordText, WorldCoordText};
use crate::features::cursor::resources::CursorCoordinates;
use bevy::prelude::{Query, Res, Text, With, Without};

pub fn show_cursor_coordinates_in_ui(
    cursor_coordinates: Res<CursorCoordinates>,
    mut q_world_coord_text: Query<&mut Text, (With<WorldCoordText>, Without<TileCoordText>)>,
    mut q_tile_coord_text: Query<&mut Text, (With<TileCoordText>, Without<WorldCoordText>)>,
) {
    let mut world_coord_text = q_world_coord_text.single_mut().unwrap();
    **world_coord_text = format!(
        "Cursor: ({:.2}, {:.2})",
        cursor_coordinates.get_world().x,
        cursor_coordinates.get_world().y
    );

    let mut tile_coord_text = q_tile_coord_text.single_mut().unwrap();
    let tile_value = if let Some(tile) = cursor_coordinates.get_tile() {
        format!("Tile: ({}, {})", tile.0, tile.1)
    } else {
        "Tile: None".to_string()
    };
    **tile_coord_text = tile_value;
}
