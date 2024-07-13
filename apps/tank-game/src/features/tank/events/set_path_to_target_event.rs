use crate::features::unit::UnitId;
use bevy::prelude::Event;

#[derive(Event)]
pub struct SetPathToTargetEvent {
    pub source: UnitId,
    pub target: UnitId,
}
