//! Renders the tile map and animates ore patches as they are mined out.

use crate::config::{z, TILE};
use crate::grid::{GameMap, Tile};
use crate::state::{GameEntity, GameState};
use bevy::prelude::*;

#[derive(Component)]
pub struct OreTile(pub Tile);

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ore.run_if(in_state(GameState::Playing)));
    }
}

/// Spawn one sprite per tile for the base terrain, plus an ore overlay sprite
/// for each ore tile.
pub fn spawn_terrain(commands: &mut Commands, map: &GameMap) {
    for row in 0..map.height as i32 {
        for col in 0..map.width as i32 {
            let terrain = map.terrain_at(col, row);
            let pos = map.tile_center(col, row);
            commands.spawn((
                Sprite::from_color(terrain.color(), Vec2::splat(TILE)),
                Transform::from_xyz(pos.x, pos.y, z::TERRAIN),
                GameEntity,
            ));

            let ore = map.ore_at(col, row);
            if ore > 0 {
                let frac = (ore as f32 / 150.0).clamp(0.25, 1.0);
                commands.spawn((
                    OreTile((col, row)),
                    Sprite::from_color(
                        Color::srgb(0.85, 0.72, 0.18),
                        Vec2::splat(TILE * 0.85 * frac),
                    ),
                    Transform::from_xyz(pos.x, pos.y, z::ORE),
                    GameEntity,
                ));
            }
        }
    }
}

fn update_ore(
    mut commands: Commands,
    map: Res<GameMap>,
    mut ore: Query<(Entity, &OreTile, &mut Sprite)>,
) {
    for (entity, tile, mut sprite) in &mut ore {
        let amount = map.ore_at(tile.0 .0, tile.0 .1);
        if amount == 0 {
            commands.entity(entity).despawn();
        } else {
            let frac = (amount as f32 / 150.0).clamp(0.25, 1.0);
            sprite.custom_size = Some(Vec2::splat(TILE * 0.85 * frac));
        }
    }
}
