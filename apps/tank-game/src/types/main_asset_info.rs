use crate::types::{AssetTile, AssetTileId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MainAssetInfo {
    tiles: HashMap<AssetTileId, AssetTile>,
}

impl MainAssetInfo {
    pub fn new(tiles: HashMap<AssetTileId, AssetTile>) -> Self {
        MainAssetInfo { tiles }
    }

    pub fn get_tiles(&self) -> &HashMap<AssetTileId, AssetTile> {
        &self.tiles
    }
}
