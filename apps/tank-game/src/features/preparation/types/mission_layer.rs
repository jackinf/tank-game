use crate::constants::TileCoord;
use crate::features::preparation::types::AssetTile;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MissionLayer {
    tiles: HashMap<TileCoord, AssetTile>,
    width: usize,
    height: usize,
}

impl MissionLayer {
    pub fn new() -> Self {
        MissionLayer {
            tiles: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn from(tiles: HashMap<TileCoord, AssetTile>, width: usize, height: usize) -> Self {
        MissionLayer {
            tiles,
            width,
            height,
        }
    }

    pub fn get_tiles(&self) -> &HashMap<TileCoord, AssetTile> {
        &self.tiles
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
