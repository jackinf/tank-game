use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct TankBullet {
    speed: f32,
    damage: u32,
    destination: Vec2,
}

impl TankBullet {
    pub fn new(destination: Vec2) -> Self {
        TankBullet {
            speed: 1000.0,
            damage: 10,
            destination,
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    pub fn get_destination(&self) -> Vec2 {
        self.destination
    }
}
