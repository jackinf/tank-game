use crate::components::HealthBar;
use crate::constants::{HEALTH_BAR_SIZE, SPRITE_SCALE, TANK_MAX_HEALTH};
use crate::features::tank::components::{Tank, TankGun, TankHealth};
use crate::features::tank::TankStrategy;
use crate::features::unit::{UnitId, UnitIdCounter};
use crate::types::player::Player;
use crate::utils::common_helpers::CommonHelpers;
use bevy::color::Srgba;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::PI;

pub fn spawn_tank(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec2,
    tank_id_counter: &mut ResMut<UnitIdCounter>,
    player: Option<Player>,
    strategy: TankStrategy,
) {
    let tank_id = tank_id_counter.0;
    tank_id_counter.0 += 1;

    let tank_texture = asset_server.load("sprites/tank_base2_bw.png");
    let gun_texture = asset_server.load("sprites/tank_head_bw.png");
    let tank = Tank::new(tank_id, translation, player.clone(), strategy);
    let layer = CommonHelpers::calculate_random_layer(5.0);

    let color = tank.get_default_color().clone();
    // Convert to Srgba to get components for brightening
    let color_srgba: Srgba = color.into();
    let brighter_color = Color::srgb(
        (color_srgba.red * 2.0).min(1.0),
        (color_srgba.green * 2.0).min(1.0),
        (color_srgba.blue * 2.0).min(1.0),
    );
    
    commands
        .spawn((
            Sprite {
                image: tank_texture,
                color: color.clone(),
                ..default()
            },
            Transform::default()
                .with_translation(translation.extend(layer))
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            tank,
            TankHealth::new(TANK_MAX_HEALTH as f32),
        ))
        .with_children(move |parent| {
            // Spawn the turret as a child of the tank
            parent.spawn((
                Sprite {
                    image: gun_texture,
                    color: brighter_color,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.1)
                    .with_rotation(Quat::from_rotation_z(PI))
                    .with_scale(Vec3::splat(SPRITE_SCALE)),
                TankGun::new(UnitId(tank_id)),
            ));

            // Debug circle removed - bevy_prototype_lyon has compatibility issues
            // Can use Bevy gizmos for runtime debug visualization if needed

            // Spawn the health bar as a child of the tank
            parent.spawn((
                Sprite {
                    color: Color::from(bevy::color::palettes::css::PURPLE),
                    custom_size: Some(HEALTH_BAR_SIZE),
                    ..default()
                },
                Anchor::CENTER_LEFT,
                Transform::from_xyz(-50.0, 40.0, 0.2),
                HealthBar,
            ));
        });
}
