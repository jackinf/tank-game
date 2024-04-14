use crate::common::constants::{SPRITE_SCALE, TILE_SIZE};
use crate::common::utils::enum_helpers::EnumHelpers;
use crate::tile::components::tile::Tile;
use crate::tile::tile_type::TileType;
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Commands, Res, SpriteBundle, Transform};

pub struct TileManager;

impl TileManager {
    pub fn spawn_tile(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        translation: Vec2,
        tile_type: TileType,
        map_coord: (usize, usize),
    ) -> Tile {
        let sprite_path = tile_type.get_tile_type_sprite();
        let layer = tile_type.get_tile_type_layer();

        let tile = Tile::new(
            Vec2::new(translation.x, translation.y),
            TILE_SIZE,
            TILE_SIZE,
            tile_type as usize,
            map_coord,
        );

        commands
            .spawn((SpriteBundle {
                transform: Transform::default()
                    .with_translation(translation.extend(layer))
                    .with_scale(Vec3::splat(SPRITE_SCALE)),
                texture: asset_server.load(sprite_path),
                ..default()
            },))
            .insert(tile.clone());

        tile
    }
}
