use crate::common::tile::Tile;
use bevy::prelude::Resource;
use std::collections::HashMap;

#[derive(Resource)]
pub struct GameMap(pub Vec<Vec<usize>>, pub HashMap<(usize, usize), (f32, f32)>);

impl GameMap {
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

    pub fn get_min_max(&self) -> (f32, f32, f32, f32) {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for (_, value) in &self.1 {
            if value.0 < min_x {
                min_x = value.0;
            }
            if value.0 > max_x {
                max_x = value.0;
            }
            if value.1 < min_y {
                min_y = value.1;
            }
            if value.1 > max_y {
                max_y = value.1;
            }
        }

        (min_x, max_x, min_y, max_y)
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
