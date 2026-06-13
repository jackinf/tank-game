//! Visual effects: explosions, weapon barrels and rotating tank turrets.

use crate::components::*;
use crate::defs::{UnitClass, UnitKind};
use crate::faction::Faction;
use crate::state::GameState;
use bevy::prelude::*;

pub struct FxPlugin;

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_explosions, rotate_turrets, draw_barrels, texture_buildings)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Draw textured buildings with their sprite asset instead of a flat colour,
/// switching to a damaged variant once health drops below 50%. The footprint
/// size set at spawn time is kept; we replace the image and drop the colour
/// tint so the texture shows its true colours. `asset_server.load` is cached,
/// and we only touch the sprite when the chosen image actually changes, so this
/// is cheap to run every frame. Buildings without an asset are untouched.
fn texture_buildings(
    asset_server: Res<AssetServer>,
    mut buildings: Query<(&Building, &Faction, &Health, &mut Sprite)>,
) {
    for (building, faction, health, mut sprite) in &mut buildings {
        let Some(healthy) = building.kind.texture_path() else {
            continue;
        };
        let path = match building.kind.damaged_texture_path() {
            Some(damaged) if health.fraction() < 0.5 => damaged,
            _ => healthy,
        };
        let handle = asset_server.load(path);
        if sprite.image != handle {
            sprite.image = handle;
            // Subtle faction wash so you can tell whose base it is at a glance.
            sprite.color = faction.tint();
        }
    }
}

/// Swivel each tank's turret to face where it is aiming (when it has a target)
/// or where it is heading (when it is just driving).
#[allow(clippy::type_complexity)]
fn rotate_turrets(
    time: Res<Time>,
    hulls: Query<(&Transform, &Mover, &WeaponState), (With<Unit>, Without<Turret>)>,
    mut turrets: Query<(&ChildOf, &mut Transform), With<Turret>>,
) {
    let dt = time.delta_secs();
    for (child_of, mut turret_tf) in &mut turrets {
        let Ok((hull_tf, mover, weapon)) = hulls.get(child_of.parent()) else {
            continue;
        };
        // Prefer aiming at the current target; otherwise face the next waypoint.
        let desired = if weapon.target.is_some() && weapon.aim != Vec2::ZERO {
            weapon.aim
        } else if let Some(&wp) = mover.path.front() {
            (wp - hull_tf.translation.truncate()).normalize_or_zero()
        } else {
            Vec2::ZERO
        };
        if desired == Vec2::ZERO {
            continue;
        }
        let target = desired.y.atan2(desired.x);
        let current = turret_tf.rotation.to_euler(EulerRot::ZYX).0;
        // Smoothly rotate toward the target angle.
        let mut delta = (target - current + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU)
            - std::f32::consts::PI;
        let max_step = 8.0 * dt;
        delta = delta.clamp(-max_step, max_step);
        turret_tf.rotation = Quat::from_rotation_z(current + delta);
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
/// Tanks have a real rotating turret sprite, so they're skipped here.
fn draw_barrels(
    mut gizmos: Gizmos,
    armed: Query<(&Transform, &WeaponState, &Faction, Option<&Unit>)>,
) {
    for (transform, weapon, faction, unit) in &armed {
        // Vehicles render a turret sprite instead of a gizmo barrel.
        if let Some(Unit { kind: UnitKind::Combat { class: UnitClass::Vehicle, .. } }) = unit {
            continue;
        }
        let pos = transform.translation.truncate();
        let len = 14.0;
        let color = match faction {
            Faction::Enemy => Color::srgb(0.5, 0.1, 0.1),
            _ => Color::srgb(0.1, 0.1, 0.25),
        };
        gizmos.line_2d(pos, pos + weapon.aim.normalize_or_zero() * len, color);
    }
}
