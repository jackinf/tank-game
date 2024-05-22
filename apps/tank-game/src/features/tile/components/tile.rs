use std::fmt::Formatter;

use bevy::prelude::*;

use crate::constants::TileCoord;
use crate::features::tile::ground_tile::GroundTileType;

#[derive(Component, Debug, Clone)]
pub struct Tile {
    center: Vec2,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    tile_type: GroundTileType,
    tile_coord: TileCoord,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile: {:?}, {:?}", self.tile_coord, self.tile_type)
    }
}

impl Tile {
    pub fn new(
        center: Vec2,
        width: f32,
        height: f32,
        ground_type: GroundTileType,
        map_coord: (usize, usize),
    ) -> Self {
        let x1 = center.x - width / 2.0;
        let x2 = center.x + width / 2.0;
        let y1 = center.y - height / 2.0;
        let y2 = center.y + height / 2.0;
        Tile {
            center,
            x1,
            x2,
            y1,
            y2,
            tile_type: ground_type,
            tile_coord: map_coord,
        }
    }

    pub fn accessible(&self) -> bool {
        self.tile_type == GroundTileType::Grass
    }

    pub fn in_range(&self, x: f32, y: f32) -> bool {
        let in_x = self.x1 <= x && x <= self.x2;
        let in_y = self.y1 <= y && y <= self.y2;
        in_x && in_y
    }

    pub fn get_tile_type(&self) -> GroundTileType {
        self.tile_type.clone()
    }

    pub fn get_tile_coord(&self) -> (usize, usize) {
        self.tile_coord
    }
    pub fn get_world_coord(&self) -> (f32, f32) {
        (self.x2, self.y2)
    }

    pub fn get_center(&self) -> Vec2 {
        self.center
    }
}
