use crate::constants::{SPRITE_SCALE, HEALTH_BAR_SIZE, TANK_MAX_HEALTH};
use crate::features::tank::components::{Tank, TankGun, TankHealth};
use crate::features::unit::{UnitId, UnitIdCounter};
use crate::types::player::Player;
use crate::utils::common_helpers::CommonHelpers;
use bevy::asset::ErasedAssetLoader;
use bevy::math::Quat;
use bevy::prelude::{
    default, AssetServer, BuildChildren, Color, Commands, Rect, Res, ResMut, Sprite, SpriteBundle,
    Transform, Vec2, Vec3,
};
use bevy::sprite::Anchor;
use bevy_prototype_lyon::prelude::{GeometryBuilder, ShapeBundle, Stroke};
use bevy_prototype_lyon::shapes;
use std::f32::consts::PI;
use crate::components::HealthBar;

pub fn spawn_tank(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec2,
    tank_id_counter: &mut ResMut<UnitIdCounter>,
    player: Option<Player>,
) {
    let tank_id = tank_id_counter.0;
    tank_id_counter.0 += 1;

    let tank_texture = asset_server.load("sprites/tank_base2_bw.png");
    let gun_texture = asset_server.load("sprites/tank_head_bw.png");
    let tank = Tank::new(tank_id, translation, player.clone());
    let layer = CommonHelpers::calculate_random_layer(5.0);

    let tank_radius = tank.get_radius().clone();

    let color = tank.get_default_color().clone();
    commands
        .spawn((SpriteBundle {
            transform: Transform::default()
                .with_translation(translation.extend(layer))
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            texture: tank_texture,
            sprite: Sprite {
                color: color.clone(),
                ..default()
            },
            ..default()
        },))
        .insert(tank)
        .insert(TankHealth::new(TANK_MAX_HEALTH as f32))
        .with_children(move |parent| {
            // Spawn the turret as a child of the tank
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.1)
                        .with_rotation(Quat::from_rotation_z(PI))
                        .with_scale(Vec3::splat(SPRITE_SCALE)), // Ensure it's positioned correctly relative to the base
                    texture: gun_texture,
                    sprite: Sprite {
                        color: color * 2.0,
                        ..default()
                    },
                    ..default()
                })
                .insert(TankGun::new(UnitId(tank_id)));

            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle {
                        radius: tank_radius / SPRITE_SCALE,
                        ..default()
                    }),
                    ..default()
                },
                // Fill::color(Color::CYAN),
                Stroke::new(Color::BLACK, 2.0),
            ));

            // Spawn the health bar as a child of the tank
            parent
                .spawn(SpriteBundle {
                    // Position the health bar above the tank
                    transform: Transform::from_xyz(-50.0, 40.0, 0.2),
                    sprite: Sprite {
                        color: Color::PURPLE, // Health bar color
                        rect: Some(Rect {
                            min: Vec2::new(0.0, 0.0),
                            max: HEALTH_BAR_SIZE,
                        }),
                        anchor: Anchor::CenterLeft, // Anchor the health bar to the left of the tank
                        ..default()
                    },
                    ..default()
                })
                .insert(HealthBar);
        });
}
