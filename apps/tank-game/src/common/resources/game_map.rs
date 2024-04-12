use crate::common::constants::{Grid, TileCoord, WorldCoord};
use bevy::prelude::Resource;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct GameMap {
    grid: Grid,
    tile_to_world_coordinates: HashMap<(usize, usize), (f32, f32)>,
}

impl GameMap {
    pub fn new() -> Self {
        GameMap {
            grid: vec![],
            tile_to_world_coordinates: HashMap::new(),
        }
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    pub fn get_tile_type_grid(&self) -> Vec<Vec<usize>> {
        self.grid
            .iter()
            .map(|row| row.iter().map(|tile| tile.get_tile_type()).collect())
            .collect()
    }

    pub fn get_tile_to_world_coordinates(&self) -> &HashMap<TileCoord, WorldCoord> {
        &self.tile_to_world_coordinates
    }

    pub fn set_map(
        &mut self,
        grid: Grid,
        tile_to_world_coordinates: HashMap<TileCoord, WorldCoord>,
    ) {
        self.grid = grid;
        self.tile_to_world_coordinates = tile_to_world_coordinates;
    }

    pub fn draw_map(&self) {
        self.grid
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

        for (_, value) in &self.tile_to_world_coordinates {
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
    use bevy::prelude::Vec2;

    #[test]
    fn test_draw_map() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 2, 0, 0, 0],
            vec![0, 0, 0, 0, 2, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 2, 0, 0, 0],
            vec![0, 0, 0, 0, 2, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0],
        ];

        let tiles: Grid = grid
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_index, &cell)| {
                        Tile::new(
                            Vec2::new(row_index as f32, col_index as f32),
                            10.0,
                            10.0,
                            cell,
                            (row_index, col_index),
                        )
                    })
                    .collect()
            })
            .collect();

        let game_map = GameMap {
            grid: tiles,
            tile_to_world_coordinates: HashMap::new(),
        };

        game_map.draw_map();
    }
}
