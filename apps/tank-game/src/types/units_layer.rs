use crate::constants::TileCoord;
use crate::features::unit::unit_tile::{create_unit_tile, UnitTile};
use crate::types::mission_layer::MissionLayer;
use crate::types::PlayersLayer;
use std::collections::HashMap;

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

pub fn create_units_layer(mission_layer: MissionLayer, players_layer: &PlayersLayer) -> UnitsLayer {
    UnitsLayer {
        units: mission_layer
            .get_tiles()
            .iter()
            .filter_map(|(coord, tile)| {
                create_unit_tile(tile.clone(), players_layer.get_by(coord))
                    .ok()
                    .map(|ground_tile| (*coord, ground_tile))
            })
            .collect(),
        width: mission_layer.get_width(),
        height: mission_layer.get_height(),
    }
}
