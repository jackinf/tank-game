//! Health bars, death, and win/lose detection.

use crate::components::*;
use crate::faction::Faction;
use crate::grid::GameMap;
use crate::state::{GameEntity, GameResult, GameState};
use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (death_system, check_game_over, draw_health_bars)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn death_system(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    dead: Query<(Entity, &Health, &Transform, Option<&Building>)>,
) {
    for (entity, health, transform, building) in &dead {
        if !health.is_dead() {
            continue;
        }
        let pos = transform.translation.truncate();

        // Free a building's footprint.
        if let Some(building) = building {
            let (w, h) = building.kind.footprint();
            for dr in 0..h {
                for dc in 0..w {
                    map.set_blocked(building.origin.0 + dc, building.origin.1 + dr, false);
                    map.set_built(building.origin.0 + dc, building.origin.1 + dr, false);
                }
            }
        }

        // Death explosion.
        let radius = if building.is_some() { 40.0 } else { 16.0 };
        commands.spawn((
            Explosion { age: 0.0, lifetime: 0.5, radius },
            Transform::from_xyz(pos.x, pos.y, crate::config::z::FX),
            GameEntity,
        ));

        commands.entity(entity).despawn();
    }
}

fn check_game_over(
    mut commands: Commands,
    mut next: ResMut<NextState<GameState>>,
    things: Query<&Faction, Or<(With<Building>, With<Unit>)>>,
) {
    let mut player = 0;
    let mut enemy = 0;
    for faction in &things {
        match faction {
            Faction::Player => player += 1,
            Faction::Enemy => enemy += 1,
            Faction::Neutral => {}
        }
    }

    // Don't end the game before anything has spawned.
    if player + enemy == 0 {
        return;
    }

    if player == 0 {
        commands.insert_resource(GameResult::Defeat);
        next.set(GameState::GameOver);
    } else if enemy == 0 {
        commands.insert_resource(GameResult::Victory);
        next.set(GameState::GameOver);
    }
}

fn draw_health_bars(
    mut gizmos: Gizmos,
    q: Query<(&Health, &Transform, &Selectable, Option<&Selected>)>,
) {
    for (health, transform, selectable, selected) in &q {
        let frac = health.fraction();
        if frac >= 1.0 && selected.is_none() {
            continue;
        }
        let pos = transform.translation.truncate();
        let width = (selectable.radius * 1.8).clamp(24.0, 90.0);
        let y = pos.y + selectable.radius + 8.0;
        let left = pos.x - width / 2.0;

        let fill_color = if frac > 0.5 {
            Color::srgb(0.2, 0.9, 0.2)
        } else if frac > 0.25 {
            Color::srgb(0.9, 0.8, 0.2)
        } else {
            Color::srgb(0.9, 0.2, 0.2)
        };

        // Background + fill, faked as a few stacked lines for thickness.
        for i in 0..3 {
            let yy = y + i as f32;
            gizmos.line_2d(
                Vec2::new(left, yy),
                Vec2::new(left + width, yy),
                Color::srgb(0.1, 0.1, 0.1),
            );
            gizmos.line_2d(
                Vec2::new(left, yy),
                Vec2::new(left + width * frac, yy),
                fill_color,
            );
        }
    }
}
