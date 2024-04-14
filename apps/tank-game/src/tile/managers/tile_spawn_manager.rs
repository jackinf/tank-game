use crate::common::constants::{RawGrid, TileCoord, TileGrid};
use crate::common::utils::enum_helpers::EnumHelpers;
use crate::tile::managers::tile_manager::TileManager;
use crate::tile::tile_type::TileType;
use bevy::prelude::{AssetServer, Commands, Res, Vec2};
use std::collections::HashMap;

pub struct TileSpawnManager;

impl TileSpawnManager {
    pub fn spawn_tiles(
        mut commands: &mut Commands,
        assets: &Res<AssetServer>,
        tile_map: RawGrid,
        mut grid_to_tilemap: &mut HashMap<TileCoord, (f32, f32)>,
        calculate_world_position: fn(usize, usize) -> Vec2,
    ) -> TileGrid {
        let grid = tile_map
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

                        TileManager::spawn_tile(
                            &mut commands,
                            &assets,
                            pos,
                            TileType::Grass,
                            map_coord,
                        );

                        let tile_type = EnumHelpers::assert_valid_enum::<TileType>(cell).unwrap();
                        TileManager::spawn_tile(&mut commands, &assets, pos, tile_type, map_coord)
                    })
                    .collect()
            })
            .collect();
        grid
    }
}
