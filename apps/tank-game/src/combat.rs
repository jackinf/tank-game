//! Targeting, weapons, projectiles and damage.

use crate::components::*;
use crate::config::z;
use crate::defs::{damage_multiplier, Role};
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
                // Back to full speed once a group move is over.
                mover.speed = mover.base_speed;
                // Defensive: fire at anything that walks into range, but don't chase.
                if let Some((e, tp)) = nearest_enemy(pos, *faction, range, &targets) {
                    weapon.target = Some(e);
                    weapon.aim = (tp - pos).normalize_or_zero();
                } else {
                    weapon.target = None;
                }
            }
            Order::Move(_dest) => {
                weapon.target = None;
                // The path now ends on a tile centre (grid-snapped), so simply
                // go idle once there are no waypoints left.
                if !mover.is_moving() {
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
                        chase(&map, &mut mover, tp);
                    }
                } else {
                    weapon.target = None;
                    if !mover.is_moving() {
                        // Arrived at the grid-snapped destination tile (within a
                        // tile's reach), or it's unreachable — either way, stop.
                        if pos.distance(dest) < crate::config::TILE {
                            *command = Order::Idle;
                        } else {
                            path_to(&map, &mut mover, pos, dest);
                        }
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
                        chase(&map, &mut mover, t.pos);
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
                role: weapon.weapon.role,
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

/// Push a single chase waypoint, but never one inside an impassable tile: if
/// the target sits on water/mountain/a footprint, aim for the nearest passable
/// tile instead. Combined with the movement guard, units stop at the edge.
fn chase(map: &GameMap, mover: &mut Mover, target: Vec2) {
    mover.path.clear();
    let tile = map.world_to_tile(target);
    let goal = if map.is_passable(tile.0, tile.1) {
        target
    } else if let Some(p) = map.nearest_passable(tile) {
        map.tile_center(p.0, p.1)
    } else {
        return;
    };
    mover.path.push_back(goal);
}

fn path_to(map: &GameMap, mover: &mut Mover, from: Vec2, to: Vec2) {
    let start = map.world_to_tile(from);
    let goal = map.world_to_tile(to);
    if let Some(path) = find_path(map, start, goal) {
        mover.path = path.into_iter().map(|(c, r)| map.tile_center(c, r)).collect();
        // Only snap the final waypoint onto the exact destination when that
        // tile is actually passable; otherwise keep the nearest-passable tile
        // centre that find_path produced so we never aim into a solid tile.
        if map.is_passable(goal.0, goal.1) {
            if let Some(last) = mover.path.back_mut() {
                *last = to;
            }
        }
    }
}
