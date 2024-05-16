use crate::constants::{TileCoord, TileGrid};
use crate::features::building::building_tile::BuildingTile;
use crate::features::preparation::main_asset_info_resource::MainAssetInfoResource;
use crate::features::preparation::types::{AssetTile, AssetTileId};
use crate::features::tile::tile_type::{GroundTile, GroundTileType};
use crate::features::unit::unit_tile::UnitTile;
use serde::Deserialize;
use std::collections::HashMap;
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
    pub fn get_data_2d(&self) -> Vec<Vec<AssetTileId>> {
        let mut data_2d = vec![vec![0; self.width]; self.height];
        for (index, tile_id) in self.data.iter().enumerate() {
            let row = index / self.width;
            let col = index % self.width;
            data_2d[row][col] = *tile_id as i32;
        }

        // rotate data_2d clockwise 90 degrees
        let mut rotated_data = vec![vec![0; self.height]; self.width]; // flipped dimensions
        for row in 0..self.height {
            for col in 0..self.width {
                // Transpose and then reverse rows to rotate counterclockwise
                rotated_data[col][self.height - 1 - row] = data_2d[row][col];
            }
        }

        rotated_data
    }
}

impl RawMissionLayer {
    pub fn from(&self, tiles_info: &MainAssetInfoResource) -> MissionLayer {
        let tiles: &HashMap<AssetTileId, AssetTile> = tiles_info.get_tiles();

        let mut tiles_coords: HashMap<TileCoord, AssetTile> = HashMap::new();

        self.get_data_2d()
            .iter()
            .enumerate()
            .for_each(|(row_index, row)| {
                row.iter().enumerate().for_each(|(col_index, tile_id)| {
                    let res = tiles.get(&(*tile_id - 1));
                    if let Some(tile) = res {
                        tiles_coords.insert((row_index, col_index), tile.clone());
                    }
                });
            });
        let width = self.width;
        let height = self.height;

        MissionLayer {
            tiles: tiles_coords,
            width,
            height,
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
    tiles: HashMap<TileCoord, AssetTile>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
pub struct GroundLayer {
    tiles: HashMap<TileCoord, GroundTile>,
    pub height: usize,
    pub width: usize,
}

impl GroundLayer {
    pub fn new() -> Self {
        GroundLayer {
            tiles: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn get_tiles(&self) -> &HashMap<TileCoord, GroundTile> {
        &self.tiles
    }

    pub fn get_width_height(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn to_2d_grid(&self) -> TileGrid {
        let mut grid = vec![vec![GroundTileType::Grass; self.height]; self.width];
        self.tiles.iter().for_each(|(coord, ground)| {
            grid[coord.0][coord.1] = ground.get_ground_type().clone();
        });
        grid
    }
}

impl Into<GroundLayer> for MissionLayer {
    fn into(self) -> GroundLayer {
        GroundLayer {
            tiles: self
                .tiles
                .iter()
                .filter_map(|(coord, tile)| {
                    GroundTile::try_from(tile.clone())
                        .ok()
                        .map(|ground_tile| (*coord, ground_tile))
                })
                .collect(),
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingsLayer {
    buildings: HashMap<TileCoord, BuildingTile>,
    pub width: usize,
    pub height: usize,
}

impl BuildingsLayer {
    pub fn new() -> Self {
        BuildingsLayer {
            buildings: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn enumerate(&self) -> Vec<(TileCoord, BuildingTile)> {
        self.buildings
            .iter()
            .map(|(coord, tile)| (*coord, tile.clone()))
            .collect()
    }
}

impl Into<BuildingsLayer> for MissionLayer {
    fn into(self) -> BuildingsLayer {
        BuildingsLayer {
            buildings: self
                .tiles
                .iter()
                .filter_map(|(coord, tile)| {
                    BuildingTile::try_from(tile.clone())
                        .ok()
                        .map(|ground_tile| (*coord, ground_tile))
                })
                .collect(),
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnitsLayer {
    units: HashMap<TileCoord, UnitTile>,
    pub width: usize,
    pub height: usize,
}

impl UnitsLayer {
    pub fn new() -> Self {
        UnitsLayer {
            units: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn get_units(&self) -> &HashMap<TileCoord, UnitTile> {
        &self.units
    }
}

impl Into<UnitsLayer> for MissionLayer {
    fn into(self) -> UnitsLayer {
        UnitsLayer {
            units: self
                .tiles
                .iter()
                .filter_map(|(coord, tile)| {
                    UnitTile::try_from(tile.clone())
                        .ok()
                        .map(|ground_tile| (*coord, ground_tile))
                })
                .collect(),
            width: self.width,
            height: self.height,
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
