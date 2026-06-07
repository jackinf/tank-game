//! Per-building construction queues, prerequisites, and building placement.
//!
//! Every production building (Construction Yard, Barracks, War Factory) owns
//! its own [`ProductionQueue`] component, so two barracks build independently.

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

/// Does this building host a build queue, and what can it make?
pub fn is_producer(kind: BuildingKind) -> bool {
    matches!(
        kind,
        BuildingKind::ConstructionYard | BuildingKind::Barracks | BuildingKind::WarFactory
    )
}

/// The list of things a given production building can build.
pub fn producible_menu(kind: BuildingKind) -> Vec<Producible> {
    match kind {
        BuildingKind::ConstructionYard => BuildingKind::ALL
            .into_iter()
            .filter(|k| *k != BuildingKind::ConstructionYard)
            .map(Producible::Building)
            .collect(),
        BuildingKind::Barracks => UnitKind::infantry().into_iter().map(Producible::Unit).collect(),
        BuildingKind::WarFactory => UnitKind::vehicles().into_iter().map(Producible::Unit).collect(),
        _ => Vec::new(),
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
        app.init_resource::<OwnedBuildings>()
            .init_resource::<PlacementMode>()
            .add_systems(
                Update,
                (
                    update_owned_buildings,
                    tick_production,
                    placement_controls,
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

/// Try to add an item to a building's queue. Charges credits up front.
pub fn try_enqueue(
    queue: &mut ProductionQueue,
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
    economy.get_mut(faction).credits -= cost;
    queue.items.push_back(item);
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

/// Keep placement mode valid and let the player cancel it. Escape (or a
/// right-click) exits placement mode; the building stays in the ready list so
/// it can be placed later. If the chosen building is no longer ready (e.g. it
/// was just placed), placement mode clears itself.
fn placement_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut placement: ResMut<PlacementMode>,
    yards: Query<(&Building, &Faction, &ProductionQueue)>,
) {
    let Some(kind) = placement.0 else { return };

    if keys.just_pressed(KeyCode::Escape) || mouse.just_pressed(MouseButton::Right) {
        placement.0 = None;
        return;
    }

    // Drop placement mode if no copy of this building is still waiting.
    let still_ready = yards.iter().any(|(b, f, q)| {
        *f == Faction::Player
            && b.kind == BuildingKind::ConstructionYard
            && q.ready.contains(&kind)
    });
    if !still_ready {
        placement.0 = None;
    }
}

#[allow(clippy::too_many_arguments)]
fn placement_input(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<CursorWorld>,
    mut map: ResMut<GameMap>,
    mut placement: ResMut<PlacementMode>,
    mut yards: Query<(&Building, &Faction, &mut ProductionQueue)>,
    buildings: Query<(&Building, &Faction, &Transform)>,
) {
    let Some(kind) = placement.0 else { return };
    if !mouse.just_pressed(MouseButton::Left) || !cursor.valid || cursor.over_ui {
        return;
    }
    let footprint = kind.footprint();
    let origin = origin_from_cursor(&map, cursor.pos, footprint);

    if can_place(&map, origin, footprint)
        && near_friendly_building(&map, origin, footprint, &buildings)
    {
        spawn_building(&mut commands, &mut map, kind, Faction::Player, origin);
        // Consume one queued copy from the player's Construction Yard.
        let mut more_ready = false;
        for (b, f, mut q) in &mut yards {
            if *f == Faction::Player && b.kind == BuildingKind::ConstructionYard {
                q.take_ready(kind);
                more_ready = q.ready.contains(&kind);
            }
        }
        // Stay in placement mode while more of the same kind are waiting,
        // otherwise exit so a stray click doesn't keep placing.
        if !more_ready {
            placement.0 = None;
        }
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
