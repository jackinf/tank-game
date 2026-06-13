//! Global tunable constants for the game.

use bevy::prelude::Color;

/// Window dimensions.
pub const WINDOW_WIDTH: f32 = 1600.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;

/// Size of a single map tile in world units (pixels at zoom 1.0).
pub const TILE: f32 = 32.0;

/// Z layers, so we get a consistent painter's-order draw.
pub mod z {
    pub const TERRAIN: f32 = 0.0;
    pub const ORE: f32 = 0.5;
    pub const BUILDING: f32 = 1.0;
    pub const RALLY: f32 = 1.5;
    pub const UNIT: f32 = 2.0;
    pub const TURRET: f32 = 2.2;
    pub const PROJECTILE: f32 = 3.0;
    pub const FX: f32 = 4.0;
    pub const LABEL: f32 = 5.0;
}

/// Camera controls.
pub const CAMERA_PAN_SPEED: f32 = 700.0;
pub const CAMERA_EDGE_MARGIN: f32 = 8.0;
pub const CAMERA_ZOOM_SPEED: f32 = 1.5;
pub const CAMERA_ZOOM_MIN: f32 = 0.4;
pub const CAMERA_ZOOM_MAX: f32 = 2.5;

/// Economy.
pub const STARTING_CREDITS: i64 = 5000;
/// Credits earned per unit of ore delivered to a refinery.
pub const CREDITS_PER_ORE: i64 = 5;

/// Faction colours.
pub const PLAYER_COLOR: Color = Color::srgb(0.40, 0.55, 1.0);
pub const ENEMY_COLOR: Color = Color::srgb(1.0, 0.40, 0.35);
pub const NEUTRAL_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

/// Subtle faction wash multiplied over textured sprites so ownership reads at a
/// glance without hiding the artwork. Kept close to white on purpose.
pub const PLAYER_TINT: Color = Color::srgb(0.78, 0.86, 1.0);
pub const ENEMY_TINT: Color = Color::srgb(1.0, 0.82, 0.78);
pub const NEUTRAL_TINT: Color = Color::WHITE;

/// Selection / UI accent.
pub const SELECT_COLOR: Color = Color::srgb(0.2, 1.0, 0.3);

/// UI layout.
pub const SIDEBAR_WIDTH: f32 = 240.0;
pub const TOPBAR_HEIGHT: f32 = 40.0;
