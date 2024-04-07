use crate::common::tile::Tile;
use bevy::math::Vec2;

pub type Grid = Vec<Vec<Tile>>;
pub type WorldCoordinates = (f32, f32);
pub type TileCoordinates = (usize, usize);

pub const MAX_WIDTH: u16 = 1600;
pub const MAX_HEIGHT: u16 = 1000;
pub const TILE_SIZE: f32 = 64.0;
pub const SPRITE_SCALE: f32 = 0.5;
pub const TILE_TANK: usize = 1;
pub const TILE_GRASS: usize = 0;
pub const TILE_WALL: usize = 2;
pub const TILE_WATER: usize = 3;
pub const OFFSET_X: f32 = -0.0;
pub const OFFSET_Y: f32 = -0.0;

pub const TANK_FULL_HEALTH_BAR_WIDTH: f32 = 100.0;
pub const TANK_HEALTH_BAR_HEIGHT: f32 = 20.0;
pub const TANK_HEALTH_BAR_SIZE: Vec2 =
    Vec2::new(TANK_FULL_HEALTH_BAR_WIDTH, TANK_HEALTH_BAR_HEIGHT);
pub const TANK_MAX_HEALTH: u32 = 100;

pub const CAMERA_SPEED: f32 = 10.0;
pub const SIDE_MARGIN_PERCENTAGE: f32 = 0.1;
