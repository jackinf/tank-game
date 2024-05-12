use crate::common::constants::TileSize;
use crate::common::player::Player;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type AssetTileId = usize;
pub type AssetImagePath = String;

// TODO: why do i need this?
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AssetTileType {
    Building,
    Unit,
    Ground,
}

impl Display for AssetTileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssetTileType::Building => write!(f, "building"),
            AssetTileType::Unit => write!(f, "unit"),
            AssetTileType::Ground => write!(f, "ground"),
        }
    }
}

impl PartialEq for AssetTileType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AssetTileType::Building, AssetTileType::Building) => true,
            (AssetTileType::Unit, AssetTileType::Unit) => true,
            (AssetTileType::Ground, AssetTileType::Ground) => true,
            _ => false,
        }
    }
}

impl FromStr for AssetTileType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "building" => Ok(AssetTileType::Building),
            "unit" => Ok(AssetTileType::Unit),
            "ground" => Ok(AssetTileType::Ground),
            _ => Err(ParseError {
                message: format!("'{}' is not a valid AssetTileType", s),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AssetTileSubType {
    Base,
    Factory,
    Powerplant,
    Tank,
    Gold,
    Ground,
    Invalid,
    Wall,
    Water,
    Soldier,
    Harvester,
}

impl Display for AssetTileSubType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AssetTileSubType::Base => write!(f, "base"),
            AssetTileSubType::Factory => write!(f, "factory"),
            AssetTileSubType::Powerplant => write!(f, "powerplant"),
            AssetTileSubType::Tank => write!(f, "tank"),
            AssetTileSubType::Gold => write!(f, "gold"),
            AssetTileSubType::Ground => write!(f, "ground"),
            AssetTileSubType::Invalid => write!(f, "invalid"),
            AssetTileSubType::Wall => write!(f, "wall"),
            AssetTileSubType::Water => write!(f, "water"),
            AssetTileSubType::Soldier => write!(f, "soldier"),
            AssetTileSubType::Harvester => write!(f, "harvester"),
        }
    }
}

impl PartialEq for AssetTileSubType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AssetTileSubType::Base, AssetTileSubType::Base) => true,
            (AssetTileSubType::Factory, AssetTileSubType::Factory) => true,
            (AssetTileSubType::Powerplant, AssetTileSubType::Powerplant) => true,
            (AssetTileSubType::Tank, AssetTileSubType::Tank) => true,
            (AssetTileSubType::Gold, AssetTileSubType::Gold) => true,
            (AssetTileSubType::Ground, AssetTileSubType::Ground) => true,
            (AssetTileSubType::Invalid, AssetTileSubType::Invalid) => true,
            (AssetTileSubType::Wall, AssetTileSubType::Wall) => true,
            (AssetTileSubType::Water, AssetTileSubType::Water) => true,
            _ => false,
        }
    }
}

impl FromStr for AssetTileSubType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "base" => Ok(AssetTileSubType::Base),
            "factory" => Ok(AssetTileSubType::Factory),
            "powerplant" => Ok(AssetTileSubType::Powerplant),
            "tank" => Ok(AssetTileSubType::Tank),
            "gold" => Ok(AssetTileSubType::Gold),
            "ground" => Ok(AssetTileSubType::Ground),
            "invalid" => Ok(AssetTileSubType::Invalid),
            "wall" => Ok(AssetTileSubType::Wall),
            "water" => Ok(AssetTileSubType::Water),
            _ => Err(ParseError {
                message: format!("'{}' is not a valid AssetTileSubType", s),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssetTile {
    id: AssetTileId,
    image: AssetImagePath,
    tile_size: TileSize,
    tile_type: AssetTileType,
    tile_sub_type: AssetTileSubType,
    player: Option<Player>,
}

impl AssetTile {
    pub fn new(
        id: AssetTileId,
        image: AssetImagePath,
        tile_size: TileSize,
        tile_type: AssetTileType,
        tile_sub_type: AssetTileSubType,
        player: Option<Player>,
    ) -> Self {
        AssetTile {
            id,
            image,
            tile_size,
            tile_type,
            tile_sub_type,
            player,
        }
    }

    pub fn is_id_and_type(&self, id: AssetTileId, tile_type: AssetTileType) -> bool {
        self.id == id && self.tile_type == tile_type
    }

    pub fn get_tile_size(&self) -> TileSize {
        self.tile_size.clone()
    }

    pub fn get_tile_type(&self) -> AssetTileType {
        self.tile_type.clone()
    }

    pub fn get_tile_sub_type(&self) -> AssetTileSubType {
        self.tile_sub_type.clone()
    }

    pub fn get_image_path(&self) -> String {
        self.image.clone()
    }

    pub fn get_player(&self) -> Option<Player> {
        self.player.clone()
    }
}

// TODO: i don't like this option
// impl From<&AssetTile> for usize {
//     fn from(asset_tile: &AssetTile) -> usize {
//         match asset_tile.tile_sub_type {
//             AssetTileSubType::Base => 0,
//             AssetTileSubType::Factory => 1,
//             AssetTileSubType::Powerplant => 2,
//             AssetTileSubType::Tank => 3,
//             AssetTileSubType::Gold => 4,
//             AssetTileSubType::Ground => 5,
//             AssetTileSubType::Invalid => 6,
//             AssetTileSubType::Wall => 7,
//             AssetTileSubType::Water => 8,
//         }
//     }
// }
