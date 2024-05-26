use crate::constants::{TileCoord, OFFSET_X, OFFSET_Y, TILE_SIZE};
use bevy::prelude::Vec2;

pub fn calculate_world_position(coord: &TileCoord) -> Vec2 {
    let x = coord.0 as f32 * TILE_SIZE + OFFSET_X;
    let y = coord.1 as f32 * TILE_SIZE + OFFSET_Y;
    Vec2::new(x, y)
}
