use bevy::prelude::Component;

#[derive(Component)]
pub struct BuildingPlacementTiles {}

impl BuildingPlacementTiles {
    pub fn new() -> Self {
        Self {}
    }
}
