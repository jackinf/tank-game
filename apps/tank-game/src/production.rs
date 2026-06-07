//! Construction queues, prerequisites, and building placement.

use crate::components::*;
use crate::config::TILE;
use crate::cursor::CursorWorld;
use crate::defs::*;
use crate::economy::Economy;
use crate::faction::Faction;
use crate::grid::GameMap;
use crate::spawn::{can_place, footprint_center, spawn_building, spawn_unit};
use crate::state::GameState;
use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};

/// The three production lanes, each with an independent queue.
pub const STRUCTURES: usize = 0;
pub const INFANTRY: usize = 1;
pub const VEHICLES: usize = 2;

pub fn category(p: Producible) -> usize {
    match p {
        Producible::Building(_) => STRUCTURES,
        Producible::Unit(UnitKind::Soldier) => INFANTRY,
        Producible::Unit(_) => VEHICLES,
    }
}

#[derive(Default)]
pub struct Queue {
    pub items: VecDeque<Producible>,
    /// Seconds elapsed building the front item.
    pub progress: f32,
    /// A finished structure awaiting placement (player) / spawning (AI).
    pub ready_structure: Option<BuildingKind>,
}

impl Queue {
    pub fn front(&self) -> Option<Producible> {
        self.items.front().copied()
    }
    pub fn fraction(&self) -> f32 {
        match self.front() {
            Some(p) => (self.progress / p.build_time()).clamp(0.0, 1.0),
            None => 0.0,
        }
    }
}

#[derive(Default)]
pub struct FactionProduction {
    pub queues: [Queue; 3],
}

#[derive(Resource, Default)]
pub struct Production {
    pub player: FactionProduction,
    pub enemy: FactionProduction,
}

impl Production {
    pub fn get(&self, f: Faction) -> &FactionProduction {
        match f {
            Faction::Enemy => &self.enemy,
            _ => &self.player,
        }
    }
    pub fn get_mut(&mut self, f: Faction) -> &mut FactionProduction {
        match f {
            Faction::Enemy => &mut self.enemy,
            _ => &mut self.player,
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

/// The building the player is currently placing.
#[derive(Resource, Default)]
pub struct PlacementMode(pub Option<BuildingKind>);

pub struct ProductionPlugin;

impl Plugin for ProductionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Production>()
            .init_resource::<OwnedBuildings>()
            .init_resource::<PlacementMode>()
            .add_systems(
                Update,
                (
                    update_owned_buildings,
                    tick_production,
                    sync_placement_mode,
                    placement_input,
                    draw_placement_preview,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Whether `faction` meets the prerequisites to start building `item`.
pub fn prerequisites_met(owned: &OwnedBuildings, faction: Faction, item: Producible) -> bool {
    let set = owned.get(faction);
    match item {
        Producible::Building(b) => b.prerequisite().map_or(true, |p| set.contains(&p)),
        Producible::Unit(u) => set.contains(&u.prerequisite()),
    }
}

/// Try to add an item to the appropriate queue. Charges credits up front.
pub fn try_enqueue(
    production: &mut Production,
    economy: &mut Economy,
    owned: &OwnedBuildings,
    faction: Faction,
    item: Producible,
) -> bool {
    if !prerequisites_met(owned, faction, item) {
        return false;
    }
    let cost = item.cost();
    if !economy.get(faction).can_afford(cost) {
        return false;
    }
    let cat = category(item);
    economy.get_mut(faction).credits -= cost;
    production.get_mut(faction).queues[cat].items.push_back(item);
    true
}

fn update_owned_buildings(
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
fn tick_production(
    time: Res<Time>,
    mut commands: Commands,
    mut production: ResMut<Production>,
    economy: Res<Economy>,
    rally_buildings: Query<(&Building, &Faction, &Transform, &RallyPoint)>,
) {
    let dt = time.delta_secs();

    for faction in [Faction::Player, Faction::Enemy] {
        let power_factor = economy.get(faction).power_factor();
        // Collect completed units to spawn after releasing the borrow.
        let mut to_spawn: Vec<UnitKind> = Vec::new();

        {
            let fp = production.get_mut(faction);
            for (lane, queue) in fp.queues.iter_mut().enumerate() {
                // Structures lane pauses while a finished structure awaits placement.
                if lane == STRUCTURES && queue.ready_structure.is_some() {
                    continue;
                }
                let Some(front) = queue.front() else { continue };
                queue.progress += dt * power_factor;
                if queue.progress >= front.build_time() {
                    queue.progress = 0.0;
                    queue.items.pop_front();
                    match front {
                        Producible::Building(b) => queue.ready_structure = Some(b),
                        Producible::Unit(u) => to_spawn.push(u),
                    }
                }
            }
        }

        for unit in to_spawn {
            spawn_completed_unit(&mut commands, &rally_buildings, faction, unit);
        }
    }
}

fn spawn_completed_unit(
    commands: &mut Commands,
    rally_buildings: &Query<(&Building, &Faction, &Transform, &RallyPoint)>,
    faction: Faction,
    unit: UnitKind,
) {
    let produced_by = unit.produced_by();
    // Find the producing building.
    let mut spawn_pos = None;
    let mut rally = None;
    for (building, bf, tf, rp) in rally_buildings.iter() {
        if *bf == faction && building.kind == produced_by {
            spawn_pos = Some(tf.translation.truncate());
            rally = Some(rp.0);
            break;
        }
    }
    let Some(base) = spawn_pos else { return };
    let rally = rally.unwrap_or(base);

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

/// Mirror the player's finished structure into the placement mode resource.
fn sync_placement_mode(production: Res<Production>, mut placement: ResMut<PlacementMode>) {
    placement.0 = production.player.queues[STRUCTURES].ready_structure;
}

#[allow(clippy::too_many_arguments)]
fn placement_input(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<CursorWorld>,
    mut map: ResMut<GameMap>,
    mut production: ResMut<Production>,
    placement: Res<PlacementMode>,
    buildings: Query<(&Building, &Faction, &Transform)>,
) {
    let Some(kind) = placement.0 else { return };
    if !mouse.just_pressed(MouseButton::Left) || !cursor.valid || cursor.over_ui {
        return;
    }
    let footprint = kind.footprint();
    let origin = origin_from_cursor(&map, cursor.pos, footprint);

    if can_place(&map, origin, footprint) && near_friendly_building(&map, origin, footprint, &buildings)
    {
        spawn_building(&mut commands, &mut map, kind, Faction::Player, origin);
        production.player.queues[STRUCTURES].ready_structure = None;
    }
}

/// Top-left footprint tile so the building is centred under the cursor.
pub fn origin_from_cursor(map: &GameMap, cursor: Vec2, footprint: (i32, i32)) -> (i32, i32) {
    let (c, r) = map.world_to_tile(cursor);
    (c - footprint.0 / 2, r - footprint.1 / 2)
}

/// Buildings must be placed near existing friendly structures.
fn near_friendly_building(
    map: &GameMap,
    origin: (i32, i32),
    footprint: (i32, i32),
    buildings: &Query<(&Building, &Faction, &Transform)>,
) -> bool {
    let center = footprint_center(map, origin, footprint);
    const BUILD_RADIUS: f32 = TILE * 8.0;
    buildings.iter().any(|(_, f, tf)| {
        *f == Faction::Player && tf.translation.truncate().distance(center) < BUILD_RADIUS
    })
}

fn draw_placement_preview(
    mut gizmos: Gizmos,
    cursor: Res<CursorWorld>,
    map: Res<GameMap>,
    placement: Res<PlacementMode>,
    buildings: Query<(&Building, &Faction, &Transform)>,
) {
    let Some(kind) = placement.0 else { return };
    if !cursor.valid || cursor.over_ui {
        return;
    }
    let footprint = kind.footprint();
    let origin = origin_from_cursor(&map, cursor.pos, footprint);
    let center = footprint_center(&map, origin, footprint);
    let size = Vec2::new(footprint.0 as f32 * TILE, footprint.1 as f32 * TILE);
    let valid = can_place(&map, origin, footprint)
        && near_friendly_building(&map, origin, footprint, &buildings);
    let color = if valid {
        Color::srgb(0.2, 1.0, 0.3)
    } else {
        Color::srgb(1.0, 0.2, 0.2)
    };
    gizmos.rect_2d(Isometry2d::from_translation(center), size, color);
    // Show the footprint grid lines.
    for dc in 0..=footprint.0 {
        let x = center.x - size.x / 2.0 + dc as f32 * TILE;
        gizmos.line_2d(
            Vec2::new(x, center.y - size.y / 2.0),
            Vec2::new(x, center.y + size.y / 2.0),
            color,
        );
    }
}
