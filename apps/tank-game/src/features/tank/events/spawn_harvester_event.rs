use crate::types::player::Player;
use bevy::prelude::{Event, Vec2};

#[derive(Event)]
pub struct SpawnHarvesterEvent {
    pub position: Vec2,
    pub player: Player,
}
