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

#[allow(clippy::too_many_arguments)]
fn ai_build(
    time: Res<Time>,
    mut commands: Commands,
    mut ai: ResMut<AiState>,
    mut map: ResMut<GameMap>,
    mut production: ResMut<Production>,
    mut economy: ResMut<Economy>,
    owned: Res<OwnedBuildings>,
    buildings: Query<(&Building, &Faction)>,
    units: Query<(&Unit, &Faction)>,
) {
    ai.decision_timer -= time.delta_secs();

    // Place any finished structure first (every frame so we never stall).
    if let Some(kind) = production.enemy.queues[STRUCTURES].ready_structure {
        if let Some(origin) = find_spot(&map, ai.base_tile, kind.footprint()) {
            spawn_building(&mut commands, &mut map, kind, Faction::Enemy, origin);
            production.enemy.queues[STRUCTURES].ready_structure = None;
        }
    }

    if ai.decision_timer > 0.0 {
        return;
    }
    ai.decision_timer = 1.5;

    let set = owned.enemy.clone();
    let eco = *economy.get(Faction::Enemy);

    // --- Structures lane ---
    if production.enemy.queues[STRUCTURES].items.is_empty() {
        let next = if !set.contains(&BuildingKind::PowerPlant) {
            Some(BuildingKind::PowerPlant)
        } else if !eco.has_power() {
            Some(BuildingKind::PowerPlant)
        } else if !set.contains(&BuildingKind::Refinery) {
            Some(BuildingKind::Refinery)
        } else if !set.contains(&BuildingKind::Barracks) {
            Some(BuildingKind::Barracks)
        } else if !set.contains(&BuildingKind::WarFactory) {
            Some(BuildingKind::WarFactory)
        } else if count_kind(&buildings, Faction::Enemy, BuildingKind::GunTurret) < 3 {
            Some(BuildingKind::GunTurret)
        } else {
            None
        };
        if let Some(b) = next {
            try_enqueue(&mut production, &mut economy, &owned, Faction::Enemy, Producible::Building(b));
        }
    }

    // --- Harvesters (vehicles lane) ---
    let harvesters = count_unit(&units, Faction::Enemy, UnitKind::Harvester);
    let vehicles_busy = !production.enemy.queues[VEHICLES].items.is_empty();
    if set.contains(&BuildingKind::WarFactory) && harvesters < 2 && !vehicles_busy {
        try_enqueue(
            &mut production,
            &mut economy,
            &owned,
            Faction::Enemy,
            Producible::Unit(UnitKind::Harvester),
        );
    } else {
        // --- Combat units ---
        let combat = count_unit(&units, Faction::Enemy, UnitKind::Soldier)
            + count_unit(&units, Faction::Enemy, UnitKind::Tank);
        if combat < UNIT_CAP {
            if set.contains(&BuildingKind::WarFactory)
                && !vehicles_busy
                && economy.get(Faction::Enemy).can_afford(UnitKind::Tank.cost())
            {
                try_enqueue(
                    &mut production,
                    &mut economy,
                    &owned,
                    Faction::Enemy,
                    Producible::Unit(UnitKind::Tank),
                );
            }
            if set.contains(&BuildingKind::Barracks)
                && production.enemy.queues[INFANTRY].items.is_empty()
            {
                try_enqueue(
                    &mut production,
                    &mut economy,
                    &owned,
                    Faction::Enemy,
                    Producible::Unit(UnitKind::Soldier),
                );
            }
        }
    }
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

fn count_unit(q: &Query<(&Unit, &Faction)>, faction: Faction, kind: UnitKind) -> usize {
    q.iter()
        .filter(|(u, f)| **f == faction && u.kind == kind)
        .count()
}

