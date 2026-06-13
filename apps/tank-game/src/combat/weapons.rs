//! Firing: spend the reload timer and spawn a projectile at an in-range target.

use crate::components::*;
use crate::config::z;
use crate::faction::Faction;
use crate::state::GameEntity;
use bevy::prelude::*;
use std::collections::HashMap;

/// Fire weapons whose target is in range and whose cooldown has elapsed.
pub(super) fn fire_weapons(
    mut commands: Commands,
    time: Res<Time>,
    positions_q: Query<(Entity, &Transform), With<Health>>,
    mut shooters: Query<(&Faction, &Transform, &mut WeaponState)>,
) {
    let dt = time.delta_secs();
    let positions: HashMap<Entity, Vec2> = positions_q
        .iter()
        .map(|(e, t)| (e, t.translation.truncate()))
        .collect();

    for (faction, transform, mut weapon) in &mut shooters {
        if weapon.cooldown > 0.0 {
            weapon.cooldown -= dt;
        }
        let Some(target) = weapon.target else {
            continue;
        };
        let Some(&target_pos) = positions.get(&target) else {
            weapon.target = None;
            continue;
        };
        let pos = transform.translation.truncate();
        if pos.distance(target_pos) > weapon.weapon.range {
            continue;
        }
        if weapon.cooldown > 0.0 {
            continue;
        }

        weapon.cooldown = weapon.weapon.reload;
        let muzzle = pos + weapon.aim * 12.0;
        let kind = weapon.weapon.projectile;
        commands.spawn((
            Projectile {
                damage: weapon.weapon.damage,
                speed: weapon.weapon.projectile_speed,
                target,
                faction: *faction,
                kind,
                role: weapon.weapon.role,
                last_seen: target_pos,
            },
            Sprite::from_color(kind.color(), Vec2::splat(kind.radius() * 2.0)),
            Transform::from_xyz(muzzle.x, muzzle.y, z::PROJECTILE),
            GameEntity,
        ));
    }
}
