//! Renders the tile map and animates ore patches as they are mined out.
//!
//! The ground is drawn from a couple of seamless grass textures. To stop the
//! grid from reading as an obvious repeat, every tile mirrors and brightness-
//! jitters its texture deterministically from its coordinates (stable across
//! reloads), and the whole texture is muted so units stay readable on top.

use crate::config::{z, GRASS_MUTE, TILE};
use crate::grid::{GameMap, Terrain, Tile};
use crate::state::{GameEntity, GameState};
use bevy::prelude::*;

const GRASS_VARIANTS: usize = 2;
const TREE_VARIANTS: usize = 4;
const DECAL_VARIANTS: usize = 12;
/// Roughly what fraction of plain grass tiles get a scattered rock/dirt decal.
const DECAL_PERCENT: u32 = 9;

#[derive(Component)]
pub struct OreTile(pub Tile);

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ore.run_if(in_state(GameState::Playing)));
    }
}

/// Cheap deterministic hash of a tile coordinate, used to pick per-tile texture
/// variant / mirroring / brightness so the ground never looks gridded.
fn tile_hash(col: i32, row: i32) -> u32 {
    let mut h = (col as u32).wrapping_mul(73856093) ^ (row as u32).wrapping_mul(19349663);
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1_e995);
    h ^ (h >> 15)
}

/// Spawn one sprite per tile for the base terrain, an ore overlay for each ore
/// tile, and a tree on each forest tile.
pub fn spawn_terrain(commands: &mut Commands, asset_server: &AssetServer, map: &GameMap) {
    let grass: Vec<Handle<Image>> = (0..GRASS_VARIANTS)
        .map(|i| asset_server.load(format!("terrain/grass_{i}.png")))
        .collect();
    let trees: Vec<Handle<Image>> = (0..TREE_VARIANTS)
        .map(|i| asset_server.load(format!("trees/tree_{i}.png")))
        .collect();
    let decals: Vec<Handle<Image>> = (0..DECAL_VARIANTS)
        .map(|i| asset_server.load(format!("decals/decal_{i:02}.png")))
        .collect();
    let ore_tex = asset_server.load("terrain/ore_0.png");
    let mute = GRASS_MUTE.to_srgba();

    for row in 0..map.height as i32 {
        for col in 0..map.width as i32 {
            let terrain = map.terrain_at(col, row);
            let pos = map.tile_center(col, row);
            let h = tile_hash(col, row);

            // Base ground tile.
            if terrain.is_grassy() {
                // Mirror + brightness jitter so the repeat disappears. The grass
                // is busy enough that mirroring hides its own seams too.
                let jitter = 0.92 + (h >> 6 & 0xff) as f32 / 255.0 * 0.16;
                commands.spawn((
                    Sprite {
                        image: grass[(h >> 2) as usize % GRASS_VARIANTS].clone(),
                        custom_size: Some(Vec2::splat(TILE)),
                        color: Color::srgb(
                            mute.red * jitter,
                            mute.green * jitter,
                            mute.blue * jitter,
                        ),
                        flip_x: h & 1 != 0,
                        flip_y: h & 2 != 0,
                        ..default()
                    },
                    Transform::from_xyz(pos.x, pos.y, z::TERRAIN),
                    GameEntity,
                ));
            } else {
                commands.spawn((
                    Sprite::from_color(terrain.color(), Vec2::splat(TILE)),
                    Transform::from_xyz(pos.x, pos.y, z::TERRAIN),
                    GameEntity,
                ));
            }

            // Ore overlay (shrinks as it is mined, revealing the grass beneath).
            let ore = map.ore_at(col, row);
            if ore > 0 {
                let frac = (ore as f32 / 150.0).clamp(0.25, 1.0);
                commands.spawn((
                    OreTile((col, row)),
                    Sprite {
                        image: ore_tex.clone(),
                        custom_size: Some(Vec2::splat(TILE * frac)),
                        ..default()
                    },
                    Transform::from_xyz(pos.x, pos.y, z::ORE),
                    GameEntity,
                ));
            }

            // Trees on forest tiles, overhanging the tile for a fuller canopy.
            // Mirrored per tile so a grove of them does not look cloned.
            if terrain == Terrain::Forest {
                commands.spawn((
                    Sprite {
                        image: trees[(h >> 4) as usize % TREE_VARIANTS].clone(),
                        custom_size: Some(Vec2::splat(TILE * 1.7)),
                        flip_x: h & 4 != 0,
                        ..default()
                    },
                    Transform::from_xyz(pos.x, pos.y, z::TREE),
                    GameEntity,
                ));
            }

            // Sparse rock/dirt decals on plain grass, to break up the texture.
            // A second hash decorrelates the scatter from the grass variation.
            if terrain == Terrain::Grass {
                let dh = tile_hash(col + 9973, row + 9973);
                if dh % 100 < DECAL_PERCENT {
                    let off = |bits: u32| ((bits & 0xff) as f32 / 255.0 - 0.5) * TILE * 0.5;
                    let size = TILE * (0.45 + (dh >> 24 & 0xff) as f32 / 255.0 * 0.4);
                    commands.spawn((
                        Sprite {
                            image: decals[(dh >> 8) as usize % DECAL_VARIANTS].clone(),
                            custom_size: Some(Vec2::splat(size)),
                            color: Color::srgba(1.0, 1.0, 1.0, 0.9),
                            flip_x: dh & 1 != 0,
                            ..default()
                        },
                        Transform::from_xyz(pos.x + off(dh >> 12), pos.y + off(dh >> 18), z::DECAL),
                        GameEntity,
                    ));
                }
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
