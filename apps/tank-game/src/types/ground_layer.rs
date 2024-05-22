use crate::constants::{TileCoord, TileGrid};
use crate::features::tile::ground_tile::{GroundTile, GroundTileType};
use crate::types::mission_layer::MissionLayer;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GroundLayer {
    tiles: HashMap<TileCoord, GroundTile>,
    pub height: usize,
    pub width: usize,
}

impl GroundLayer {
    pub fn new() -> Self {
        GroundLayer {
            tiles: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn get_tiles(&self) -> &HashMap<TileCoord, GroundTile> {
        &self.tiles
    }

    pub fn get_width_height(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn to_2d_grid(&self) -> TileGrid {
        let mut grid = vec![vec![GroundTileType::Grass; self.height]; self.width];
        self.tiles.iter().for_each(|(coord, ground)| {
            grid[coord.0][coord.1] = ground.get_ground_type().clone();
        });
        grid
    }
}

impl Into<GroundLayer> for MissionLayer {
    fn into(self) -> GroundLayer {
        GroundLayer {
            tiles: self
                .get_tiles()
                .iter()
                .filter_map(|(coord, tile)| {
                    GroundTile::try_from(tile.clone())
                        .ok()
                        .map(|ground_tile| (*coord, ground_tile))
                })
                .collect(),
            width: self.get_width(),
            height: self.get_height(),
        }
    }
}
