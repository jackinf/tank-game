use crate::common::constants::TileCoord;

pub trait Hoverable {
    fn is_hovered_over(&self, tile_coord: TileCoord) -> bool;
}

// impl Hoverable for Tank {
//     fn is_hovered_over(&self, tile_coord: TileCoord) -> bool {
//         self.
//     }
// }
