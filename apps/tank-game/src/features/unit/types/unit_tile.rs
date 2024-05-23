use crate::constants::TileSize;
use crate::features::unit::types::UnitTileType;
use crate::types::player::Player;

#[derive(Clone, Debug, PartialEq)]
pub struct UnitTile {
    image_path: String,
    tile_size: TileSize,
    unit_type: UnitTileType,
    player: Option<Player>,
}

impl UnitTile {
    pub fn new(
        image_path: String,
        tile_size: TileSize,
        unit_type: UnitTileType,
        player: Option<Player>,
    ) -> Self {
        UnitTile {
            image_path,
            tile_size,
            unit_type,
            player,
        }
    }

    pub fn get_image_path(&self) -> String {
        self.image_path.clone()
    }

    pub fn get_tile_size(&self) -> TileSize {
        self.tile_size.clone()
    }

    pub fn get_unit_type(&self) -> UnitTileType {
        self.unit_type.clone()
    }

    pub fn get_player(&self) -> Option<Player> {
        self.player.clone()
    }
}
