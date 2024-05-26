use crate::constants::{TileCoord, TileGrid, WorldCoord};
use std::collections::HashMap;

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
}
