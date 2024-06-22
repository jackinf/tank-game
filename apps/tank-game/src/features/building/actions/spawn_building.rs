use bevy::math::Rect;
use crate::constants::{SPRITE_SCALE, HEALTH_BAR_HEIGHT};
use crate::features::building::components::Building;
use crate::features::building::types::BuildingTile;
use crate::types::player::Player;
use bevy::prelude::{default, AssetServer, Commands, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, Color, BuildChildren, ResMut};
use bevy::sprite::Anchor;
use crate::components::HealthBar;
use crate::features::unit::UnitIdCounter;

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

    let player = building_tile.get_player();
    let color = match player {
        Some(Player::P1) => crate::constants::P1_COLOR,
        Some(Player::P2) => crate::constants::P2_COLOR,
        _ => crate::constants::NEUTRAL_COLOR,
    };
    let tile_width_units = building_tile.get_size().0;
    let tile_width = 130. * tile_width_units as f32;

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
        .with_children(|parent| {
            // Spawn the health bar as a child of the tank
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(-10.0, 10.0, 0.2),
                    sprite: Sprite {
                        color: Color::PURPLE, // Health bar color
                        rect: Some(Rect {
                            min: Vec2::new(0.0, 0.0),
                            max: Vec2::new(tile_width, HEALTH_BAR_HEIGHT),
                        }),
                        anchor: Anchor::CenterLeft, // Anchor the health bar to the left of the tank
                        ..default()
                    },
                    ..default()
                })
                .insert(HealthBar);
        })
        .insert(Building::new(building_id, building_tile, map_coord, player, 100));
}
