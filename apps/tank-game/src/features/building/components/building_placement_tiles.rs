use crate::features::building::types::BuildingTile;
use bevy::prelude::Component;

#[derive(Component)]
pub struct BuildingPlacementTiles {
    layout: (usize, usize),
    building_tile: Option<BuildingTile>,
}

impl BuildingPlacementTiles {
    pub fn new() -> Self {
        Self {
            layout: (2, 2),
            building_tile: None,
        }
    }

    pub fn get_layout(&self) -> (usize, usize) {
        self.layout
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
}
