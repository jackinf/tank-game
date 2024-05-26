use crate::constants::SPRITE_SCALE;
use crate::features::building::components::Building;
use crate::features::building::types::BuildingTile;
use crate::types::player::Player;
use bevy::prelude::{
    default, AssetServer, Commands, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3,
};
use bevy::sprite::Anchor;

pub fn spawn_building(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec2,
    building_tile: BuildingTile,
    map_coord: (usize, usize),
) {
    let sprite_path = building_tile.get_image_path();
    let layer = building_tile.get_layer();

    let player = building_tile.get_player();
    let color = match player {
        Some(Player::P1) => crate::constants::P1_COLOR,
        Some(Player::P2) => crate::constants::P2_COLOR,
        _ => crate::constants::NEUTRAL_COLOR,
    };

    commands
        .spawn((SpriteBundle {
            transform: Transform::default()
                .with_translation(translation.extend(layer))
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            texture: asset_server.load(sprite_path),
            sprite: Sprite {
                color,
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        },))
        .insert(Building::new(building_tile, map_coord, player));
}
