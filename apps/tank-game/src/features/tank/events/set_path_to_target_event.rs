use crate::features::unit::UnitId;
use bevy::ecs::message::Message;

#[derive(Message)]
pub struct SetPathToTargetEvent {
    pub source: UnitId,
    pub target: UnitId,
}
