use crate::features::unit::{UnitTile, UnitTileType};
use crate::types::player::Player;
use crate::types::{AssetTile, AssetTileSubType, AssetTileType};

#[derive(Debug)]
pub enum UnitTileErrors {
    TileTypeIsRequired,
    TileSubTypeIsRequired,
    InvalidUnitType { message: String },
}

pub fn create_unit_tile(
    value: AssetTile,
    player: Option<Player>,
) -> Result<UnitTile, UnitTileErrors> {
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

    let image_path = value.get_image_path();
    let tile_size = value.get_tile_size();

    Ok(UnitTile::new(image_path, tile_size, unit_tile_type, player))
}
