use bevy::ecs::message::Message;
use bevy::prelude::Vec2;

#[derive(Message)]
pub struct TriggerExplosionAnimationEvent {
    at: Vec2,
    scale: f32,
}

impl TriggerExplosionAnimationEvent {
    pub fn new(at: Vec2, scale: f32) -> Self {
        TriggerExplosionAnimationEvent { at, scale }
    }

    pub fn at(&self) -> Vec2 {
        self.at
    }
    pub fn scale(&self) -> f32 {
        self.scale
    }
}
