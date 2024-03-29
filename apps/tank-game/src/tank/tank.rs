use crate::tank::tank_id::TankId;
use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Tank {
    pub id: TankId,
    pub selected: bool,
    pub target_position: Vec2,
    pub speed: f32, // Units per second
    pub moving: bool,
}

impl Tank {
    pub fn new(id: usize, target_position: Vec2) -> Self {
        Tank {
            id: TankId(id),
            selected: false,
            target_position,
            speed: 500.0,
            moving: false,
        }
    }

    pub fn start_moving_to(&mut self, target_position: Vec2) {
        self.target_position = target_position;
        self.moving = true;
    }

    pub fn stop(&mut self) {
        self.moving = false;
    }
}
