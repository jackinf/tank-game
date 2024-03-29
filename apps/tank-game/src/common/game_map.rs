use crate::common::tile::Tile;
use bevy::prelude::Resource;
use std::collections::HashMap;

#[derive(Resource)]
pub struct GameMap(pub Vec<Vec<usize>>, pub HashMap<(usize, usize), (f32, f32)>);

impl GameMap {
    // pub fn tiles_to_world_coords(&self) {
    //     self.1.iter().map(|(map_coord, tile)| {
    //         let (x, y) = map_coord;
    //         // let x = *x as f32 * 32.0;
    //         // let y = *y as f32 * 32.0;
    //         // let center = Vec2::new(x, y);
    //         // let tile = Tile::new(center, 32.0, 32.0, tile.tile_type, *map_coord);
    //         // self.1.insert(*map_coord, tile);
    //     });
    // }
}
