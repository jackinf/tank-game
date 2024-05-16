use crate::constants::TileSize;
use crate::features::preparation::types::{AssetTile, AssetTileSubType, AssetTileType};
use crate::types::player::Player;

// TODO: consider using trait like CommonTile or GeneralTile
#[derive(Clone, Debug, PartialEq)]
pub struct UnitTile {
    image_path: String,
    tile_size: TileSize,
    unit_type: UnitTileType,
    player: Player,
}

impl UnitTile {
    pub fn get_image_path(&self) -> String {
        self.image_path.clone()
    }

    pub fn get_tile_size(&self) -> TileSize {
        self.tile_size.clone()
    }

    pub fn get_unit_type(&self) -> UnitTileType {
        self.unit_type.clone()
    }

    pub fn get_player(&self) -> Player {
        self.player.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum UnitTileType {
    Tank = 1,
    Soldier = 2,
    Harvester = 3,
}

#[derive(Debug)]
pub enum UnitTileErrors {
    TileTypeIsRequired,
    TileSubTypeIsRequired,
    InvalidUnitType { message: String },
    MissingPlayer,
}

impl TryFrom<AssetTile> for UnitTile {
    type Error = UnitTileErrors;

    fn try_from(value: AssetTile) -> Result<Self, Self::Error> {
        let tile_type = value.get_tile_type();
        let tile_sub_type = value.get_tile_sub_type();

        if tile_type.is_none() {
            return Err(UnitTileErrors::TileTypeIsRequired);
        }

        if tile_sub_type.is_none() {
            return Err(UnitTileErrors::TileSubTypeIsRequired);
        }

        let tile_type = tile_type.unwrap();
        let tile_sub_type = tile_sub_type.unwrap();

        if tile_type != AssetTileType::Unit {
            return Err(UnitTileErrors::InvalidUnitType {
                message: format!("'{}' is not a valid AssetTileType", tile_type.to_string()),
            });
        }

        let unit_tile_type = match tile_sub_type {
            AssetTileSubType::Tank => Ok(UnitTileType::Tank),
            AssetTileSubType::Soldier => Ok(UnitTileType::Soldier),
            AssetTileSubType::Harvester => Ok(UnitTileType::Harvester),
            _ => Err(UnitTileErrors::InvalidUnitType {
                message: format!(
                    "'{}' is not a valid UnitTileType",
                    tile_sub_type.to_string()
                ),
            }),
        };

        if unit_tile_type.is_err() {
            return Err(UnitTileErrors::InvalidUnitType {
                message: format!(
                    "'{}' is not a valid UnitTileType",
                    tile_sub_type.to_string()
                ),
            });
        }
        let unit_tile_type = unit_tile_type.unwrap();

        if value.get_player().is_none() {
            return Err(UnitTileErrors::MissingPlayer);
        }
        let player = value.get_player().unwrap();

        Ok(UnitTile {
            image_path: value.get_image_path(),
            tile_size: value.get_tile_size(),
            unit_type: unit_tile_type,
            player,
        })
    }
}
