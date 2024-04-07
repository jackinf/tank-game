use bevy::math::Vec2;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct CursorCoordinates(pub Vec2);
