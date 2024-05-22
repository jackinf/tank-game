use crate::constants::TileSize;
use crate::types::asset_tile_sub_type::AssetTileSubType;
use crate::types::asset_tile_type::AssetTileType;
use crate::types::player::Player;

pub type AssetTileId = i32;
pub type AssetImagePath = String;

#[derive(Debug, Clone)]
pub struct AssetTile {
    id: AssetTileId,
    image: AssetImagePath,
    tile_size: TileSize,
    tile_type: Option<AssetTileType>,
    tile_sub_type: Option<AssetTileSubType>,
}

impl AssetTile {
    pub fn new(
        id: AssetTileId,
        image: AssetImagePath,
        tile_size: TileSize,
        tile_type: Option<AssetTileType>,
        tile_sub_type: Option<AssetTileSubType>,
    ) -> Self {
        AssetTile {
            id,
            image,
            tile_size,
            tile_type,
            tile_sub_type,
        }
    }

    pub fn get_id(&self) -> AssetTileId {
        self.id
    }

    pub fn is_id_and_type(&self, id: AssetTileId, tile_type: AssetTileType) -> bool {
        if self.tile_type.is_none() {
            return false;
        }
        self.id == id && self.tile_type.clone().unwrap() == tile_type
    }

    pub fn get_tile_size(&self) -> TileSize {
        self.tile_size.clone()
    }

    pub fn get_tile_type(&self) -> Option<AssetTileType> {
        self.tile_type.clone()
    }

    pub fn get_tile_sub_type(&self) -> Option<AssetTileSubType> {
        self.tile_sub_type.clone()
    }

    pub fn get_image_path(&self) -> String {
        self.image.clone()
    }
}
