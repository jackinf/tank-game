use crate::constants::TileCoord;
use crate::features::preparation::types::mission_layer::MissionLayer;
use crate::features::unit::unit_tile::UnitTile;
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

impl Into<UnitsLayer> for MissionLayer {
    fn into(self) -> UnitsLayer {
        UnitsLayer {
            units: self
                .get_tiles()
                .iter()
                .filter_map(|(coord, tile)| {
                    UnitTile::try_from(tile.clone())
                        .ok()
                        .map(|ground_tile| (*coord, ground_tile))
                })
                .collect(),
            width: self.get_width(),
            height: self.get_height(),
        }
    }
}
