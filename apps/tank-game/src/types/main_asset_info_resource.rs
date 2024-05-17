use crate::features::building::types::{BuildingTile, BuildingTileType};
use crate::features::tile::tile_type::{GroundTile, GroundTileType};
use crate::features::unit::unit_tile::{UnitTile, UnitTileType};
use crate::types::{AssetTile, AssetTileId, AssetTileSubType, AssetTileType};
use bevy::prelude::Resource;
use std::collections::HashMap;

#[derive(Debug, Resource)]
pub struct MainAssetInfoResource {
    loaded: bool,
    tiles: HashMap<AssetTileId, AssetTile>,
    building_tiles: HashMap<BuildingTileType, BuildingTile>,
    unit_tiles: HashMap<UnitTileType, UnitTile>,
    ground_tiles: HashMap<GroundTileType, GroundTile>,
}

impl MainAssetInfoResource {
    pub fn new() -> Self {
        MainAssetInfoResource {
            loaded: false,
            tiles: HashMap::new(),
            building_tiles: HashMap::new(),
            unit_tiles: HashMap::new(),
            ground_tiles: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, tiles: HashMap<AssetTileId, AssetTile>) {
        self.tiles = tiles;

        for tile in self.tiles.values() {
            match tile.get_tile_type() {
                Some(AssetTileType::Building) => {
                    let res = BuildingTile::try_from(tile.clone());
                    if let Ok(building_tile) = res {
                        self.building_tiles
                            .insert(building_tile.get_building_type(), building_tile);
                    }
                }
                Some(AssetTileType::Unit) => {
                    let res = UnitTile::try_from(tile.clone());
                    if let Ok(unit_tile) = res {
                        self.unit_tiles.insert(unit_tile.get_unit_type(), unit_tile);
                    }
                }
                Some(AssetTileType::Ground) => {
                    let res = GroundTile::try_from(tile.clone());
                    if let Ok(ground_tile) = res {
                        self.ground_tiles
                            .insert(ground_tile.get_ground_type(), ground_tile);
                    }
                }
                None => {}
            }
        }

        self.loaded = true;
    }

    pub fn get_tiles(&self) -> &HashMap<AssetTileId, AssetTile> {
        &self.tiles
    }

    pub fn get_building_tiles(&self) -> &HashMap<BuildingTileType, BuildingTile> {
        &self.building_tiles
    }

    pub fn get_unit_tiles(&self) -> &HashMap<UnitTileType, UnitTile> {
        &self.unit_tiles
    }

    pub fn get_ground_tiles(&self) -> &HashMap<GroundTileType, GroundTile> {
        &self.ground_tiles
    }

    pub fn find_tile_by_sub_type(&self, sub_type: AssetTileSubType) -> Option<&AssetTile> {
        self.tiles
            .values()
            .filter(|tile| tile.get_tile_sub_type().is_some())
            .find(|tile| tile.get_tile_sub_type().unwrap() == sub_type)
    }
}
