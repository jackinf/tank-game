//! Visual effects: explosions and weapon barrels.

use crate::components::*;
use crate::faction::Faction;
use crate::state::GameState;
use bevy::prelude::*;

pub struct FxPlugin;

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_explosions, draw_barrels).run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut explosions: Query<(Entity, &Transform, &mut Explosion)>,
) {
    let dt = time.delta_secs();
    for (entity, transform, mut explosion) in &mut explosions {
        explosion.age += dt;
        if explosion.age >= explosion.lifetime {
            commands.entity(entity).despawn();
            continue;
        }
        let t = explosion.age / explosion.lifetime;
        let radius = explosion.radius * (0.3 + t);
        let pos = transform.translation.truncate();
        let color = Color::srgba(1.0, 0.6 - t * 0.4, 0.1, 1.0 - t);
        gizmos.circle_2d(Isometry2d::from_translation(pos), radius, color);
        gizmos.circle_2d(Isometry2d::from_translation(pos), radius * 0.6, color);
    }
}

/// Draw a short barrel from each armed entity in the direction it is aiming.
fn draw_barrels(
    mut gizmos: Gizmos,
    armed: Query<(&Transform, &WeaponState, &Faction)>,
) {
    for (transform, weapon, faction) in &armed {
        let pos = transform.translation.truncate();
        let len = 14.0;
        let color = match faction {
            Faction::Enemy => Color::srgb(0.5, 0.1, 0.1),
            _ => Color::srgb(0.1, 0.1, 0.25),
        };
        gizmos.line_2d(pos, pos + weapon.aim.normalize_or_zero() * len, color);
    }
}
