use crate::constants::{TileCoord, TileGrid, WorldCoord};
use crate::features::tile::GroundTileType;
use crate::resources::map_trait::MapTrait;
use bevy::prelude::Resource;
use std::collections::{HashMap, HashSet};

#[derive(Resource, Default, Clone)]
pub struct GroundMap {
    grid: TileGrid,
    tile_to_world_coordinates: HashMap<TileCoord, WorldCoord>,
}

impl MapTrait for GroundMap {
    fn get_grid(&self) -> &TileGrid {
        &self.grid
    }

    fn get_tile_to_world_coordinates(&self) -> &HashMap<TileCoord, WorldCoord> {
        &self.tile_to_world_coordinates
    }

    fn set_map(
        &mut self,
        grid: TileGrid,
        tile_to_world_coordinates: HashMap<TileCoord, WorldCoord>,
    ) {
        self.grid = grid;
        self.tile_to_world_coordinates = tile_to_world_coordinates;
    }

    fn get_blocking_cells(&self) -> HashSet<TileCoord> {
        // traverse self.grid and return coordinates only if the value is GroundTileType::Wall or GroundTileType::Water
        let mut blocking_cells: HashSet<TileCoord> = HashSet::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == GroundTileType::Wall || *tile == GroundTileType::Water {
                    blocking_cells.insert((y, x));
                }
            }
        }

        blocking_cells
    }
}
