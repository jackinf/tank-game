//! Target acquisition: how units and turrets decide what to shoot, and how
//! units chase a target or follow an attack-move order.

use crate::components::*;
use crate::config::TILE;
use crate::faction::Faction;
use crate::grid::{find_path, GameMap};
use bevy::prelude::*;

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
        if d <= max_range && best.is_none_or(|(_, _, bd)| d < bd) {
            best = Some((t.entity, t.pos, d));
        }
    }
    best.map(|(e, p, _)| (e, p))
}

/// Drives units: process orders, acquire targets, manage chasing.
#[allow(clippy::type_complexity)]
pub(super) fn unit_brain(
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
                        if pos.distance(dest) < TILE {
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
pub(super) fn turret_targeting(
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
