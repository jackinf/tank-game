//! Per-building build queues and the bookkeeping that drives them: ticking the
//! front item to completion, and tracking which buildings each faction owns.

use crate::components::*;
use crate::config::TILE;
use crate::defs::*;
use crate::economy::Economy;
use crate::faction::Faction;
use crate::spawn::spawn_unit;
use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};

/// An independent build queue attached to a single production building.
#[derive(Component, Default)]
pub struct ProductionQueue {
    pub items: VecDeque<Producible>,
    /// Seconds elapsed building the front item.
    pub progress: f32,
    /// Finished structures awaiting placement (player) / spawning (AI). Only
    /// ever populated on a Construction Yard's queue. Multiple buildings can
    /// accumulate here, so the player can queue several and place them later.
    pub ready: VecDeque<BuildingKind>,
}

impl ProductionQueue {
    pub fn front(&self) -> Option<Producible> {
        self.items.front().copied()
    }

    pub fn fraction(&self) -> f32 {
        match self.front() {
            Some(p) => (self.progress / p.build_time()).clamp(0.0, 1.0),
            None => 0.0,
        }
    }

    /// Remove the first queued copy of `kind` from the ready list, if present.
    pub fn take_ready(&mut self, kind: BuildingKind) -> bool {
        if let Some(i) = self.ready.iter().position(|&k| k == kind) {
            self.ready.remove(i);
            true
        } else {
            false
        }
    }
}

/// What buildings each faction currently owns (recomputed each frame).
#[derive(Resource, Default)]
pub struct OwnedBuildings {
    pub player: HashSet<BuildingKind>,
    pub enemy: HashSet<BuildingKind>,
}

impl OwnedBuildings {
    pub fn get(&self, f: Faction) -> &HashSet<BuildingKind> {
        match f {
            Faction::Enemy => &self.enemy,
            _ => &self.player,
        }
    }
}

pub(super) fn update_owned_buildings(
    mut owned: ResMut<OwnedBuildings>,
    buildings: Query<(&Building, &Faction)>,
) {
    owned.player.clear();
    owned.enemy.clear();
    for (building, faction) in &buildings {
        match faction {
            Faction::Enemy => owned.enemy.insert(building.kind),
            _ => owned.player.insert(building.kind),
        };
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn tick_production(
    time: Res<Time>,
    mut commands: Commands,
    economy: Res<Economy>,
    mut queues: Query<(
        &Building,
        &Faction,
        &mut ProductionQueue,
        &Transform,
        Option<&RallyPoint>,
    )>,
) {
    let dt = time.delta_secs();

    for (_building, faction, mut queue, tf, rally) in &mut queues {
        let Some(front) = queue.front() else { continue };
        let power_factor = economy.get(*faction).power_factor();
        queue.progress += dt * power_factor;
        if queue.progress >= front.build_time() {
            queue.progress = 0.0;
            queue.items.pop_front();
            match front {
                // Finished structures accumulate; the player places them when
                // ready, the AI drops them straight onto the map.
                Producible::Building(b) => queue.ready.push_back(b),
                Producible::Unit(u) => {
                    let base = tf.translation.truncate();
                    let rally_pos = rally.map(|r| r.0).unwrap_or(base);
                    spawn_completed_unit(&mut commands, *faction, u, base, rally_pos);
                }
            }
        }
    }
}

fn spawn_completed_unit(
    commands: &mut Commands,
    faction: Faction,
    unit: UnitKind,
    base: Vec2,
    rally: Vec2,
) {
    // Spawn just below the building, then send to the rally point.
    let spawn_at = Vec2::new(base.x, base.y - TILE * 1.5);
    let entity = spawn_unit(commands, unit, faction, spawn_at);
    if unit != UnitKind::Harvester {
        commands
            .entity(entity)
            .insert(Order::Move(rally))
            .insert({
                let mut m = Mover::new(unit.speed());
                m.path.push_back(rally);
                m
            });
    }
}
