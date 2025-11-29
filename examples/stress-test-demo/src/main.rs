//! Stress Test Demo
//!
//! Tests game performance under load:
//! - Spawns a new tank every second
//! - All tanks shoot at a central target (building)
//! - Monitors FPS and entity count
//! - Press SPACE to pause/resume spawning
//! - Press R to reset (despawn all tanks)

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin};
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

// Constants
const SPAWN_INTERVAL: f32 = 1.0;
const TANK_SIZE: f32 = 20.0;
const BULLET_SPEED: f32 = 300.0;
const BULLET_SIZE: f32 = 6.0;
const SHOOT_COOLDOWN: f32 = 2.0;
const TARGET_SIZE: f32 = 80.0;
const ARENA_WIDTH: f32 = 700.0;
const ARENA_HEIGHT: f32 = 500.0;
const SPAWN_MARGIN: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Stress Test - SPACE: pause spawning, R: reset".into(),
                resolution: bevy::window::WindowResolution::new(900, 700),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin::default())
        .init_resource::<SpawnState>()
        .add_systems(Startup, (setup_camera, setup_arena, setup_ui).chain())
        .add_systems(
            Update,
            (
                handle_input,
                spawn_tanks,
                update_tank_shooting,
                move_bullets,
                check_bullet_hits,
                cleanup_dead_bullets,
                update_ui,
            ),
        )
        .run();
}

// Components
#[derive(Component)]
struct Tank {
    shoot_timer: Timer,
}

#[derive(Component)]
struct Bullet {
    direction: Vec2,
}

#[derive(Component)]
struct Target {
    health: u32,
    max_health: u32,
}

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct InfoText;

#[derive(Component)]
struct Arena;

// Resources
#[derive(Resource)]
struct SpawnState {
    timer: Timer,
    paused: bool,
    total_spawned: u32,
}

impl Default for SpawnState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating),
            paused: false,
            total_spawned: 0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 1000.0)),
    ));
}

fn setup_arena(mut commands: Commands) {
    // Arena background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.15, 0.15, 0.2),
            custom_size: Some(Vec2::new(ARENA_WIDTH, ARENA_HEIGHT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0)),
        Arena,
    ));

    // Central target (building)
    let target_pos = Vec3::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 5.0);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.2, 0.2),
            custom_size: Some(Vec2::splat(TARGET_SIZE)),
            ..default()
        },
        Transform::from_translation(target_pos),
        Target {
            health: 10000,
            max_health: 10000,
        },
    ));

    // Health bar background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.1, 0.1),
            custom_size: Some(Vec2::new(TARGET_SIZE + 20.0, 8.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            ARENA_WIDTH / 2.0,
            ARENA_HEIGHT / 2.0 + TARGET_SIZE / 2.0 + 15.0,
            6.0,
        )),
    ));

    // Health bar foreground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.7, 0.1),
            custom_size: Some(Vec2::new(TARGET_SIZE + 20.0, 8.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            ARENA_WIDTH / 2.0,
            ARENA_HEIGHT / 2.0 + TARGET_SIZE / 2.0 + 15.0,
            7.0,
        )),
        HealthBar,
    ));
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Loading..."),
        TextFont {
            font_size: 16.0,
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

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut spawn_state: ResMut<SpawnState>,
    mut commands: Commands,
    tanks_query: Query<Entity, With<Tank>>,
    bullets_query: Query<Entity, With<Bullet>>,
    mut target_query: Query<&mut Target>,
) {
    // Toggle spawning
    if keyboard.just_pressed(KeyCode::Space) {
        spawn_state.paused = !spawn_state.paused;
    }

    // Reset
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Despawn all tanks
        for entity in tanks_query.iter() {
            commands.entity(entity).despawn();
        }
        // Despawn all bullets
        for entity in bullets_query.iter() {
            commands.entity(entity).despawn();
        }
        // Reset target health
        if let Ok(mut target) = target_query.single_mut() {
            target.health = target.max_health;
        }
        // Reset spawn state
        spawn_state.total_spawned = 0;
        spawn_state.timer.reset();
    }
}

fn spawn_tanks(mut commands: Commands, time: Res<Time>, mut spawn_state: ResMut<SpawnState>) {
    if spawn_state.paused {
        return;
    }

    spawn_state.timer.tick(time.delta());

    if spawn_state.timer.just_finished() {
        let mut rng = rand::thread_rng();

        // Random position along the edge of the arena
        let side = rng.gen_range(0..4);
        let (x, y) = match side {
            0 => (rng.gen_range(SPAWN_MARGIN..ARENA_WIDTH - SPAWN_MARGIN), SPAWN_MARGIN), // Bottom
            1 => (rng.gen_range(SPAWN_MARGIN..ARENA_WIDTH - SPAWN_MARGIN), ARENA_HEIGHT - SPAWN_MARGIN), // Top
            2 => (SPAWN_MARGIN, rng.gen_range(SPAWN_MARGIN..ARENA_HEIGHT - SPAWN_MARGIN)), // Left
            _ => (ARENA_WIDTH - SPAWN_MARGIN, rng.gen_range(SPAWN_MARGIN..ARENA_HEIGHT - SPAWN_MARGIN)), // Right
        };

        // Random color (blue-ish variants)
        let r = rng.gen_range(0.1..0.4);
        let g = rng.gen_range(0.2..0.5);
        let b = rng.gen_range(0.5..0.9);

        commands.spawn((
            Sprite {
                color: Color::srgb(r, g, b),
                custom_size: Some(Vec2::splat(TANK_SIZE)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 10.0)),
            Tank {
                shoot_timer: Timer::new(
                    Duration::from_secs_f32(SHOOT_COOLDOWN + rng.gen_range(-0.5..0.5)),
                    TimerMode::Repeating,
                ),
            },
        ));

        spawn_state.total_spawned += 1;
    }
}

fn update_tank_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut tanks_query: Query<(&mut Tank, &Transform)>,
    target_query: Query<&Transform, With<Target>>,
) {
    let Ok(target_transform) = target_query.single() else {
        return;
    };
    let target_pos = target_transform.translation.truncate();

    for (mut tank, transform) in tanks_query.iter_mut() {
        tank.shoot_timer.tick(time.delta());

        if tank.shoot_timer.just_finished() {
            let tank_pos = transform.translation.truncate();
            let direction = (target_pos - tank_pos).normalize();

            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 0.8, 0.2),
                    custom_size: Some(Vec2::splat(BULLET_SIZE)),
                    ..default()
                },
                Transform::from_translation(tank_pos.extend(15.0)),
                Bullet { direction },
            ));
        }
    }
}

fn move_bullets(time: Res<Time>, mut bullets_query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in bullets_query.iter_mut() {
        transform.translation += (bullet.direction * BULLET_SPEED * time.delta_secs()).extend(0.0);
    }
}

fn check_bullet_hits(
    mut commands: Commands,
    bullets_query: Query<(Entity, &Transform), With<Bullet>>,
    mut target_query: Query<(&mut Target, &Transform)>,
) {
    let Ok((mut target, target_transform)) = target_query.single_mut() else {
        return;
    };
    let target_pos = target_transform.translation.truncate();

    for (bullet_entity, bullet_transform) in bullets_query.iter() {
        let bullet_pos = bullet_transform.translation.truncate();

        if bullet_pos.distance(target_pos) < TARGET_SIZE / 2.0 + BULLET_SIZE / 2.0 {
            commands.entity(bullet_entity).despawn();
            target.health = target.health.saturating_sub(10);
        }
    }
}

fn cleanup_dead_bullets(
    mut commands: Commands,
    bullets_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in bullets_query.iter() {
        let pos = transform.translation;
        if pos.x < -50.0 || pos.x > ARENA_WIDTH + 50.0 || pos.y < -50.0 || pos.y > ARENA_HEIGHT + 50.0
        {
            commands.entity(entity).despawn();
        }
    }
}

fn update_ui(
    spawn_state: Res<SpawnState>,
    diagnostics: Res<DiagnosticsStore>,
    tanks_query: Query<(), With<Tank>>,
    bullets_query: Query<(), With<Bullet>>,
    target_query: Query<&Target>,
    mut text_query: Query<&mut Text, With<InfoText>>,
    mut health_bar_query: Query<&mut Sprite, With<HealthBar>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    let entity_count = diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|d| d.value())
        .unwrap_or(0.0) as u32;

    let tank_count = tanks_query.iter().count();
    let bullet_count = bullets_query.iter().count();

    let target_health = target_query
        .single()
        .map(|t| (t.health, t.max_health))
        .unwrap_or((0, 1));

    let status = if spawn_state.paused {
        "PAUSED"
    } else {
        "RUNNING"
    };

    text.0 = format!(
        "Stress Test Demo\n\
         SPACE: pause/resume | R: reset\n\
         \n\
         Status: {}\n\
         FPS: {:.1}\n\
         Total Entities: {}\n\
         \n\
         Tanks: {}\n\
         Bullets: {}\n\
         Total Spawned: {}\n\
         \n\
         Target HP: {}/{}",
        status,
        fps,
        entity_count,
        tank_count,
        bullet_count,
        spawn_state.total_spawned,
        target_health.0,
        target_health.1
    );

    // Update health bar
    if let Ok(mut health_bar) = health_bar_query.single_mut() {
        let health_pct = target_health.0 as f32 / target_health.1 as f32;
        let max_width = TARGET_SIZE + 20.0;
        health_bar.custom_size = Some(Vec2::new(max_width * health_pct, 8.0));
    }
}

