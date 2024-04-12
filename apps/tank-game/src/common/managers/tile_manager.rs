use crate::common::components::tile::Tile;
use crate::common::constants::{TileType, SPRITE_SCALE, TILE_SIZE};
use crate::common::utils::enum_helpers::EnumHelpers;
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Commands, Res, SpriteBundle, Transform};

pub struct TileManager;

impl TileManager {
    pub fn get_tile_type_sprite(tile_type: &TileType) -> String {
        match tile_type {
            TileType::Wall => "sprites/tiles/wall.png".into(),
            TileType::Grass => "sprites/tiles/grass3.png".into(),
            TileType::Gold => "sprites/tiles/grass3.png".into(),
            TileType::Water => "sprites/tiles/water.png".into(),
        }
    }

    pub fn get_tile_type_layer(tile_type: &TileType) -> f32 {
        match tile_type {
            TileType::Grass => 0.,
            TileType::Gold => 0.,
            TileType::Wall => 10.,
            TileType::Water => 0.,
        }
    }

    pub fn spawn_tile(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        translation: Vec2,
        tile_type_raw: usize,
        map_coord: (usize, usize),
    ) -> Tile {
        let tile_type: TileType = EnumHelpers::assert_valid_enum::<TileType>(tile_type_raw);
        let sprite_path = TileManager::get_tile_type_sprite(&tile_type);
        let layer = TileManager::get_tile_type_layer(&tile_type);

        let tile = Tile::new(
            Vec2::new(translation.x, translation.y),
            TILE_SIZE,
            TILE_SIZE,
            tile_type_raw,
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
