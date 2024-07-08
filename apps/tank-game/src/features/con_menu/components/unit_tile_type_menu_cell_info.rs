use crate::features::unit::UnitTileType;
use bevy::prelude::Component;

#[derive(Debug, Clone, Component)]
pub struct UnitTileTypeMenuCellInfo {
    unit_tile_type: UnitTileType,
}

impl UnitTileTypeMenuCellInfo {
    pub fn new(unit_tile_type: UnitTileType) -> Self {
        Self { unit_tile_type }
    }

    pub fn unit_tile_type(&self) -> UnitTileType {
        self.unit_tile_type.clone()
    }
}
