use bevy::math::Vec2;
use bevy::prelude::{Resource, Timer};

#[derive(Resource, Default)]
pub struct WorldCoordinates(pub Vec2);

#[derive(Resource)]
pub struct TankLogTimer(pub Timer);

#[derive(Resource)]
pub struct TankIdCounter(pub usize);
