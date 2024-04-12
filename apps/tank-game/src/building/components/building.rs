use crate::building::building_type::BuildingType;
use crate::common::constants::TileCoord;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Building {
    building_type: BuildingType,
    building_tile_coord: TileCoord,
}

impl Building {
    pub fn new(building_type: BuildingType, building_tile_coord: TileCoord) -> Self {
        Building {
            building_type,
            building_tile_coord,
        }
    }

    pub fn get_building_type(&self) -> BuildingType {
        self.building_type.clone()
    }

    pub fn get_building_tile_coord(&self) -> TileCoord {
        self.building_tile_coord
    }
}
