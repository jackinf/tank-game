use crate::common::constants::{RawGrid, SPRITE_SCALE, TILE_SIZE, TileSize};
use crate::preparation::types::{
    AssetImagePath, AssetTile, AssetTileId, AssetTileSubType, AssetTileType,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{fmt, fs};
use crate::common::player::Player;

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

pub struct FileHelpers;

impl FileHelpers {
    pub fn read_map_from_file(map_path: &str) -> RawGrid {
        let map_file = File::open(map_path).expect("Failed to read file");
        let reader = BufReader::new(map_file);

        let mut tilemap: RawGrid = vec![];
        for line_result in reader.lines() {
            if let Err(_) = line_result {
                continue;
            }
            let line = line_result.unwrap();
            if line.is_empty() {
                continue;
            }

            let cells: Vec<usize> = line
                .split(' ')
                .map(|letter| letter.parse::<usize>().unwrap())
                .collect();
            tilemap.push(cells.clone());
        }
        tilemap
    }

    pub fn read_assets(asset_path: &str) -> Result<MainAssetInfo, FileHelperErrors> {
        let file_content =
            fs::read_to_string(asset_path).map_err(|_| FileHelperErrors::FileReadError)?;
        let tile_set: AssetRawTileSet =
            serde_json::from_str(&file_content).map_err(|_| FileHelperErrors::JsonParseError)?;

        let mut tiles_map: HashMap<AssetTileId, AssetTile> = HashMap::new();
        for tile in tile_set.tiles {
            let tile_id = tile.id;
            let image_width = tile.image_width as f32;
            let image_height = tile.image_height as f32;
            let tile_width = image_width / TILE_SIZE / SPRITE_SCALE;
            let tile_height = image_height / TILE_SIZE / SPRITE_SCALE;
            let tile_size: TileSize = (tile_width.round() as usize, tile_height.round() as usize);

            let tile_type = tile.properties.iter().find(|p| p.name == "type");
            let tile_sub_type = tile.properties.iter().find(|p| p.name == "subtype");
            let player = tile.properties.iter().find(|p| p.name == "player");

            let tile_type = tile_type
                .ok_or_else(|| FileHelperErrors::TileTypeNotFound { tile_id })?
                .value
                .clone();
            let tile_sub_type = tile_sub_type
                .ok_or_else(|| FileHelperErrors::TileSubTypeNotFound { tile_id })?
                .value
                .clone();
            let player: Option<Player> = player
                .map(|p| p.value.clone())
                .map(|p| if p == "1" { Some(Player::P1) } else if p == "2" { Some(Player::P2) } else { None })
                .flatten();

            let tile_type = AssetTileType::from_str(&tile_type)
                .map_err(|_| FileHelperErrors::TileTypeParseFailed { tile_id })?;
            let tile_sub_type = AssetTileSubType::from_str(&tile_sub_type)
                .map_err(|_| FileHelperErrors::TileSubTypeParseFailed { tile_id })?;

            tiles_map.insert(
                tile_id,
                AssetTile::new(tile_id, tile.image, tile_size, tile_type, tile_sub_type, player),
            );
        }

        Ok(MainAssetInfo::new(tiles_map))
    }
}
