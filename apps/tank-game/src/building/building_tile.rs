use crate::common::constants::TileSize;
use crate::common::player::Player;
use crate::con_menu::components::submenu_info::SubMenuType;
use crate::preparation::types::{AssetTile, AssetTileSubType, AssetTileType};
use crate::unit::unit_tile::UnitTileErrors;

// TODO: consider using trait like CommonTile or GeneralTile
#[derive(Clone, Debug, PartialEq)]
pub struct BuildingTile {
    image_path: String,
    tile_size: TileSize,
    building_type: BuildingTileType,
    player: Player,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuildingTileType {
    Base = 10,
    Factory = 20,
    PowerPlant = 30,
}

#[derive(Debug)]
pub enum BuildingTileErrors {
    InvalidBuildingType,
    MissingPlayer,
}

impl TryFrom<AssetTile> for BuildingTile {
    type Error = BuildingTileErrors;

    fn try_from(value: AssetTile) -> Result<Self, Self::Error> {
        if value.get_tile_type() != AssetTileType::Building {
            return Err(BuildingTileErrors::InvalidBuildingType);
        }

        let building_tile_type = match value.get_tile_sub_type() {
            AssetTileSubType::Base => Ok(BuildingTile::Base),
            AssetTileSubType::Factory => Ok(BuildingTile::Factory),
            AssetTileSubType::Powerplant => Ok(BuildingTile::PowerPlant),
            _ => Err(BuildingTileErrors::InvalidBuildingType),
        };

        if building_tile_type.is_err() {
            return Err(BuildingTileErrors::InvalidBuildingType);
        }
        let building_tile_type = building_tile_type.unwrap();

        if value.get_player().is_none() {
            return Err(BuildingTileErrors::MissingPlayer);
        }
        let player = value.get_player().unwrap();

        Ok(BuildingTile {
            image_path: value.get_image_path(),
            tile_size: value.get_tile_size(),
            building_type: building_tile_type,
            player,
        })
    }
}

impl BuildingTile {
    pub fn get_image_path(&self) -> String {
        self.image_path.clone()
    }

    pub fn get_layer(&self) -> f32 {
        match &self {
            BuildingTile::Base => 20.,
            BuildingTile::Factory => 20.,
            BuildingTile::PowerPlant => 20.,
        }
    }

    pub fn get_size(&self) -> TileSize {
        self.tile_size
    }

    pub fn get_player(&self) -> Player {
        self.player.clone()
    }

    // TODO: what is this for?
    pub fn get_sub_menu_type(&self) -> Option<SubMenuType> {
        match &self {
            BuildingTile::Base => Some(SubMenuType::Base),
            BuildingTile::Factory => Some(SubMenuType::Factory),
            _ => None,
        }
    }

    pub fn get_power_level(&self) -> i32 {
        match &self {
            BuildingTile::Base => -50,
            BuildingTile::Factory => -20,
            BuildingTile::PowerPlant => 100,
        }
    }
}
