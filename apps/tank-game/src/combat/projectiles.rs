//! Projectile flight and impact: home toward the last-seen target position and
//! apply armour-scaled damage on contact.

use crate::components::*;
use crate::config::z;
use crate::defs::{damage_multiplier, Role};
use crate::state::GameEntity;
use bevy::prelude::*;
use std::collections::HashMap;

/// Move projectiles toward their targets and apply damage on impact.
#[allow(clippy::type_complexity)]
pub(super) fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut params: ParamSet<(
        Query<(Entity, &mut Transform, &mut Projectile)>,
        Query<(Entity, &Transform, &mut Health, Option<&Armor>)>,
    )>,
) {
    let dt = time.delta_secs();

    // Snapshot target positions.
    let positions: HashMap<Entity, Vec2> = params
        .p1()
        .iter()
        .map(|(e, t, _, _)| (e, t.translation.truncate()))
        .collect();

    let mut hits: Vec<(Entity, f32, Role)> = Vec::new();

    {
        let mut projectiles = params.p0();
        for (entity, mut transform, mut proj) in &mut projectiles {
            let aim = positions.get(&proj.target).copied();
            if let Some(p) = aim {
                proj.last_seen = p;
            }
            let goal = proj.last_seen;
            let pos = transform.translation.truncate();
            let to = goal - pos;
            let dist = to.length();
            let step = proj.speed * dt;

            if dist <= step.max(6.0) {
                // Impact.
                if aim.is_some() {
                    hits.push((proj.target, proj.damage, proj.role));
                }
                commands.spawn((
                    Explosion {
                        age: 0.0,
                        lifetime: 0.25,
                        radius: 10.0,
                    },
                    Transform::from_xyz(goal.x, goal.y, z::FX),
                    GameEntity,
                ));
                commands.entity(entity).despawn();
            } else {
                let delta = to / dist * step;
                transform.translation.x += delta.x;
                transform.translation.y += delta.y;
            }
        }
    }

    // Apply damage, scaled by the target's armour against this warhead.
    let mut healths = params.p1();
    for (target, damage, role) in hits {
        if let Ok((_, _, mut health, armor)) = healths.get_mut(target) {
            let mult = armor.map_or(1.0, |a| damage_multiplier(role, a.0));
            health.damage(damage * mult);
        }
    }
}
