use crate::types::buildings_layer::{create_buildings_layer, BuildingsLayer};
use crate::types::ground_layer::GroundLayer;
use crate::types::mission_info::MissionInfo;
use crate::types::mission_layer::MissionLayer;
use crate::types::raw_mission_layer::RawMission;
use crate::types::units_layer::{create_units_layer, UnitsLayer};
use crate::types::{AssetTile, AssetTileId, PlayersLayer};
use std::collections::HashMap;
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
    assets: &HashMap<AssetTileId, AssetTile>,
    mission_file_path: &str,
) -> Result<MissionInfo, LoadMissionError> {
    let content =
        fs::read_to_string(mission_file_path).map_err(|_| LoadMissionError::FileReadError)?;
    let raw_mission: RawMission =
        serde_json::from_str(&content).map_err(|e| LoadMissionError::JsonParseError {
            message: e.to_string(),
        })?;

    let players_layer: PlayersLayer = raw_mission
        .get_layers()
        .iter()
        .find(|layer| layer.is_players())
        .ok_or(LoadMissionError::NoGroundLayerError)?
        .from(&assets)
        .into();

    let ground_layer: GroundLayer = raw_mission
        .get_layers()
        .iter()
        .find(|layer| layer.is_ground())
        .ok_or(LoadMissionError::NoGroundLayerError)?
        .from(&assets)
        .into();

    let resource_layer: GroundLayer = raw_mission
        .get_layers()
        .iter()
        .find(|layer| layer.is_resource())
        .ok_or(LoadMissionError::NoGroundLayerError)?
        .from(&assets)
        .into();

    let buildings_layer: MissionLayer = raw_mission
        .get_layers()
        .iter()
        .find(|layer| layer.is_buildings())
        .ok_or(LoadMissionError::NoBuildingsLayerError)?
        .from(&assets);
    let buildings_layer: BuildingsLayer = create_buildings_layer(buildings_layer, &players_layer);

    let units_layer: MissionLayer = raw_mission
        .get_layers()
        .iter()
        .find(|layer| layer.is_units())
        .ok_or(LoadMissionError::NoUnitsLayerError)?
        .from(&assets);
    let units_layer: UnitsLayer = create_units_layer(units_layer, &players_layer);

    let mission_info = MissionInfo::new(ground_layer, resource_layer, buildings_layer, units_layer);

    Ok(mission_info)
}
