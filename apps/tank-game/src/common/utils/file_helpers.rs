use std::collections::HashMap;
use std::fs;
use crate::common::constants::RawGrid;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Deserialize, Serialize};

type TileId = u32;
type ImagePath = String;

#[derive(Debug)]
pub struct MainAssetInfo {
    tiles: HashMap<TileId, ImagePath>,
}

impl MainAssetInfo {
    pub fn new(tiles: HashMap<TileId, ImagePath>) -> Self {
        MainAssetInfo { tiles }
    }

    pub fn tiles(&self) -> &HashMap<TileId, ImagePath> {
        &self.tiles
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetTile {
    id: TileId,
    image: ImagePath,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetTileSet {
    tiles: Vec<AssetTile>,
}

#[derive(Debug)]
pub enum FileHelperErrors {
    FileReadError,
    JsonParseError,
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
        let file_content = fs::read_to_string(asset_path).map_err(|_| FileHelperErrors::FileReadError)?;
        let tile_set: AssetTileSet = serde_json::from_str(&file_content).map_err(|_| FileHelperErrors::JsonParseError)?;

        let mut tiles_map: HashMap<TileId, ImagePath> = HashMap::new();
        for tile in tile_set.tiles {
            tiles_map.insert(tile.id, tile.image);
        }

        Ok(MainAssetInfo::new(tiles_map))
    }
}
