//! Shared gameplay components.

use crate::defs::*;
use crate::faction::Faction;
use crate::grid::Tile;
use bevy::prelude::*;
use std::collections::VecDeque;

/// The armour category an entity presents to incoming fire. Combined with a
/// weapon's `Role`, this scales how much damage a shot actually deals.
#[derive(Component, Clone, Copy)]
pub struct Armor(pub ArmorKind);

/// Health for any destructible entity.
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
    pub fn fraction(&self) -> f32 {
        (self.current / self.max).clamp(0.0, 1.0)
    }
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
}

/// Marks an entity the player is allowed to select.
#[derive(Component)]
pub struct Selectable {
    /// Selection radius for click / box tests, world units.
    pub radius: f32,
}

/// Marks a currently-selected entity.
#[derive(Component)]
pub struct Selected;

/// A mobile entity following a path of world-space waypoints.
#[derive(Component)]
pub struct Mover {
    /// Effective speed for the current order (may be lowered to match the
    /// slowest member of a group move).
    pub speed: f32,
    /// The unit's intrinsic top speed; `speed` is reset to this when idle.
    pub base_speed: f32,
    pub path: VecDeque<Vec2>,
}

impl Mover {
    pub fn new(speed: f32) -> Self {
        Self { speed, base_speed: speed, path: VecDeque::new() }
    }
    pub fn is_moving(&self) -> bool {
        !self.path.is_empty()
    }
    pub fn stop(&mut self) {
        self.path.clear();
    }
}

/// High-level order for a controllable unit.
#[derive(Component, Clone, Copy, PartialEq, Default)]
pub enum Order {
    #[default]
    Idle,
    Move(Vec2),
    AttackMove(Vec2),
    Attack(Entity),
}

/// Weapon state attached to an armed entity.
#[derive(Component)]
pub struct WeaponState {
    pub weapon: Weapon,
    pub cooldown: f32,
    /// Currently acquired target, if any.
    pub target: Option<Entity>,
    /// Direction the barrel is pointing (for rendering).
    pub aim: Vec2,
}

impl WeaponState {
    pub fn new(weapon: Weapon) -> Self {
        Self { weapon, cooldown: 0.0, target: None, aim: Vec2::X }
    }
}

/// A building entity.
#[derive(Component)]
pub struct Building {
    pub kind: BuildingKind,
    /// Top-left tile of the footprint.
    pub origin: Tile,
}

/// A unit entity.
#[derive(Component)]
pub struct Unit {
    pub kind: UnitKind,
}

/// Where a production building sends freshly built units.
#[derive(Component)]
pub struct RallyPoint(pub Vec2);

/// An in-flight projectile.
#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub target: Entity,
    pub faction: Faction,
    pub kind: ProjectileKind,
    /// Warhead type; scaled against the target's armour on impact.
    pub role: Role,
    /// Last known target position, in case the target dies mid-flight.
    pub last_seen: Vec2,
}

/// A short-lived visual explosion.
#[derive(Component)]
pub struct Explosion {
    pub age: f32,
    pub lifetime: f32,
    pub radius: f32,
}

/// Marks the main child sprite of a building/unit so we can recolour for
/// build-in / damage flashes if desired (kept simple for now).
#[derive(Component)]
pub struct Body;

/// Marks a tank's rotating turret/barrel sprite (a child of the hull). The
/// turret swivels to face where the tank is aiming or moving.
#[derive(Component)]
pub struct Turret;
