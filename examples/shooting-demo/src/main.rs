//! Shooting Demo
//!
//! Demonstrates tank combat mechanics:
//! - Player tank (blue) vs enemy tank (red) and enemy building (dark red)
//! - Press SPACE to select a target (cycles through enemies)
//! - Tank automatically shoots at the target until it dies
//! - Shows explosion animation and plays sound on impact
//! - Entities despawn when HP reaches 0

use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

// Constants
const TILE_SIZE: f32 = 64.0;
const BULLET_SPEED: f32 = 400.0;
const BULLET_DAMAGE: u32 = 25;
const TANK_SHOOT_COOLDOWN: f32 = 1.0;
const TANK_RANGE: f32 = 300.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Shooting Demo - SPACE to cycle targets".into(),
                resolution: bevy::window::WindowResolution::new(800, 600),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<TargetSelection>()
        .add_systems(
            Startup,
            (setup_camera, setup_entities, setup_explosion_animation, setup_ui).chain(),
        )
        .add_systems(
            Update,
            (
                handle_target_selection,
                shoot_at_target,
                move_bullets,
                check_bullet_hits,
                play_explosion,
                cleanup_dead_entities,
                update_health_bars,
                update_info_text,
            ),
        )
        .run();
}

// Components
#[derive(Component)]
struct PlayerTank {
    target: Option<Entity>,
    cooldown_timer: Timer,
}

#[derive(Component)]
struct EnemyTank;

#[derive(Component)]
struct EnemyBuilding;

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct HealthBarBackground;

#[derive(Component)]
struct Bullet {
    target_pos: Vec2,
    damage: u32,
}

#[derive(Component)]
struct Targetable;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationActive(bool);

#[derive(Component)]
struct Explosion;

#[derive(Component)]
struct InfoText;

// Resources
#[derive(Resource, Default)]
struct TargetSelection {
    current_index: usize,
}

#[derive(Resource)]
struct ExplosionAssets {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_translation(Vec3::new(400.0, 300.0, 1000.0))));
}

fn setup_entities(mut commands: Commands) {
    // Player tank (blue) on the left
    let player_pos = Vec2::new(150.0, 300.0);
    let player_entity = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.2, 0.4, 0.8),
                custom_size: Some(Vec2::splat(TILE_SIZE - 8.0)),
                ..default()
            },
            Transform::from_translation(player_pos.extend(10.0)),
            PlayerTank {
                target: None,
                cooldown_timer: Timer::from_seconds(TANK_SHOOT_COOLDOWN, TimerMode::Once),
            },
            Health {
                current: 100,
                max: 100,
            },
        ))
        .id();

    spawn_health_bar(&mut commands, player_entity, player_pos);

    // Enemy tank (red) on the right
    let enemy_tank_pos = Vec2::new(500.0, 400.0);
    let enemy_tank = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.8, 0.2, 0.2),
                custom_size: Some(Vec2::splat(TILE_SIZE - 8.0)),
                ..default()
            },
            Transform::from_translation(enemy_tank_pos.extend(10.0)),
            EnemyTank,
            Targetable,
            Health {
                current: 100,
                max: 100,
            },
        ))
        .id();

    spawn_health_bar(&mut commands, enemy_tank, enemy_tank_pos);

    // Enemy building (dark red) on the right
    let building_pos = Vec2::new(600.0, 200.0);
    let building = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.5, 0.1, 0.1),
                custom_size: Some(Vec2::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0)),
                ..default()
            },
            Transform::from_translation(building_pos.extend(5.0)),
            EnemyBuilding,
            Targetable,
            Health {
                current: 200,
                max: 200,
            },
        ))
        .id();

    spawn_health_bar(&mut commands, building, building_pos);
}

fn spawn_health_bar(commands: &mut Commands, parent: Entity, pos: Vec2) {
    let bar_width = 50.0;
    let bar_height = 6.0;
    let bar_y_offset = 40.0;

    // Background (red)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.1, 0.1),
            custom_size: Some(Vec2::new(bar_width, bar_height)),
            ..default()
        },
        Transform::from_translation(Vec3::new(pos.x, pos.y + bar_y_offset, 15.0)),
        HealthBarBackground,
        TargetOf(parent),
    ));

    // Foreground (green)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.7, 0.1),
            custom_size: Some(Vec2::new(bar_width, bar_height)),
            ..default()
        },
        Transform::from_translation(Vec3::new(pos.x, pos.y + bar_y_offset, 16.0)),
        HealthBar,
        TargetOf(parent),
    ));
}

#[derive(Component)]
struct TargetOf(Entity);

fn setup_explosion_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("animations/explosion.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(31, 35), 5, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(ExplosionAssets {
        texture,
        layout: layout_handle,
    });
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Press SPACE to cycle targets\nTank will auto-shoot when target selected"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        InfoText,
    ));
}

fn handle_target_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<TargetSelection>,
    mut player_query: Query<&mut PlayerTank>,
    targets_query: Query<Entity, With<Targetable>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let targets: Vec<Entity> = targets_query.iter().collect();
    if targets.is_empty() {
        if let Ok(mut player) = player_query.single_mut() {
            player.target = None;
        }
        return;
    }

    selection.current_index = (selection.current_index + 1) % (targets.len() + 1);

    if let Ok(mut player) = player_query.single_mut() {
        if selection.current_index == 0 {
            player.target = None;
        } else {
            player.target = Some(targets[selection.current_index - 1]);
        }
    }
}

fn shoot_at_target(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut player_query: Query<(&mut PlayerTank, &Transform)>,
    targets_query: Query<&Transform, With<Targetable>>,
) {
    let Ok((mut player, player_transform)) = player_query.single_mut() else {
        return;
    };

    player.cooldown_timer.tick(time.delta());

    let Some(target_entity) = player.target else {
        return;
    };

    let Ok(target_transform) = targets_query.get(target_entity) else {
        player.target = None;
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let target_pos = target_transform.translation.truncate();
    let distance = player_pos.distance(target_pos);

    // Check range
    if distance > TANK_RANGE {
        return;
    }

    // Check cooldown
    if !player.cooldown_timer.finished() {
        return;
    }

    // Reset cooldown
    player.cooldown_timer.reset();

    // Spawn bullet
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.0),
            custom_size: Some(Vec2::splat(8.0)),
            ..default()
        },
        Transform::from_translation(player_pos.extend(20.0)),
        Bullet {
            target_pos,
            damage: BULLET_DAMAGE,
        },
    ));

    // Play shoot sound
    commands.spawn(AudioPlayer::new(asset_server.load("sounds/explosion.ogg")));
}

fn move_bullets(time: Res<Time>, mut bullets_query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in bullets_query.iter_mut() {
        let current_pos = transform.translation.truncate();
        let direction = bullet.target_pos - current_pos;

        if direction.length() > 0.1 {
            let movement = direction.normalize() * BULLET_SPEED * time.delta_secs();
            transform.translation += movement.extend(0.0);
        }
    }
}

fn check_bullet_hits(
    mut commands: Commands,
    explosion_assets: Res<ExplosionAssets>,
    bullets_query: Query<(Entity, &Bullet, &Transform)>,
    mut targets_query: Query<(&mut Health, &Transform), With<Targetable>>,
) {
    for (bullet_entity, bullet, bullet_transform) in bullets_query.iter() {
        let bullet_pos = bullet_transform.translation.truncate();

        // Check if bullet reached target
        if bullet_pos.distance(bullet.target_pos) < 10.0 {
            // Despawn bullet
            commands.entity(bullet_entity).despawn();

            // Find and damage target at this position
            for (mut health, target_transform) in targets_query.iter_mut() {
                let target_pos = target_transform.translation.truncate();
                if target_pos.distance(bullet.target_pos) < 50.0 {
                    health.current = health.current.saturating_sub(bullet.damage);

                    // Spawn explosion
                    spawn_explosion(&mut commands, &explosion_assets, bullet.target_pos);
                    break;
                }
            }
        }
    }
}

fn spawn_explosion(commands: &mut Commands, assets: &ExplosionAssets, pos: Vec2) {
    commands.spawn((
        Sprite {
            image: assets.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(pos.extend(25.0)).with_scale(Vec3::splat(2.0)),
        AnimationIndices { first: 0, last: 4 },
        AnimationTimer(Timer::new(Duration::from_millis(50), TimerMode::Repeating)),
        AnimationActive(true),
        Explosion,
    ));
}

fn play_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &AnimationIndices,
        &mut AnimationTimer,
        &mut Sprite,
        &mut AnimationActive,
    )>,
) {
    for (entity, indices, mut timer, mut sprite, mut active) in &mut query {
        if active.0 {
            if timer.tick(time.delta()).just_finished() {
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    if atlas.index >= indices.last {
                        commands.entity(entity).despawn();
                    } else {
                        atlas.index += 1;
                    }
                }
            }
        }
    }
}

fn cleanup_dead_entities(
    mut commands: Commands,
    mut player_query: Query<&mut PlayerTank>,
    dead_query: Query<(Entity, &Health), With<Targetable>>,
    health_bars: Query<(Entity, &TargetOf)>,
) {
    for (entity, health) in dead_query.iter() {
        if health.current == 0 {
            // Clear player target if it was this entity
            if let Ok(mut player) = player_query.single_mut() {
                if player.target == Some(entity) {
                    player.target = None;
                }
            }

            // Despawn health bars
            for (bar_entity, target_of) in health_bars.iter() {
                if target_of.0 == entity {
                    commands.entity(bar_entity).despawn();
                }
            }

            commands.entity(entity).despawn();
        }
    }
}

fn update_health_bars(
    health_query: Query<(Entity, &Health, &Transform)>,
    mut bar_query: Query<(&mut Transform, &mut Sprite, &TargetOf), (With<HealthBar>, Without<Health>)>,
    mut bg_query: Query<(&mut Transform, &TargetOf), (With<HealthBarBackground>, Without<Health>, Without<HealthBar>)>,
) {
    let health_map: HashMap<Entity, (&Health, &Transform)> = health_query
        .iter()
        .map(|(e, h, t)| (e, (h, t)))
        .collect();

    for (mut bar_transform, mut bar_sprite, target_of) in bar_query.iter_mut() {
        if let Some((health, entity_transform)) = health_map.get(&target_of.0) {
            let health_pct = health.current as f32 / health.max as f32;
            let max_width = 50.0;
            let new_width = max_width * health_pct;

            bar_sprite.custom_size = Some(Vec2::new(new_width, 6.0));

            let y_offset = 40.0;
            let x_offset = (max_width - new_width) / 2.0;
            bar_transform.translation.x = entity_transform.translation.x - x_offset;
            bar_transform.translation.y = entity_transform.translation.y + y_offset;
        }
    }

    for (mut bg_transform, target_of) in bg_query.iter_mut() {
        if let Some((_, entity_transform)) = health_map.get(&target_of.0) {
            let y_offset = 40.0;
            bg_transform.translation.x = entity_transform.translation.x;
            bg_transform.translation.y = entity_transform.translation.y + y_offset;
        }
    }
}

fn update_info_text(
    player_query: Query<&PlayerTank>,
    targets_query: Query<(Entity, &Health, Option<&EnemyTank>, Option<&EnemyBuilding>), With<Targetable>>,
    mut text_query: Query<&mut Text, With<InfoText>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    let target_info = if let Some(target_entity) = player.target {
        if let Ok((_, health, is_tank, is_building)) = targets_query.get(target_entity) {
            let target_type = if is_tank.is_some() {
                "Enemy Tank"
            } else if is_building.is_some() {
                "Enemy Building"
            } else {
                "Unknown"
            };
            format!("{} (HP: {}/{})", target_type, health.current, health.max)
        } else {
            "Target lost".to_string()
        }
    } else {
        "None".to_string()
    };

    let remaining_targets: Vec<String> = targets_query
        .iter()
        .map(|(_, health, is_tank, is_building)| {
            let t = if is_tank.is_some() { "Tank" } else { "Building" };
            format!("{}: {}/{}", t, health.current, health.max)
        })
        .collect();

    text.0 = format!(
        "Press SPACE to cycle targets\n\
         Current target: {}\n\
         \n\
         Remaining enemies:\n\
         {}",
        target_info,
        if remaining_targets.is_empty() {
            "All enemies destroyed!".to_string()
        } else {
            remaining_targets.join("\n")
        }
    );
}

