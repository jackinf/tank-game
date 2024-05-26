use crate::constants::{TileCoord, TileGrid, WorldCoord};
use crate::resources::map_trait::MapTrait;
use bevy::prelude::Resource;
use std::collections::{HashMap, HashSet};

#[derive(Resource, Default)]
pub struct BuildingMap {
    grid: TileGrid,
    tile_to_world_coordinates: HashMap<TileCoord, WorldCoord>,
}

impl MapTrait for BuildingMap {
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
        let mut blocking_cells: HashSet<TileCoord> = HashSet::new();

        // TODO: implement

        blocking_cells
    }
}
