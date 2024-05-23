use crate::features::building::types::building_tile::create_building_tile;
use crate::features::building::types::{BuildingTile, BuildingTileType};
use crate::features::unit::{create_unit_tile, UnitTile, UnitTileType};
use crate::types::{AssetTile, AssetTileId, AssetTileType};
use bevy::prelude::Resource;
use std::collections::HashMap;

#[derive(Debug, Resource)]
pub struct MainAssetInfoResource {
    loaded: bool,
    building_tiles: HashMap<BuildingTileType, BuildingTile>,
    unit_tiles: HashMap<UnitTileType, UnitTile>,
}

impl MainAssetInfoResource {
    pub fn new() -> Self {
        MainAssetInfoResource {
            loaded: false,
            building_tiles: HashMap::new(),
            unit_tiles: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, tiles: HashMap<AssetTileId, AssetTile>) {
        for tile in tiles.values() {
            match tile.get_tile_type() {
                Some(AssetTileType::Building) => {
                    let res = create_building_tile(tile.clone(), None);
                    if let Ok(building_tile) = res {
                        self.building_tiles
                            .insert(building_tile.get_building_type(), building_tile);
                    }
                }
                Some(AssetTileType::Unit) => {
                    let res = create_unit_tile(tile.clone(), None);
                    if let Ok(unit_tile) = res {
                        self.unit_tiles.insert(unit_tile.get_unit_type(), unit_tile);
                    }
                }
                Some(AssetTileType::Player) => {}
                Some(AssetTileType::Ground) => {}
                Some(AssetTileType::Resource) => {}
                None => {}
            }
        }

        self.loaded = true;
    }

    pub fn get_building_tiles(&self) -> &HashMap<BuildingTileType, BuildingTile> {
        &self.building_tiles
    }

    pub fn get_unit_tiles(&self) -> &HashMap<UnitTileType, UnitTile> {
        &self.unit_tiles
    }
}
