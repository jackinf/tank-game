use crate::constants::TileCoord;
use crate::types::mission_layer::MissionLayer;
use crate::types::player::Player;
use crate::types::AssetTileSubType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PlayersLayer {
    tiles: HashMap<TileCoord, Option<Player>>,
}

impl PlayersLayer {
    pub(crate) fn get_by(&self, coord: &TileCoord) -> Option<Player> {
        self.tiles
            .get(&coord)
            .map(|player| player.clone())
            .flatten()
    }
}

impl Into<PlayersLayer> for MissionLayer {
    fn into(self) -> PlayersLayer {
        PlayersLayer {
            tiles: self
                .get_tiles()
                .iter()
                .filter_map(|(coord, tile)| match tile.get_tile_sub_type()? {
                    AssetTileSubType::P1 => Some((*coord, Some(Player::P1))),
                    AssetTileSubType::P2 => Some((*coord, Some(Player::P2))),
                    _ => None,
                })
                .collect(),
        }
    }
}
