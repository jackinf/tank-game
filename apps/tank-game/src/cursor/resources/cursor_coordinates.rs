use crate::common::constants::TileCoord;
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

    pub fn set_tile(&mut self, tile: Option<TileCoord>) {
        self.tile = tile;
    }

    pub fn get_world(&self) -> Vec2 {
        self.world
    }

    pub fn get_tile(&self) -> Option<TileCoord> {
        self.tile
    }
}
