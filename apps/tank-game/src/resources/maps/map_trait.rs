use crate::constants::{GridSize, TileCoord, TileGrid, WorldCoord};
use std::collections::{HashMap, HashSet};

pub trait MapTrait {
    fn get_grid(&self) -> &TileGrid;

    fn get_tile_type_grid_i32(&self) -> Vec<Vec<i32>> {
        self.get_grid()
            .iter()
            .map(|row| row.iter().map(|tile| *tile as i32).collect())
            .collect()
    }

    fn get_tile_type_grid_usize(&self) -> Vec<Vec<usize>> {
        self.get_grid()
            .iter()
            .map(|row| row.iter().map(|tile| *tile as usize).collect())
            .collect()
    }

    fn get_tile_to_world_coordinates(&self) -> &HashMap<TileCoord, WorldCoord>;

    fn set_map(
        &mut self,
        grid: TileGrid,
        tile_to_world_coordinates: HashMap<TileCoord, WorldCoord>,
    );

    fn draw_map(&self) {
        self.get_grid()
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

    fn get_blocking_cells(&self) -> HashSet<TileCoord>;

    fn get_grid_size(&self) -> GridSize {
        (self.get_width(), self.get_height())
    }

    fn get_width(&self) -> usize {
        self.get_grid().len()
    }

    fn get_height(&self) -> usize {
        let grid = self.get_grid();
        if grid.len() > 0 {
            grid[0].len()
        } else {
            0
        }
    }
}
