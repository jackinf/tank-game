use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::{fmt, fs};

use serde::{Deserialize, Serialize};

use crate::constants::{TileSize, SPRITE_SCALE, TILE_SIZE};
use crate::types::main_asset_info::MainAssetInfo;
use crate::types::player::Player;
use crate::types::{AssetImagePath, AssetTile, AssetTileId, AssetTileSubType, AssetTileType};

#[derive(Serialize, Deserialize, Debug)]
struct AssetRawTileProperty {
    name: String,
    #[serde(rename = "type")]
    value_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetRawTile {
    id: AssetTileId,
    image: AssetImagePath,
    #[serde(rename = "imageheight")]
    image_height: usize,
    #[serde(rename = "imagewidth")]
    image_width: usize,
    #[serde(default)]
    properties: Vec<AssetRawTileProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetRawTileSet {
    tiles: Vec<AssetRawTile>,
}

#[derive(Debug)]
pub enum FileHelperErrors {
    FileReadError,
    JsonParseError,
    TileTypeNotFound { tile_id: AssetTileId },
    TileSubTypeNotFound { tile_id: AssetTileId },
    TileTypeParseFailed { tile_id: AssetTileId },
    TileSubTypeParseFailed { tile_id: AssetTileId },
}

impl Display for FileHelperErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

pub fn read_main_assets(asset_path: &str) -> Result<MainAssetInfo, FileHelperErrors> {
    let file_content =
        fs::read_to_string(asset_path).map_err(|_| FileHelperErrors::FileReadError)?;
    let tile_set: AssetRawTileSet =
        serde_json::from_str(&file_content).map_err(|_| FileHelperErrors::JsonParseError)?;

    let mut tiles_map: HashMap<AssetTileId, AssetTile> = HashMap::new();
    for tile in tile_set.tiles {
        let tile_id = tile.id;
        let image_width = tile.image_width as f32;
        let image_height = tile.image_height as f32;
        let tile_width = image_width / TILE_SIZE * SPRITE_SCALE;
        let tile_height = image_height / TILE_SIZE * SPRITE_SCALE;
        let tile_size: TileSize = (tile_width.round() as usize, tile_height.round() as usize);

        let tile_type = tile
            .properties
            .iter()
            .find(|p| p.name == "type")
            .map(|p| p.value.clone())
            .map(|tt| {
                AssetTileType::from_str(&tt)
                    .map_err(|_| FileHelperErrors::TileTypeParseFailed { tile_id })
            })
            .transpose()?;

        let tile_sub_type = tile
            .properties
            .iter()
            .find(|p| p.name == "subtype")
            .map(|p| p.value.clone())
            .map(|tst| {
                AssetTileSubType::from_str(&tst)
                    .map_err(|_| FileHelperErrors::TileSubTypeParseFailed { tile_id })
            })
            .transpose()?;

        tiles_map.insert(
            tile_id,
            AssetTile::new(tile_id, tile.image, tile_size, tile_type, tile_sub_type),
        );
    }

    Ok(MainAssetInfo::new(tiles_map))
}
