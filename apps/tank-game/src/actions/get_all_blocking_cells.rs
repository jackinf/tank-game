use crate::constants::TileCoord;
use crate::features::building::components::Building;
use crate::features::tile::Tile;
use bevy::prelude::Query;
use std::collections::HashSet;

pub fn get_all_blocking_cells(
    q_tiles: &Query<&Tile>,
    q_buildings: &Query<&Building>,
) -> HashSet<TileCoord> {
    let building_blocking_cells: HashSet<TileCoord> = q_buildings
        .iter()
        .map(|building| building.get_building_tiles())
        .flatten()
        .collect();
    let ground_blocking_cells: HashSet<TileCoord> = q_tiles
        .iter()
        .filter(|tile| tile.is_blocking())
        .map(|tile| tile.get_tile_coord())
        .collect();
    let all_blocking_cells = building_blocking_cells
        .union(&ground_blocking_cells)
        .cloned()
        .collect();

    all_blocking_cells
}
