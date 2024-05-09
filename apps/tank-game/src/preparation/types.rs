use serde::{Deserialize, Serialize};
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

pub type AssetTileId = u32;
pub type AssetImagePath = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AssetTileType {
    Building,
    Unit,
    Ground,
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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
}
