use crate::actions::calculate_tile_world_position::calculate_world_to_tile_position;
use crate::constants::{TileCoord, TILE_SIZE};
use bevy::math::Vec2;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct CursorCoordinates {
    pub world: Vec2,
    pub tile: Option<TileCoord>,
}

impl CursorCoordinates {
    pub fn new() -> Self {
        CursorCoordinates {
            world: Vec2::new(0.0, 0.0),
            tile: Some((0, 0)),
        }
    }

    pub fn set_world(&mut self, world: Vec2) {
        self.world = world;
    }

    pub fn get_world(&self) -> Vec2 {
        self.world
    }

    pub fn get_tile(&self) -> Option<TileCoord> {
        Some(calculate_world_to_tile_position(&self.world))
    }
}
