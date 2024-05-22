use crate::constants::TileCoord;
use crate::features::building::types::building_tile::create_building_tile;
use crate::features::building::types::BuildingTile;
use crate::types::mission_layer::MissionLayer;
use crate::types::player::Player;
use crate::types::PlayersLayer;
use std::collections::HashMap;

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

pub fn create_buildings_layer(
    mission_layer: MissionLayer,
    players_layer: &PlayersLayer,
) -> BuildingsLayer {
    BuildingsLayer {
        buildings: mission_layer
            .get_tiles()
            .iter()
            .filter_map(|(coord, tile)| {
                create_building_tile(tile.clone(), players_layer.get_by(coord))
                    .ok()
                    .map(|ground_tile| (*coord, ground_tile))
            })
            .collect(),
        width: mission_layer.get_width(),
        height: mission_layer.get_height(),
    }
}
