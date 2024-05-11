use std::convert::TryFrom;
use crate::common::constants::TileSize;
use crate::preparation::types::{AssetTile, AssetTileSubType, AssetTileType};

// TODO: consider using trait like CommonTile or GeneralTile
#[derive(Clone, Debug, PartialEq)]
pub struct GroundTile {
    pub tile_type: GroundTileType,
    pub tile_size: TileSize,
    pub image_path: String
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroundTileType {
    Grass = 0,
    Gold = 1,
    Wall = 2,
    Water = 3,
}

#[derive(Debug)]
pub enum GroundTileErrors {
    InvalidTileType,
    InvalidTileSubType,
}

impl TryFrom<AssetTile> for GroundTile {
    type Error = GroundTileErrors;

    fn try_from(value: AssetTile) -> Result<Self, Self::Error> {
        if value.get_tile_type() != AssetTileType::Ground {
            return Err(GroundTileErrors::InvalidTileType);
        }

        let ground_tile_type = match value.get_tile_sub_type() {
            AssetTileSubType::Ground => Ok(GroundTileType::Grass),
            AssetTileSubType::Gold => Ok(GroundTileType::Gold),
            AssetTileSubType::Wall => Ok(GroundTileType::Wall),
            AssetTileSubType::Water => Ok(GroundTileType::Water),
            _ => Err(GroundTileErrors::InvalidTileSubType),
        };
        if ground_tile_type.is_err() {
            return Err(GroundTileErrors::InvalidTileSubType);
        }
        let ground_tile_type = ground_tile_type.unwrap();

        Ok(GroundTile {
            tile_type: ground_tile_type,
            tile_size: value.get_tile_size(),
            image_path: value.get_image_path(),
        })
    }
}

impl GroundTile {
    pub fn get_tile_type_sprite(&self) -> String {
        self.image_path.clone()
    }

    pub fn get_tile_type_layer(&self) -> f32 {
        match self {
            GroundTile::Grass => 0.,
            GroundTile::Gold => 5.,
            GroundTile::Wall => 5.,
            GroundTile::Water => 5.,
        }
    }
}
