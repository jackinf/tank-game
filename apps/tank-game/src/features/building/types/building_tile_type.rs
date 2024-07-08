use bevy::prelude::Component;

// TODO: move to "components" folder
#[derive(Clone, Debug, PartialEq, Hash, Eq, Component)]
pub enum BuildingTileType {
    Base = 10,
    Factory = 20,
    PowerPlant = 30,
}
