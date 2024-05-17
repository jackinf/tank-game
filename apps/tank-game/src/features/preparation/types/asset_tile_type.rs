use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AssetTileType {
    Building,
    Unit,
    Ground,
}

#[derive(Debug, Clone)]
pub struct AssetTileTypeParseError {
    message: String,
}

impl Display for AssetTileTypeParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for AssetTileType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    type Err = AssetTileTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "building" => Ok(AssetTileType::Building),
            "unit" => Ok(AssetTileType::Unit),
            "ground" => Ok(AssetTileType::Ground),
            _ => Err(AssetTileTypeParseError {
                message: format!("'{}' is not a valid AssetTileType", s),
            }),
        }
    }
}
