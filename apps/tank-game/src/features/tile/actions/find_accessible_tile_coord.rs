use crate::constants::TileCoord;
use crate::features::tile::Tile;
use bevy::math::Vec2;
use bevy::prelude::Query;

pub fn find_accessible_tile_coord(q_tiles: &Query<&Tile>, pos: &Vec2) -> Option<TileCoord> {
    q_tiles
        .iter()
        .find(|tile| tile.in_range(pos.x, pos.y) && tile.accessible())
        .map(|tile| tile.get_tile_coord())
}
