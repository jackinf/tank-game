use std::collections::HashMap;

use serde::Deserialize;

use crate::constants::TileCoord;
use crate::types::mission_layer::MissionLayer;
use crate::types::{AssetTile, AssetTileId};

#[derive(Deserialize, Debug)]
pub struct RawMission {
    layers: Vec<RawMissionLayer>,
}

impl RawMission {
    pub fn get_layers(&self) -> &Vec<RawMissionLayer> {
        &self.layers
    }
}

#[derive(Deserialize, Debug)]
pub struct RawMissionLayer {
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

    pub fn is_ground(&self) -> bool {
        self.name == "ground"
    }

    pub fn is_resource(&self) -> bool {
        self.name == "resources"
    }

    pub fn is_players(&self) -> bool {
        self.name == "players"
    }

    pub fn is_buildings(&self) -> bool {
        self.name == "buildings"
    }

    pub fn is_units(&self) -> bool {
        self.name == "units"
    }
}

impl RawMissionLayer {
    pub fn from(&self, tiles: &HashMap<AssetTileId, AssetTile>) -> MissionLayer {
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

        MissionLayer::from(tiles_coords, width, height)
    }
}
