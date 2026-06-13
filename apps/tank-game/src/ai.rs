//! A simple enemy AI: builds an economy, defends, and periodically attacks.

use crate::components::*;
use crate::defs::*;
use crate::economy::Economy;
use crate::faction::Faction;
use crate::grid::{GameMap, Tile};
use crate::production::*;
use crate::spawn::{find_spot, spawn_building};
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AiState {
    pub base_tile: Tile,
    pub base_pos: Vec2,
    pub decision_timer: f32,
    pub attack_timer: f32,
}

impl Default for AiState {
    fn default() -> Self {
        Self {
            base_tile: (0, 0),
            base_pos: Vec2::ZERO,
            decision_timer: 0.0,
            attack_timer: 30.0,
        }
    }
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AiState>().add_systems(
            Update,
            (ai_build, ai_attack).run_if(in_state(GameState::Playing)),
        );
    }
}

const UNIT_CAP: usize = 14;
const ATTACK_SQUAD: usize = 5;

// The enemy fields a representative slice of the unit roster.
const AI_TANK: UnitKind = UnitKind::BASIC_TANK;
const AI_SOLDIER: UnitKind = UnitKind::BASIC_SOLDIER;

#[allow(clippy::too_many_arguments)]
fn ai_build(
    time: Res<Time>,
    mut commands: Commands,
    mut ai: ResMut<AiState>,
    mut map: ResMut<GameMap>,
    mut economy: ResMut<Economy>,
    owned: Res<OwnedBuildings>,
    mut queues: Query<(&Building, &Faction, &mut ProductionQueue)>,
    buildings: Query<(&Building, &Faction)>,
    units: Query<(&Unit, &Faction)>,
) {
    ai.decision_timer -= time.delta_secs();

    // Place any finished structure first (every frame so we never stall).
    let ready = enemy_yard_ready(&queues);
    if let Some(kind) = ready {
        if let Some(origin) = find_spot(&map, ai.base_tile, kind.footprint()) {
            spawn_building(&mut commands, &mut map, kind, Faction::Enemy, origin);
            for (b, f, mut q) in &mut queues {
                if *f == Faction::Enemy && b.kind == BuildingKind::ConstructionYard {
                    q.take_ready(kind);
                }
            }
        }
    }

    if ai.decision_timer > 0.0 {
        return;
    }
    ai.decision_timer = 1.5;

    let set = owned.enemy.clone();
    let eco = *economy.get(Faction::Enemy);

    // --- Structures ---
    if !queue_busy(&queues, BuildingKind::ConstructionYard) {
        let next = if !set.contains(&BuildingKind::PowerPlant) || !eco.has_power() {
            Some(BuildingKind::PowerPlant)
        } else if !set.contains(&BuildingKind::Refinery) {
            Some(BuildingKind::Refinery)
        } else if !set.contains(&BuildingKind::Barracks) {
            Some(BuildingKind::Barracks)
        } else if !set.contains(&BuildingKind::WarFactory) {
            Some(BuildingKind::WarFactory)
        } else if count_kind(&buildings, Faction::Enemy, BuildingKind::AntiInfantryTurret) < 2 {
            Some(BuildingKind::AntiInfantryTurret)
        } else if count_kind(&buildings, Faction::Enemy, BuildingKind::AntiTankTurret) < 2 {
            Some(BuildingKind::AntiTankTurret)
        } else {
            None
        };
        if let Some(b) = next {
            ai_enqueue(&mut queues, &mut economy, &owned, Producible::Building(b));
        }
    }

    // --- Harvesters (war factory) ---
    let harvesters = count_harvesters(&units, Faction::Enemy);
    let vehicles_busy = queue_busy(&queues, BuildingKind::WarFactory);
    if set.contains(&BuildingKind::WarFactory) && harvesters < 2 && !vehicles_busy {
        ai_enqueue(
            &mut queues,
            &mut economy,
            &owned,
            Producible::Unit(UnitKind::Harvester),
        );
    } else {
        // --- Combat units ---
        let combat = count_combat(&units, Faction::Enemy);
        if combat < UNIT_CAP {
            if set.contains(&BuildingKind::WarFactory)
                && !vehicles_busy
                && economy.get(Faction::Enemy).can_afford(AI_TANK.cost())
            {
                ai_enqueue(&mut queues, &mut economy, &owned, Producible::Unit(AI_TANK));
            }
            if set.contains(&BuildingKind::Barracks)
                && !queue_busy(&queues, BuildingKind::Barracks)
            {
                ai_enqueue(&mut queues, &mut economy, &owned, Producible::Unit(AI_SOLDIER));
            }
        }
    }
}

/// The structure waiting to be placed at the enemy Construction Yard, if any.
fn enemy_yard_ready(queues: &Query<(&Building, &Faction, &mut ProductionQueue)>) -> Option<BuildingKind> {
    queues
        .iter()
        .find(|(b, f, _)| **f == Faction::Enemy && b.kind == BuildingKind::ConstructionYard)
        .and_then(|(_, _, q)| q.ready.front().copied())
}

/// Is the enemy's producer of `producer_kind` currently building something?
fn queue_busy(queues: &Query<(&Building, &Faction, &mut ProductionQueue)>, producer_kind: BuildingKind) -> bool {
    queues
        .iter()
        .any(|(b, f, q)| *f == Faction::Enemy && b.kind == producer_kind && !q.items.is_empty())
}

/// Enqueue `item` into the enemy building that produces it.
fn ai_enqueue(
    queues: &mut Query<(&Building, &Faction, &mut ProductionQueue)>,
    economy: &mut Economy,
    owned: &OwnedBuildings,
    item: Producible,
) -> bool {
    let producer = match item {
        Producible::Building(_) => BuildingKind::ConstructionYard,
        Producible::Unit(u) => u.produced_by(),
    };
    for (b, f, mut q) in queues.iter_mut() {
        if *f == Faction::Enemy && b.kind == producer {
            return try_enqueue(&mut q, economy, owned, Faction::Enemy, item);
        }
    }
    false
}

#[allow(clippy::type_complexity)]
fn ai_attack(
    time: Res<Time>,
    mut ai: ResMut<AiState>,
    player_things: Query<(&Transform, &Faction), Or<(With<Building>, With<Unit>)>>,
    mut squad: Query<
        (&Transform, &mut Mover, &mut Order, &Faction),
        (With<Unit>, With<WeaponState>),
    >,
) {
    ai.attack_timer -= time.delta_secs();
    if ai.attack_timer > 0.0 {
        return;
    }
    ai.attack_timer = 25.0;

    // Find a target: nearest player thing to the enemy base.
    let mut target: Option<Vec2> = None;
    let mut best = f32::MAX;
    for (tf, faction) in &player_things {
        if *faction == Faction::Player {
            let p = tf.translation.truncate();
            let d = p.distance(ai.base_pos);
            if d < best {
                best = d;
                target = Some(p);
            }
        }
    }
    let Some(target) = target else { return };

    // Count idle combat units.
    let mut ready = 0usize;
    for (_, _, command, faction) in &squad {
        if *faction == Faction::Enemy && matches!(*command, Order::Idle) {
            ready += 1;
        }
    }
    if ready < ATTACK_SQUAD {
        return;
    }

    for (_, mut mover, mut command, faction) in &mut squad {
        if *faction == Faction::Enemy && matches!(*command, Order::Idle) {
            mover.path.clear();
            mover.path.push_back(target);
            *command = Order::AttackMove(target);
        }
    }
}

fn count_kind(q: &Query<(&Building, &Faction)>, faction: Faction, kind: BuildingKind) -> usize {
    q.iter()
        .filter(|(b, f)| **f == faction && b.kind == kind)
        .count()
}

fn count_harvesters(q: &Query<(&Unit, &Faction)>, faction: Faction) -> usize {
    q.iter()
        .filter(|(u, f)| **f == faction && matches!(u.kind, UnitKind::Harvester))
        .count()
}

fn count_combat(q: &Query<(&Unit, &Faction)>, faction: Faction) -> usize {
    q.iter()
        .filter(|(u, f)| **f == faction && matches!(u.kind, UnitKind::Combat { .. }))
        .count()
}
