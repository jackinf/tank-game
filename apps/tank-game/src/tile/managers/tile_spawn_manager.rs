use crate::common::constants::{RawGrid, TileCoord, TileGrid};
use crate::tile::managers::tile_manager::TileManager;
use crate::tile::tile_type::GroundTile;
use bevy::prelude::{AssetServer, Commands, Res, Vec2};
use std::collections::HashMap;
use crate::preparation::load_mission::{GroundLayer};

#[derive(Debug)]
pub enum TileSpawnManagerErrors {
    TileSpawnError,
}

pub struct TileSpawnManager;

impl TileSpawnManager {
    pub fn spawn_tiles(
        mut commands: &mut Commands,
        assets: &Res<AssetServer>,
        tile_map: GroundLayer,
        grid_to_tilemap: &mut HashMap<TileCoord, (f32, f32)>,
        calculate_world_position: fn(usize, usize) -> Vec2,
    ) -> Result<TileGrid, TileSpawnManagerErrors> {
        let grid = tile_map
            .get_grid()
            .into_iter()
            .enumerate()
            .map(|(row_index, row_on_row)| {
                row_on_row
                    .into_iter()
                    .enumerate()
                    .map(|(col_index, cell)| {
                        let pos = calculate_world_position(row_index, col_index);
                        let map_coord = (row_index, col_index);
                        grid_to_tilemap.insert(map_coord, (pos.x, pos.y));

                        // TODO: Consider a more appropriate error message instead of unwrap
                        let tile_type = GroundTile::try_from(cell).unwrap();
                        TileManager::spawn_tile(&mut commands, &assets, pos, tile_type, map_coord)
                    })
                    .collect()
            })
            .collect();

        Ok(grid)
    }
}
