use crate::common::constants::{TILE_GRASS, TILE_SIZE};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Tile {
    center: Vec2,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    tile_type: usize,
    map_coord: (usize, usize),
}

impl Tile {
    pub fn new(
        center: Vec2,
        width: f32,
        height: f32,
        tile_type: usize,
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
            tile_type,
            map_coord,
        }
    }

    pub fn accessible(&self) -> bool {
        self.tile_type == TILE_GRASS
    }

    pub fn in_range(&self, x: f32, y: f32) -> bool {
        let in_x = self.x1 <= x && x <= self.x2;
        let in_y = self.y1 <= y && y <= self.y2;
        in_x && in_y
    }

    pub fn get_map_coord(&self) -> (usize, usize) {
        self.map_coord
    }

    pub fn get_center(&self) -> Vec2 {
        self.center
    }
}
