use crate::constants::TileCoord;
use crate::features::tile::Tile;
use bevy::prelude::Query;

pub fn find_tile(q_tiles: &Query<&Tile>, pos: TileCoord) -> Option<Tile> {
    q_tiles
        .iter()
        .find(|tile| {
            let (x, y) = tile.get_tile_coord();
            x == pos.0 && y == pos.1
        })
        .map(|tile| tile.clone())
}
