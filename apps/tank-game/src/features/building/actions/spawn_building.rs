use crate::components::HealthBar;
use crate::constants::{HEALTH_BAR_HEIGHT, SPRITE_SCALE};
use crate::features::building::components::{Building, UnitSpawner};
use crate::features::building::types::BuildingTile;
use crate::features::unit::UnitIdCounter;
use crate::types::player::Player;
use bevy::math::Rect;
use bevy::prelude::{
    default, AssetServer, BuildChildren, Color, Commands, Res, ResMut, Sprite, SpriteBundle,
    Transform, Vec2, Vec3,
};
use bevy::sprite::Anchor;

pub fn spawn_building(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec2,
    building_tile: BuildingTile,
    map_coord: (usize, usize),
    id_counter: &mut ResMut<UnitIdCounter>,
) {
    let building_id = id_counter.1;
    id_counter.1 += 1;

    let sprite_path = building_tile.get_image_path();
    let layer = building_tile.get_layer();
    let spawn_timer = building_tile.get_spawn_timer().clone();

    let player = building_tile.get_player();
    let color = match player {
        Some(Player::P1) => crate::constants::P1_COLOR,
        Some(Player::P2) => crate::constants::P2_COLOR,
        _ => crate::constants::NEUTRAL_COLOR,
    };
    let health_rect = building_tile.get_health_rect_default();

    commands
        .spawn((SpriteBundle {
            transform: Transform::default()
                .with_translation(translation.extend(layer).clone())
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            texture: asset_server.load(sprite_path),
            sprite: Sprite {
                color,
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            // Spawn the health bar as a child of the tank
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(-10.0, 10.0, 0.2),
                    sprite: Sprite {
                        color: Color::PURPLE, // Health bar color
                        rect: Some(health_rect),
                        anchor: Anchor::CenterLeft, // Anchor the health bar to the left of the tank
                        ..default()
                    },
                    ..default()
                })
                .insert(HealthBar);
        })
        .insert(Building::new(
            building_id.clone(),
            building_tile.clone(),
            map_coord.clone(),
            player.clone(),
        ))
        .insert(UnitSpawner {
            spawn_timer,
            spawn_position: translation,
            player: player.clone(),
        });
}
