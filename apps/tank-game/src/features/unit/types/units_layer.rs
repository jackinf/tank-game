use crate::constants::TileCoord;
use crate::features::unit::types::unit_tile::UnitTile;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UnitsLayer {
    units: HashMap<TileCoord, UnitTile>,
    pub width: usize,
    pub height: usize,
}

impl UnitsLayer {
    pub fn empty() -> Self {
        UnitsLayer {
            units: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn new(units: HashMap<TileCoord, UnitTile>, width: usize, height: usize) -> Self {
        UnitsLayer {
            units,
            width,
            height,
        }
    }

    pub fn get_units(&self) -> &HashMap<TileCoord, UnitTile> {
        &self.units
    }
}
