use crate::building::building_tile::BuildingTile;
use crate::preparation::main_asset_info_resource::MainAssetInfoResource;
use crate::preparation::types::AssetTile;
use crate::tile::tile_type::GroundTile;
use crate::unit::unit_tile::UnitTile;
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

impl RawMissionLayer {
    pub fn from(&self, tiles_info: &MainAssetInfoResource) -> MissionLayer {
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
pub struct MissionInfo {
    pub ground_layer: GroundLayer,
    pub buildings_layer: BuildingsLayer,
    pub units_layer: UnitsLayer,
}

impl MissionInfo {
    pub fn new(
        ground_layer: GroundLayer,
        buildings_layer: BuildingsLayer,
        units_layer: UnitsLayer,
    ) -> Self {
        MissionInfo {
            ground_layer,
            buildings_layer,
            units_layer,
        }
    }
}

#[derive(Clone, Debug)]
struct MissionLayer {
    pub id: usize,    // TODO: not relevant
    pub name: String, // TODO: not relevant
    pub data: Vec<Vec<AssetTile>>,
}

#[derive(Debug, Clone)]
pub struct GroundLayer {
    grid: Vec<Vec<GroundTile>>,
}

impl GroundLayer {
    pub fn new() -> Self {
        GroundLayer { grid: vec![] }
    }

    pub fn get_grid(&self) -> Vec<Vec<GroundTile>> {
        self.grid.clone()
    }
}

impl Into<GroundLayer> for MissionLayer {
    fn into(self) -> GroundLayer {
        GroundLayer {
            grid: self
                .data
                .iter()
                .map(|row| {
                    row.iter()
                        .filter_map(|tile| GroundTile::try_from(tile.clone()).ok())
                        .collect()
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingsLayer {
    grid: Vec<Vec<BuildingTile>>,
}

impl BuildingsLayer {
    pub fn new() -> Self {
        BuildingsLayer { grid: vec![] }
    }

    pub fn get_grid(&self) -> Vec<Vec<BuildingTile>> {
        self.grid.clone()
    }
}

impl Into<BuildingsLayer> for MissionLayer {
    fn into(self) -> BuildingsLayer {
        BuildingsLayer {
            grid: self
                .data
                .iter()
                .map(|row| {
                    row.iter()
                        .filter_map(|tile| BuildingTile::try_from(tile.clone()).ok())
                        // .map(|tile| BuildingTile::try_from(tile.clone()).unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnitsLayer {
    grid: Vec<Vec<UnitTile>>,
}

impl UnitsLayer {
    pub fn new() -> Self {
        UnitsLayer { grid: vec![] }
    }

    pub fn get_grid(&self) -> Vec<Vec<UnitTile>> {
        self.grid.clone()
    }
}

impl Into<UnitsLayer> for MissionLayer {
    fn into(self) -> UnitsLayer {
        UnitsLayer {
            grid: self
                .data
                .iter()
                .map(|row| {
                    row.iter()
                        .filter_map(|tile| UnitTile::try_from(tile.clone()).ok())
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
    assets: &MainAssetInfoResource,
    mission_file_path: &str,
) -> Result<MissionInfo, LoadMissionError> {
    let content =
        fs::read_to_string(mission_file_path).map_err(|_| LoadMissionError::FileReadError)?;
    let raw_mission: RawMission =
        serde_json::from_str(&content).map_err(|e| LoadMissionError::JsonParseError {
            message: e.to_string(),
        })?;

    let ground_layer: GroundLayer = raw_mission
        .layers
        .iter()
        .find(|layer| layer.name == "ground")
        .ok_or(LoadMissionError::NoGroundLayerError)?
        .from(&assets)
        .into();

    let buildings_layer: BuildingsLayer = raw_mission
        .layers
        .iter()
        .find(|layer| layer.name == "buildings")
        .ok_or(LoadMissionError::NoBuildingsLayerError)?
        .from(&assets)
        .into();
    dbg!(&buildings_layer);

    let units_layer: UnitsLayer = raw_mission
        .layers
        .iter()
        .find(|layer| layer.name == "units")
        .ok_or(LoadMissionError::NoUnitsLayerError)?
        .from(&assets)
        .into();

    let mission_info = MissionInfo::new(ground_layer, buildings_layer, units_layer);

    Ok(mission_info)
}
