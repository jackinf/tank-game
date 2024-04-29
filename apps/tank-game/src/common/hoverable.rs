use crate::common::constants::TileCoord;
use crate::tank::components::tank::Tank;

pub trait Hoverable {
    fn is_hovered_over(&self, tile_coord: TileCoord) -> bool;
}

// impl Hoverable for Tank {
//     fn is_hovered_over(&self, tile_coord: TileCoord) -> bool {
//         self.
//     }
// }
