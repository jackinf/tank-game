use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
    P1,
    P2,
}

#[derive(Debug, Clone)]
pub struct AssetTileSubTypeParseError {
    message: String,
}

impl Display for AssetTileSubTypeParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
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
            AssetTileSubType::P1 => write!(f, "p1"),
            AssetTileSubType::P2 => write!(f, "p2"),
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
            (AssetTileSubType::Soldier, AssetTileSubType::Soldier) => true,
            (AssetTileSubType::Harvester, AssetTileSubType::Harvester) => true,
            (AssetTileSubType::P1, AssetTileSubType::P1) => true,
            (AssetTileSubType::P2, AssetTileSubType::P2) => true,
            _ => false,
        }
    }
}

impl FromStr for AssetTileSubType {
    type Err = AssetTileSubTypeParseError;

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
            "soldier" => Ok(AssetTileSubType::Soldier),
            "harvester" => Ok(AssetTileSubType::Harvester),
            "p1" => Ok(AssetTileSubType::P1),
            "p2" => Ok(AssetTileSubType::P2),
            _ => Err(AssetTileSubTypeParseError {
                message: format!("'{}' is not a valid AssetTileSubType", s),
            }),
        }
    }
}
