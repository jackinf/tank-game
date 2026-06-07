//! Targeting, weapons, projectiles and damage.

use crate::components::*;
use crate::config::z;
use crate::faction::Faction;
use crate::grid::{find_path, GameMap};
use crate::state::{GameEntity, GameState};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                unit_brain,
                turret_targeting,
                fire_weapons,
                update_projectiles,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// A lightweight snapshot of a potential target.
#[derive(Clone, Copy)]
struct TargetInfo {
    entity: Entity,
    faction: Faction,
    pos: Vec2,
}

fn snapshot_targets(q: &Query<(Entity, &Faction, &Transform), With<Health>>) -> Vec<TargetInfo> {
    q.iter()
        .map(|(entity, faction, tf)| TargetInfo {
            entity,
            faction: *faction,
            pos: tf.translation.truncate(),
        })
        .collect()
}

fn nearest_enemy(
    pos: Vec2,
    mine: Faction,
    max_range: f32,
    targets: &[TargetInfo],
) -> Option<(Entity, Vec2)> {
    let mut best: Option<(Entity, Vec2, f32)> = None;
    for t in targets {
        if !mine.is_hostile_to(t.faction) {
            continue;
        }
        let d = pos.distance(t.pos);
        if d <= max_range && best.map_or(true, |(_, _, bd)| d < bd) {
            best = Some((t.entity, t.pos, d));
        }
    }
    best.map(|(e, p, _)| (e, p))
}

/// Drives units: process orders, acquire targets, manage chasing.
#[allow(clippy::type_complexity)]
fn unit_brain(
    map: Res<GameMap>,
    targets_q: Query<(Entity, &Faction, &Transform), With<Health>>,
    mut units: Query<
        (
            &Faction,
            &Transform,
            &mut Mover,
            &mut Order,
            &mut WeaponState,
        ),
        With<Unit>,
    >,
) {
    let targets = snapshot_targets(&targets_q);

    for (faction, transform, mut mover, mut command, mut weapon) in &mut units {
        let pos = transform.translation.truncate();
        let range = weapon.weapon.range;
        let sight = weapon.weapon.sight;

        match *command {
            Order::Idle => {
                // Defensive: fire at anything that walks into range, but don't chase.
                if let Some((e, tp)) = nearest_enemy(pos, *faction, range, &targets) {
                    weapon.target = Some(e);
                    weapon.aim = (tp - pos).normalize_or_zero();
                } else {
                    weapon.target = None;
                }
            }
            Order::Move(dest) => {
                weapon.target = None;
                if !mover.is_moving() && pos.distance(dest) < 6.0 {
                    *command = Order::Idle;
                }
            }
            Order::AttackMove(dest) => {
                if let Some((e, tp)) = nearest_enemy(pos, *faction, sight, &targets) {
                    weapon.target = Some(e);
                    weapon.aim = (tp - pos).normalize_or_zero();
                    if pos.distance(tp) <= range {
                        mover.stop();
                    } else {
                        chase(&mut mover, tp);
                    }
                } else {
                    weapon.target = None;
                    if pos.distance(dest) < 6.0 {
                        *command = Order::Idle;
                    } else if !mover.is_moving() {
                        path_to(&map, &mut mover, pos, dest);
                    }
                }
            }
            Order::Attack(target_entity) => {
                if let Some(t) = targets.iter().find(|t| t.entity == target_entity) {
                    weapon.target = Some(target_entity);
                    weapon.aim = (t.pos - pos).normalize_or_zero();
                    if pos.distance(t.pos) <= range {
                        mover.stop();
                    } else {
                        chase(&mut mover, t.pos);
                    }
                } else {
                    weapon.target = None;
                    *command = Order::Idle;
                }
            }
        }
    }
}

/// Stationary buildings (turrets) acquire any enemy within range.
#[allow(clippy::type_complexity)]
fn turret_targeting(
    targets_q: Query<(Entity, &Faction, &Transform), With<Health>>,
    mut turrets: Query<(&Faction, &Transform, &mut WeaponState), (With<Building>, Without<Unit>)>,
) {
    let targets = snapshot_targets(&targets_q);
    for (faction, transform, mut weapon) in &mut turrets {
        let pos = transform.translation.truncate();
        let sight = weapon.weapon.sight;
        if let Some((e, tp)) = nearest_enemy(pos, *faction, sight, &targets) {
            weapon.target = Some(e);
            weapon.aim = (tp - pos).normalize_or_zero();
        } else {
            weapon.target = None;
        }
    }
}

/// Fire weapons whose target is in range and whose cooldown has elapsed.
fn fire_weapons(
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
                last_seen: target_pos,
            },
            Sprite::from_color(kind.color(), Vec2::splat(kind.radius() * 2.0)),
            Transform::from_xyz(muzzle.x, muzzle.y, z::PROJECTILE),
            GameEntity,
        ));
    }
}

/// Move projectiles toward their targets and apply damage on impact.
fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut params: ParamSet<(
        Query<(Entity, &mut Transform, &mut Projectile)>,
        Query<(Entity, &Transform, &mut Health)>,
    )>,
) {
    let dt = time.delta_secs();

    // Snapshot target positions.
    let positions: HashMap<Entity, Vec2> = params
        .p1()
        .iter()
        .map(|(e, t, _)| (e, t.translation.truncate()))
        .collect();

    let mut hits: Vec<(Entity, f32, Vec2)> = Vec::new();

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
                    hits.push((proj.target, proj.damage, goal));
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

    // Apply damage.
    let mut healths = params.p1();
    for (target, damage, _) in hits {
        if let Ok((_, _, mut health)) = healths.get_mut(target) {
            health.damage(damage);
        }
    }
}

fn chase(mover: &mut Mover, target: Vec2) {
    mover.path.clear();
    mover.path.push_back(target);
}

fn path_to(map: &GameMap, mover: &mut Mover, from: Vec2, to: Vec2) {
    let start = map.world_to_tile(from);
    let goal = map.world_to_tile(to);
    if let Some(path) = find_path(map, start, goal) {
        mover.path = path.into_iter().map(|(c, r)| map.tile_center(c, r)).collect();
        if let Some(last) = mover.path.back_mut() {
            *last = to;
        }
    }
}
