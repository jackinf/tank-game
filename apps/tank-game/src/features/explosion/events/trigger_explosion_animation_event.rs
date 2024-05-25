use bevy::prelude::{Event, Vec2};

#[derive(Event)]
pub struct TriggerExplosionAnimationEvent {
    at: Vec2,
}

impl TriggerExplosionAnimationEvent {
    pub fn new(at: Vec2) -> Self {
        TriggerExplosionAnimationEvent { at }
    }

    pub fn at(&self) -> Vec2 {
        self.at
    }
}
