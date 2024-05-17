use crate::features::preparation::types::buildings_layer::BuildingsLayer;
use crate::features::preparation::types::ground_layer::GroundLayer;
use crate::features::preparation::types::main_asset_info_resource::MainAssetInfoResource;
use crate::features::preparation::types::mission_info::MissionInfo;
use crate::features::preparation::types::raw_mission_layer::RawMission;
use crate::features::preparation::types::units_layer::UnitsLayer;
use std::fs;

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
        .get_layers()
        .iter()
        .find(|layer| layer.is_ground())
        .ok_or(LoadMissionError::NoGroundLayerError)?
        .from(&assets)
        .into();

    let buildings_layer: BuildingsLayer = raw_mission
        .get_layers()
        .iter()
        .find(|layer| layer.is_buildings())
        .ok_or(LoadMissionError::NoBuildingsLayerError)?
        .from(&assets)
        .into();

    let units_layer: UnitsLayer = raw_mission
        .get_layers()
        .iter()
        .find(|layer| layer.is_units())
        .ok_or(LoadMissionError::NoUnitsLayerError)?
        .from(&assets)
        .into();

    let mission_info = MissionInfo::new(ground_layer, buildings_layer, units_layer);

    Ok(mission_info)
}
