use bevy::prelude::Component;

// TODO: to components
#[derive(Clone, Debug, PartialEq, Hash, Eq, Component)]
pub enum UnitTileType {
    Tank = 1,
    Harvester = 3,
}
