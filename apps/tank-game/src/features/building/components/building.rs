use std::collections::HashSet;

use bevy::prelude::Component;

use crate::constants::{TileCoord, TileSize};
use crate::features::building::types::{BuildingTile, BuildingTileType};
use crate::types::player::Player;

#[derive(Component, Clone)]
pub struct Building {
    building_tile: BuildingTile,
    building_tile_coord: TileCoord,
    building_tiles: HashSet<TileCoord>,
    player: Option<Player>,
}

impl Building {
    pub fn new(
        building_tile: BuildingTile,
        building_tile_coord: TileCoord,
        player: Option<Player>,
    ) -> Self {
        let building_tiles: HashSet<TileCoord> =
            calculate_all_building_tiles(building_tile_coord, building_tile.get_size());

        Building {
            building_tile,
            building_tile_coord,
            building_tiles,
            player,
        }
    }

    pub fn get_building_tile(&self) -> BuildingTile {
        self.building_tile.clone()
    }

    pub fn get_building_tile_type(&self) -> BuildingTileType {
        self.building_tile.get_building_type()
    }

    pub fn get_building_tile_coord(&self) -> TileCoord {
        self.building_tile_coord
    }

    pub fn get_player(&self) -> Option<Player> {
        self.player.clone()
    }

    pub fn contains(&self, current: TileCoord) -> bool {
        self.building_tiles.contains(&current)
    }

    pub fn is_power_plant(&self) -> bool {
        self.building_tile.get_building_type() == BuildingTileType::PowerPlant
    }
}

fn calculate_all_building_tiles(start: TileCoord, size: TileSize) -> HashSet<TileCoord> {
    let (start_x, start_y) = start;
    let (width, height) = size;

    let mut building_tiles = HashSet::new();
    for x in start_x..start_x + width {
        for y in start_y..start_y + height {
            building_tiles.insert((x, y));
        }
    }

    building_tiles
}
