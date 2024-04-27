use crate::common::constants::TileCoord;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Gold {
    value: usize,
    tile_coord: TileCoord,
}

impl Gold {
    pub fn new(value: usize, tile_coord: TileCoord) -> Self {
        Self { value, tile_coord }
    }

    pub fn reduce_value_by(&mut self, amount: usize) {
        self.value -= amount;
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn at(&self, ) -> TileCoord {
        self.tile_coord
    }
}
