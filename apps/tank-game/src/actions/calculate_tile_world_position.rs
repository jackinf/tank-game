use crate::constants::{TileCoord, TILE_SIZE};
use bevy::prelude::Vec2;

pub fn calculate_tile_to_world_position(coord: &TileCoord) -> Vec2 {
    let x = coord.0 as f32 * TILE_SIZE;
    let y = coord.1 as f32 * TILE_SIZE;
    Vec2::new(x, y)
}

pub fn calculate_world_to_tile_position(world_pos: &Vec2) -> TileCoord {
    let x = (world_pos.x / TILE_SIZE).floor() as usize;
    let y = (world_pos.y / TILE_SIZE).floor() as usize;
    (x, y)
}
