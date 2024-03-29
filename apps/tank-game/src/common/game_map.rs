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

    pub fn draw_map(&self) {
        self.0
            .iter()
            .enumerate()
            .for_each(|(col_index, row_on_row)| {
                row_on_row.iter().enumerate().for_each(|(row_index, cell)| {
                    print!("{}, ", cell);
                });
                println!()
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_map() {
        let game_map = GameMap(
            vec![
                vec![0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0, 0, 0],
            ],
            HashMap::new(),
        );

        game_map.draw_map();
    }
}
