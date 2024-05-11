use crate::common::constants::TileSize;
use crate::common::player::Player;
use crate::preparation::types::{AssetTile, AssetTileSubType, AssetTileType};

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

#[derive(Clone, Debug, PartialEq)]
pub enum UnitTileType {
    Tank = 1,
    Soldier = 2,
    Harvester = 3,
}

#[derive(Debug)]
pub enum UnitTileErrors {
    InvalidUnitType,
    MissingPlayer,
}

impl TryFrom<AssetTile> for UnitTile {
    type Error = UnitTileErrors;

    fn try_from(value: AssetTile) -> Result<Self, Self::Error> {
        if value.get_tile_type() != AssetTileType::Unit {
            return Err(UnitTileErrors::InvalidUnitType);
        }

        let unit_tile_type = match value.get_tile_sub_type() {
            AssetTileSubType::Tank => Ok(UnitTileType::Tank),
            AssetTileSubType::Soldier => Ok(UnitTileType::Soldier),
            AssetTileSubType::Harvester => Ok(UnitTileType::Harvester),
            _ => Err(UnitTileErrors::InvalidUnitType),
        };

        if unit_tile_type.is_err() {
            return Err(UnitTileErrors::InvalidUnitType);
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
