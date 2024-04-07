use crate::common::constants::TileCoord;
use crate::common::tile::Tile;
use bevy::prelude::{Query, Vec2};

pub struct TileQueries;

impl TileQueries {
    pub fn find_accessible(q_tiles: &Query<&Tile>, pos: &Vec2) -> Option<TileCoord> {
        q_tiles
            .iter()
            .find(|tile| tile.in_range(pos.x, pos.y) && tile.accessible())
            .map(|tile| tile.get_tile_coord())
    }
}
