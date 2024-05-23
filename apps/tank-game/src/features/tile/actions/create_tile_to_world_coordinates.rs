use crate::constants::{TileCoord, WorldCoord};
use crate::features::tile::GroundLayer;
use bevy::math::Vec2;
use std::collections::HashMap;

pub fn create_tile_to_world_coordinates(
    ground_layer: &GroundLayer,
    calculate_world_position: fn(&TileCoord) -> Vec2,
) -> HashMap<TileCoord, WorldCoord> {
    let mut tile_to_world_coordinates = HashMap::new();
    ground_layer.get_tiles().into_iter().for_each(|(coord, _)| {
        let pos = calculate_world_position(coord);
        tile_to_world_coordinates.insert(coord.clone(), (pos.x, pos.y));
    });
    tile_to_world_coordinates
}
