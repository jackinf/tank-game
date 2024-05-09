use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AssetTileType {
    Building,
    Unit,
    Ground,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetTile {
    id: AssetTileId,
    image: AssetImagePath,
    tile_type: AssetTileType,
    tile_sub_type: AssetTileSubType,
}

impl AssetTile {
    pub fn new(
        id: AssetTileId,
        image: AssetImagePath,
        tile_type: AssetTileType,
        tile_sub_type: AssetTileSubType,
    ) -> Self {
        AssetTile {
            id,
            image,
            tile_type,
            tile_sub_type,
        }
    }

    pub fn is_id_and_type(&self, id: AssetTileId, tile_type: AssetTileType) -> bool {
        self.id == id && self.tile_type == tile_type
    }
}
