use crate::features::unit::UnitTileType;
use bevy::prelude::Component;

#[derive(Debug, Clone, Component)]
pub struct UnitTileTypeMenuCellInfo {
    unit_type: UnitTileType,
}

impl UnitTileTypeMenuCellInfo {
    pub fn new(unit_type: UnitTileType) -> Self {
        Self { unit_type }
    }

    pub fn unit_type(&self) -> UnitTileType {
        self.unit_type.clone()
    }
}
