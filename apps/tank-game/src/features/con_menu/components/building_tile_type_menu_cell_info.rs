use crate::features::building::types::BuildingTileType;
use bevy::prelude::Component;

#[derive(Debug, Clone, Component)]
pub struct BuildingTileTypeMenuCellInfo {
    building_type: BuildingTileType,
}

impl BuildingTileTypeMenuCellInfo {
    pub fn new(building_type: BuildingTileType) -> Self {
        Self { building_type }
    }

    pub fn get_building_tile_type(&self) -> BuildingTileType {
        self.building_type.clone()
    }
}
