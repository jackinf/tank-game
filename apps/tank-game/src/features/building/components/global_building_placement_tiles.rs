use crate::constants::TileCoord;
use crate::features::building::types::BuildingTile;
use bevy::prelude::Component;
use std::collections::HashSet;

/// User's global building placement tiles that is single for the whole game.
/// This component is the aid for the user when they decide to place a building.
#[derive(Component)]
pub struct GlobalBuildingPlacementTiles {
    building_tile: Option<BuildingTile>, // todo: refactor to BuildingTileType
}

impl GlobalBuildingPlacementTiles {
    pub fn new() -> Self {
        Self {
            building_tile: None,
        }
    }

    pub fn set_ready(&mut self, building_type: Option<BuildingTile>) {
        self.building_tile = building_type;
    }

    pub fn is_ready(&self) -> bool {
        self.building_tile.is_some()
    }

    pub fn get_building_tile(&self) -> Option<BuildingTile> {
        self.building_tile.clone()
    }

    pub fn get_placement_tiles(&self, at: &TileCoord) -> HashSet<TileCoord> {
        match &self.building_tile {
            Some(building_tile) => {
                let mut tiles = HashSet::new();
                let (at_x, at_y) = at;
                let (building_width, building_height) = building_tile.get_size();
                let max_width = at_x + building_width;
                let max_height = at_y + building_height;

                for x in *at_x..max_width {
                    for y in *at_y..max_height {
                        tiles.insert((x, y));
                    }
                }
                tiles
            }
            None => HashSet::new(),
        }
    }
}
