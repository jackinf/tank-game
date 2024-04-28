use crate::building::building_type::BuildingType;
use crate::common::constants::TileCoord;
use crate::common::player::Player;
use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct Building {
    building_type: BuildingType,
    building_tile_coord: TileCoord,
    player: Player,
}

impl Building {
    pub fn new(
        building_type: BuildingType,
        building_tile_coord: TileCoord,
        player: Player,
    ) -> Self {
        Building {
            building_type,
            building_tile_coord,
            player,
        }
    }

    pub fn get_building_type(&self) -> BuildingType {
        self.building_type.clone()
    }

    pub fn get_building_tile_coord(&self) -> TileCoord {
        self.building_tile_coord
    }

    pub fn get_building_tile_end_coord(&self) -> TileCoord {
        let (x, y) = self.building_tile_coord;
        let (width, height) = self.building_type.get_size();
        (x + width, y + height - 1)
    }

    pub fn get_player(&self) -> Player {
        self.player.clone()
    }
}
