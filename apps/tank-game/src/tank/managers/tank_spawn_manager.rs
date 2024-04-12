use crate::common::components::unit_id::UnitId;
use crate::common::constants::{Player, SPRITE_SCALE, TANK_HEALTH_BAR_SIZE, TANK_MAX_HEALTH};
use crate::common::resources::unit_id_counter::UnitIdCounter;
use crate::tank::components::tank::Tank;
use crate::tank::components::tank_gun::TankGun;
use crate::tank::components::tank_health::{HealthBar, TankHealth};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct TankSpawnManager;

impl TankSpawnManager {
    pub fn spawn_tank(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        translation: Vec2,
        tank_id_counter: &mut ResMut<UnitIdCounter>,
        player: Player,
    ) {
        let tank_id = tank_id_counter.0;
        tank_id_counter.0 += 1;

        let tank_texture = asset_server.load("sprites/tank_base.png");
        let gun_texture = asset_server.load("sprites/tank3gun.png");
        let tank = Tank::new(tank_id, translation, player.clone());

        // generate a random number between 5.0 and 6.0 with 4 decimal places
        let layer = (5.0 + (rand::random::<f32>() * 1.0)).round() * 10000.0 / 10000.0;

        let tank_base: Entity = commands
            .spawn((SpriteBundle {
                transform: Transform::default()
                    .with_translation(translation.extend(layer))
                    .with_scale(Vec3::splat(SPRITE_SCALE)),
                texture: tank_texture,
                sprite: Sprite {
                    color: tank.get_default_color(),
                    ..default()
                },
                ..default()
            },))
            .insert(tank)
            .insert(TankHealth::new(TANK_MAX_HEALTH as f32))
            .id();

        // Spawn the tank gun as a child of the tank base
        commands.entity(tank_base).with_children(|parent| {
            // Spawn the turret as a child of the tank
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.1)
                        .with_scale(Vec3::splat(SPRITE_SCALE)), // Ensure it's positioned correctly relative to the base
                    texture: gun_texture,
                    ..default()
                })
                .insert(TankGun::new(UnitId(tank_id)));

            // Spawn the health bar as a child of the tank
            parent
                .spawn(SpriteBundle {
                    // Position the health bar above the tank
                    transform: Transform::from_xyz(-50.0, 40.0, 0.2),
                    sprite: Sprite {
                        color: Color::PURPLE, // Health bar color
                        rect: Some(Rect {
                            min: Vec2::new(0.0, 0.0),
                            max: TANK_HEALTH_BAR_SIZE,
                        }),
                        anchor: Anchor::CenterLeft, // Anchor the health bar to the left of the tank
                        ..default()
                    },
                    ..default()
                })
                .insert(HealthBar);
        });
    }

    pub fn despawn_tanks_with_zero_health(mut commands: Commands, query: Query<(Entity, &Tank)>) {
        for (entity, tank) in query.iter() {
            if tank.is_dead() {
                // Despawn the tank entity
                commands.entity(entity).despawn();
            }
        }
    }
}
