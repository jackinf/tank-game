use crate::constants::{TileCoord, TileGrid, WorldCoord};
use bevy::prelude::Resource;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct GoldMap {
    grid: TileGrid,
    tile_to_world_coordinates: HashMap<TileCoord, WorldCoord>,
}

impl GoldMap {
    pub fn new() -> Self {
        GoldMap {
            grid: vec![],
            tile_to_world_coordinates: HashMap::new(),
        }
    }

    pub fn get_grid(&self) -> &TileGrid {
        &self.grid
    }

    pub fn get_tile_type_grid_i32(&self) -> Vec<Vec<i32>> {
        self.grid
            .iter()
            .map(|row| row.iter().map(|tile| *tile as i32).collect())
            .collect()
    }

    pub fn get_tile_type_grid_usize(&self) -> Vec<Vec<usize>> {
        self.grid
            .iter()
            .map(|row| row.iter().map(|tile| *tile as usize).collect())
            .collect()
    }

    pub fn get_tile_to_world_coordinates(&self) -> &HashMap<TileCoord, WorldCoord> {
        &self.tile_to_world_coordinates
    }

    pub fn set_map(
        &mut self,
        grid: TileGrid,
        tile_to_world_coordinates: HashMap<TileCoord, WorldCoord>,
    ) {
        self.grid = grid;
        self.tile_to_world_coordinates = tile_to_world_coordinates;
    }

    #[cfg(test)]
    pub fn draw_map(&self) {
        self.grid
            .iter()
            .enumerate()
            .for_each(|(_col_index, row_on_row)| {
                row_on_row
                    .iter()
                    .enumerate()
                    .for_each(|(_row_index, cell)| {
                        print!("{}, ", cell);
                    });
                println!()
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::TileGrid;
    use crate::features::tile::Tile;
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

        let tiles: TileGrid = grid
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

        let game_map = GoldMap {
            grid: tiles,
            tile_to_world_coordinates: HashMap::new(),
        };

        game_map.draw_map();
    }
}
