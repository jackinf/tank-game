use crate::common::utils::file_helpers::MainAssetInfo;
use crate::preparation::types::AssetTile;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct RawMission {
    layers: Vec<RawMissionLayer>,
}

#[derive(Deserialize, Debug)]
struct RawMissionLayer {
    id: usize,
    name: String,
    data: Vec<usize>,
    width: usize,
    height: usize,
}

impl RawMissionLayer {
    pub fn get_data_2d(&self) -> Vec<Vec<usize>> {
        let mut data_2d = vec![vec![0; self.width]; self.height];
        for (index, tile_id) in self.data.iter().enumerate() {
            let row = index / self.width;
            let col = index % self.width;
            data_2d[row][col] = *tile_id;
        }
        data_2d
    }
}

#[derive(Debug)]
pub struct MissionInfo {
    pub layers: Vec<MissionLayer>,
}

#[derive(Clone, Debug)]
pub struct MissionLayer {
    pub id: usize,
    pub name: String,
    pub data: Vec<Vec<AssetTile>>,
}

impl RawMissionLayer {
    pub fn from(&self, tiles_info: &MainAssetInfo) -> MissionLayer {
        let tiles = tiles_info.get_tiles();
        MissionLayer {
            id: self.id,
            name: self.name.clone(),
            data: self
                .get_data_2d()
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile_id| tiles.get(&tile_id).unwrap().clone())
                        .collect()
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub enum LoadMissionError {
    FileReadError,
    JsonParseError { message: String },
    NoGroundLayerError,
    NoBuildingsLayerError,
    NoUnitsLayerError,
}

pub fn load_mission(
    assets: &MainAssetInfo,
    mission_file_path: &str,
) -> Result<MissionInfo, LoadMissionError> {
    let content =
        fs::read_to_string(mission_file_path).map_err(|_| LoadMissionError::FileReadError)?;
    let raw_mission: RawMission =
        serde_json::from_str(&content).map_err(|e| LoadMissionError::JsonParseError {
            message: e.to_string(),
        })?;

    let ground_layer = &raw_mission
        .layers
        .iter()
        .find(|layer| layer.name == "ground")
        .ok_or(LoadMissionError::NoGroundLayerError)?
        .from(&assets);
    let buildings_layer = raw_mission
        .layers
        .iter()
        .find(|layer| layer.name == "buildings")
        .ok_or(LoadMissionError::NoBuildingsLayerError)?
        .from(&assets);
    let units_layer = raw_mission
        .layers
        .iter()
        .find(|layer| layer.name == "units")
        .ok_or(LoadMissionError::NoUnitsLayerError)?
        .from(&assets);

    let mission_info = MissionInfo {
        layers: vec![
            ground_layer.clone(),
            buildings_layer.clone(),
            units_layer.clone(),
        ],
    };

    Ok(mission_info)
}
