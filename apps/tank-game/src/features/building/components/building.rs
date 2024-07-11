use std::collections::HashSet;

use crate::actions::calculate_tile_world_position::calculate_tile_to_world_position;
use bevy::prelude::{Component, Rect, Vec2};

use crate::constants::{TileCoord, TileSize, TILE_SIZE};
use crate::features::building::types::{BuildingTile, BuildingTileType};
use crate::features::con_menu::MenuInfo;
use crate::features::unit::UnitId;
use crate::types::player::Player;

#[derive(Component, Clone)]
pub struct Building {
    id: UnitId,
    building_tile: BuildingTile,
    building_tile_coord: TileCoord,
    building_tiles: HashSet<TileCoord>,
    player: Option<Player>,
    health: u32,
}

impl Building {
    pub fn new(
        id: usize,
        building_tile: BuildingTile,
        building_tile_coord: TileCoord,
        player: Option<Player>,
    ) -> Self {
        let building_tiles: HashSet<TileCoord> =
            calculate_all_building_tiles(building_tile_coord, building_tile.get_size());
        let health = building_tile.get_max_health();

        Building {
            id: UnitId(id),
            building_tile,
            building_tile_coord,
            building_tiles,
            player,
            health,
        }
    }

    pub fn id(&self) -> UnitId {
        self.id.clone()
    }

    pub fn get_building_tile(&self) -> BuildingTile {
        self.building_tile.clone()
    }

    pub fn get_building_tile_type(&self) -> BuildingTileType {
        self.building_tile.get_building_type()
    }

    pub fn get_door(&self) -> TileCoord {
        self.building_tile_coord
    }

    pub fn get_building_tiles(&self) -> HashSet<TileCoord> {
        self.building_tiles.clone()
    }

    pub fn radius(&self) -> f32 {
        self.building_tile.radius()
    }

    pub fn get_player(&self) -> Option<Player> {
        self.player.clone()
    }

    pub fn contains(&self, current: TileCoord) -> bool {
        self.building_tiles.contains(&current)
    }

    pub fn is_power_plant(&self) -> bool {
        self.building_tile.get_building_type() == BuildingTileType::PowerPlant
    }

    pub fn get_max_health(&self) -> u32 {
        self.building_tile.get_max_health().clone()
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }

    pub fn damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }

    pub fn is_destroyed(&self) -> bool {
        self.health <= 0
    }

    pub fn is_mine(&self, me: &MenuInfo) -> bool {
        if let Some(player) = self.player.clone() {
            player == me.player()
        } else {
            false
        }
    }

    pub fn center(&self) -> Vec2 {
        let top_left = calculate_tile_to_world_position(&self.building_tile_coord);

        let (width, height) = self.building_tile.get_size();
        let center_x = top_left.x + width as f32 * 0.5;
        let center_y = top_left.y + height as f32 * 0.5;

        Vec2::new(center_x, center_y)
    }

    pub fn get_outer_tiles(&self) -> HashSet<TileCoord> {
        let mut outer_tiles = HashSet::new();

        for tile in self.building_tiles.iter() {
            // Iterating from -1 to 1, convert this to check for underflow and overflow
            for x in 0..=2 {
                for y in 0..=2 {
                    // Convert from 0..=2 range to -1..=1 by subtracting 1
                    let dx = x as isize - 1;
                    let dy = y as isize - 1;

                    if let (Some(nx), Some(ny)) = (
                        (tile.0 as isize)
                            .checked_add(dx)
                            .and_then(|x| usize::try_from(x).ok()),
                        (tile.1 as isize)
                            .checked_add(dy)
                            .and_then(|y| usize::try_from(y).ok()),
                    ) {
                        outer_tiles.insert((nx, ny));
                    }
                }
            }
        }

        // other buildings cannot be placed on the building itself
        for tile in self.building_tiles.iter() {
            outer_tiles.remove(tile);
        }

        outer_tiles
    }

    pub fn get_size(&self) -> TileSize {
        self.building_tile.get_size()
    }
}

fn calculate_all_building_tiles(start: TileCoord, size: TileSize) -> HashSet<TileCoord> {
    let (start_x, start_y) = start;
    let (width, height) = size;

    let mut building_tiles = HashSet::new();
    for x in start_x..start_x + width {
        for y in start_y..start_y + height {
            building_tiles.insert((x, y));
        }
    }

    building_tiles
}
