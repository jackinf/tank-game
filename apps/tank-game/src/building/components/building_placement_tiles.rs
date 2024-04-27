use crate::building::building_type::BuildingType;
use bevy::prelude::Component;

#[derive(Component)]
pub struct BuildingPlacementTiles {
    layout: (usize, usize),
    building_type: Option<BuildingType>,
}

impl BuildingPlacementTiles {
    pub fn new() -> Self {
        Self {
            layout: (2, 2),
            building_type: None,
        }
    }

    pub fn get_layout(&self) -> (usize, usize) {
        self.layout
    }

    pub fn set_ready(&mut self, building_type: Option<BuildingType>) {
        self.building_type = building_type;
    }

    pub fn is_ready(&self) -> bool {
        self.building_type.is_some()
    }

    pub fn get_building_type(&self) -> Option<BuildingType> {
        self.building_type.clone()
    }
}
