use crate::features::tank::TankStrategy;
use crate::types::player::Player;
use bevy::ecs::message::Message;
use bevy::prelude::Vec2;

#[derive(Message)]
pub struct SpawnTankEvent {
    pub position: Vec2,
    pub player: Player,
    pub strategy: TankStrategy,
}
