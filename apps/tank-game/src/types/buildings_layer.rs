use crate::constants::TileCoord;
use crate::features::building::types::BuildingTile;
use crate::types::mission_layer::MissionLayer;
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

impl Into<BuildingsLayer> for MissionLayer {
    fn into(self) -> BuildingsLayer {
        BuildingsLayer {
            buildings: self
                .get_tiles()
                .iter()
                .filter_map(|(coord, tile)| {
                    BuildingTile::try_from(tile.clone())
                        .ok()
                        .map(|ground_tile| (*coord, ground_tile))
                })
                .collect(),
            width: self.get_width(),
            height: self.get_height(),
        }
    }
}
