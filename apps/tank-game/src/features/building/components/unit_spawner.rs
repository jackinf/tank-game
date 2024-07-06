use crate::types::player::Player;
use bevy::prelude::{Component, Timer, Vec2};

#[derive(Component)]
pub struct UnitSpawner {
    pub spawn_timer: Timer,
    pub spawn_position: Vec2,
    pub player: Option<Player>,
}
