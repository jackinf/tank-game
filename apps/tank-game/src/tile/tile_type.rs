use crate::common::constants::{TileCoord, TileSize};
use crate::preparation::types::{AssetTile, AssetTileSubType, AssetTileType};
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct GroundTile {
    ground_type: GroundTileType,
    tile_size: TileSize,
    image_path: String,
}

impl GroundTile {
    pub fn get_ground_type(&self) -> GroundTileType {
        self.ground_type.clone()
    }

    pub fn get_tile_type_sprite(&self) -> String {
        self.image_path.clone()
    }

    pub fn get_tile_type_layer(&self) -> f32 {
        match self.ground_type {
            GroundTileType::Grass => 0.,
            GroundTileType::Gold => 5.,
            GroundTileType::Wall => 5.,
            GroundTileType::Water => 5.,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
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
            ground_type: ground_tile_type,
            tile_size: value.get_tile_size(),
            image_path: value.get_image_path(),
        })
    }
}
