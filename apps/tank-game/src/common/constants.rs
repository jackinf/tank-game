use crate::common::components::tile::Tile;
use bevy::math::Vec2;
use bevy::prelude::Color;

pub type TileGrid = Vec<Vec<Tile>>;
pub type RawGrid = Vec<Vec<usize>>;
pub type WorldCoord = (f32, f32);
pub type TileCoord = (usize, usize);
pub type TileSize = (usize, usize);

pub const MAX_WIDTH: u16 = 1600;
pub const MAX_HEIGHT: u16 = 1000;
pub const TILE_SIZE: f32 = 64.0;
pub const SPRITE_SCALE: f32 = 0.5;
pub const OFFSET_X: f32 = -0.0;
pub const OFFSET_Y: f32 = -0.0;

pub const TANK_FULL_HEALTH_BAR_WIDTH: f32 = 100.0;
pub const TANK_HEALTH_BAR_HEIGHT: f32 = 20.0;
pub const TANK_HEALTH_BAR_SIZE: Vec2 =
    Vec2::new(TANK_FULL_HEALTH_BAR_WIDTH, TANK_HEALTH_BAR_HEIGHT);
pub const TANK_MAX_HEALTH: u32 = 100;
pub const TANK_ROTATION_SPEED: f32 = 10.0;
pub const BULLET_RADIUS: f32 = 10.0;

pub const CAMERA_SPEED: f32 = 10.0;
pub const SIDE_MARGIN_PERCENTAGE: f32 = 0.1;

pub const P1_COLOR: Color = Color::rgba(0.7, 0.7, 1.0, 1.);
pub const P2_COLOR: Color = Color::rgba(1.0, 0.7, 0.7, 1.);

pub enum TileType {
    Grass = 0,
    Gold = 1,
    Wall = 2,
    Water = 3,
}

impl TryFrom<usize> for TileType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TileType::Grass),
            1 => Ok(TileType::Gold),
            2 => Ok(TileType::Wall),
            3 => Ok(TileType::Water),
            _ => Err(()),
        }
    }
}

pub enum UnitType {
    Tank = 1,
    Soldier = 2,
    Harvester = 3,
}

#[derive(Clone)]
pub enum Player {
    P1 = 1,
    P2 = 2,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Player::P1, Player::P1) => true,
            (Player::P2, Player::P2) => true,
            _ => false,
        }
    }
}
