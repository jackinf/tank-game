use crate::constants::TILE_SIZE;
use crate::features::building::components::BuildingPlacementTiles;
use bevy::prelude::{
    default, AssetServer, Color, Commands, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3,
};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // selector entity for placing buildings
    commands
        .spawn((SpriteBundle {
            texture: asset_server.load("pixels/white.png"),
            transform: Transform::default()
                .with_translation(Vec3::new(0., 0., 100.))
                .with_scale(Vec2::new(2.0 * TILE_SIZE, 2.0 * TILE_SIZE).extend(1.0)),
            sprite: Sprite {
                color: Color::PINK.with_a(0.0), // hide by default
                ..default()
            },
            ..default()
        },))
        .insert(BuildingPlacementTiles::new());
}
