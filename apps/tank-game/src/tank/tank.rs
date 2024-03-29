use crate::tank::tank_id::TankId;
use bevy::math::Vec2;
use bevy::prelude::Component;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Tank {
    pub id: TankId,
    pub selected: bool,
    pub target_position: Vec2,
    pub speed: f32, // Units per second
    pub moving: bool,
    pub movement_path: VecDeque<(f32, f32)>,
}

impl Tank {
    pub fn new(id: usize, target_position: Vec2) -> Self {
        Tank {
            id: TankId(id),
            selected: false,
            target_position,
            speed: 500.0,
            moving: false,
            movement_path: VecDeque::new(),
        }
    }

    pub fn start_moving_to(&mut self, target_position: Vec2) {
        self.target_position = target_position;
        self.moving = true;
    }

    pub fn set_movement_path(&mut self, path: VecDeque<(f32, f32)>) {
        self.moving = true;
        self.movement_path = path;
        // self.target_position = path[0];
    }

    pub fn is_moving(&self) -> bool {
        self.moving
        // self.movement_path.len() > 0
    }

    pub fn try_take_next_position_in_path(&mut self) {
        if self.movement_path.len() > 0 {
            let (x, y) = self.movement_path.pop_front().unwrap();
            self.target_position = Vec2::new(x, y);
        } else {
            self.moving = false;
        }
    }

    pub fn stop(&mut self) {
        self.moving = false;
    }
}
