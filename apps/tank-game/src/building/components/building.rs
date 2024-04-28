use crate::building::building_type::BuildingType;
use crate::common::constants::{TileCoord, TileSize};
use crate::common::player::Player;
use crate::cursor::resources::cursor_coordinates::CursorCoordinates;
use bevy::prelude::Component;
use std::collections::HashSet;

#[derive(Component, Clone)]
pub struct Building {
    building_type: BuildingType,
    building_tile_coord: TileCoord,
    building_tiles: HashSet<TileCoord>,
    player: Player,
}

impl Building {
    pub fn new(
        building_type: BuildingType,
        building_tile_coord: TileCoord,
        player: Player,
    ) -> Self {
        let building_tiles =
            calculate_all_building_tiles(building_tile_coord, building_type.get_size());

        Building {
            building_type,
            building_tile_coord,
            building_tiles,
            player,
        }
    }

    pub fn get_building_type(&self) -> BuildingType {
        self.building_type.clone()
    }

    pub fn get_building_tile_coord(&self) -> TileCoord {
        self.building_tile_coord
    }

    pub fn get_player(&self) -> Player {
        self.player.clone()
    }

    pub fn is_clicked(&self, clicked_tile_coord: TileCoord) -> bool {
        self.building_tiles.contains(&clicked_tile_coord)
    }
}

fn calculate_all_building_tiles(start: TileCoord, size: TileSize) -> HashSet<TileCoord> {
    let (start_x, start_y) = start;
    let (width, height) = size;

    let mut building_tiles = HashSet::new();
    for x in start_x..start_x + width {
        for y in start_y - height + 1..start_y + 1 {
            building_tiles.insert((x, y));
        }
    }

    building_tiles
}
