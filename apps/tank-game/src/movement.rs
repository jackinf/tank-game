//! Path-following movement for units, plus light collision so two idle units
//! never sit on the same tile.

use crate::components::*;
use crate::grid::{GameMap, Tile};
use crate::harvester::Harvester;
use crate::state::GameState;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (follow_path, resolve_overlap).run_if(in_state(GameState::Playing)),
        );
    }
}

/// How often (seconds) to run the un-stacking pass. Cheap, but no need to do it
/// every frame — a few times a second is plenty for settling.
const OVERLAP_INTERVAL: f32 = 0.25;

/// Spread out units that have come to rest on the same tile. Only units that
/// are *at rest* (idle and not moving) are considered, so a unit that stopped
/// mid-march to fight is left exactly where it is. Moving units separate
/// naturally via formation offsets. Each over-stacked unit is nudged to the
/// nearest free tile, which is cheap because we only touch resting units a few
/// times a second.
fn resolve_overlap(
    time: Res<Time>,
    mut timer: Local<f32>,
    map: Res<GameMap>,
    // Harvesters run their own seek/dock AI, so leave them out of un-stacking.
    mut units: Query<(Entity, &Transform, &mut Mover, &Order), (With<Unit>, Without<Harvester>)>,
) {
    *timer -= time.delta_secs();
    if *timer > 0.0 {
        return;
    }
    *timer = OVERLAP_INTERVAL;

    // Claim the tiles of resting units; collect the ones that need a new home.
    let mut claimed: HashMap<Tile, Entity> = HashMap::new();
    let mut resting: Vec<(Entity, Tile)> = Vec::new();
    for (entity, tf, mover, order) in &units {
        if *order != Order::Idle || mover.is_moving() {
            continue;
        }
        resting.push((entity, map.world_to_tile(tf.translation.truncate())));
    }
    // Deterministic order so the same unit keeps a contested tile each pass.
    resting.sort_by_key(|(e, _)| e.index());

    let mut relocate: Vec<(Entity, Vec2)> = Vec::new();
    for (entity, tile) in resting {
        if !claimed.contains_key(&tile) {
            claimed.insert(tile, entity);
            continue;
        }
        // Tile taken: search outward rings for the nearest free, passable tile.
        if let Some(free) = nearest_free_tile(&map, tile, &claimed) {
            claimed.insert(free, entity);
            relocate.push((entity, map.tile_center(free.0, free.1)));
        }
    }

    for (entity, dest) in relocate {
        if let Ok((_, _, mut mover, _)) = units.get_mut(entity) {
            mover.path.clear();
            mover.path.push_back(dest);
        }
    }
}

fn nearest_free_tile(map: &GameMap, from: Tile, claimed: &HashMap<Tile, Entity>) -> Option<Tile> {
    for radius in 1i32..=4 {
        for dr in -radius..=radius {
            for dc in -radius..=radius {
                if dc.abs() != radius && dr.abs() != radius {
                    continue; // outer ring only
                }
                let t = (from.0 + dc, from.1 + dr);
                if map.is_passable(t.0, t.1) && !claimed.contains_key(&t) {
                    return Some(t);
                }
            }
        }
    }
    None
}

fn follow_path(
    time: Res<Time>,
    map: Res<GameMap>,
    mut movers: Query<(&mut Transform, &mut Mover)>,
) {
    let dt = time.delta_secs();
    let (min, max) = map.world_bounds();

    for (mut transform, mut mover) in &mut movers {
        let speed = mover.speed;
        let Some(&target) = mover.path.front() else {
            // Standing still: recover full speed (a group move may have slowed
            // this unit to match the pack).
            mover.speed = mover.base_speed;
            continue;
        };
        let pos = transform.translation.truncate();
        let to = target - pos;
        let dist = to.length();
        let step = speed * dt;
        let arrived = dist <= step.max(2.0);
        let new_pos = if arrived { target } else { pos + to / dist * step };

        // Safety net: never let a unit walk into an impassable tile (water,
        // mountain, building footprint), no matter how its path was set. If the
        // next step would enter a new blocked tile, drop the waypoint and stay
        // put so the unit stops at the edge instead of getting stuck inside.
        let new_tile = map.world_to_tile(new_pos);
        if new_tile != map.world_to_tile(pos) && map.is_blocked(new_tile.0, new_tile.1) {
            mover.path.pop_front();
            continue;
        }

        transform.translation.x = new_pos.x.clamp(min.x, max.x);
        transform.translation.y = new_pos.y.clamp(min.y, max.y);
        if arrived {
            mover.path.pop_front();
        }
    }
}
