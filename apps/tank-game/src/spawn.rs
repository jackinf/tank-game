//! Helpers to spawn buildings and units with all their components and child
//! sprites. Shared by setup, production and the AI.

use crate::components::*;
use crate::config::{z, TILE};
use crate::defs::*;
use crate::faction::Faction;
use crate::grid::{GameMap, Tile};
use crate::harvester::Harvester;
use crate::production::{is_producer, ProductionQueue};
use crate::state::GameEntity;
use bevy::prelude::*;

/// Can a building of `footprint` (w,h) be placed with top-left at `origin`?
/// Enforces a one-tile gap to every other building so bases never pack solid.
pub fn can_place(map: &GameMap, origin: Tile, footprint: (i32, i32)) -> bool {
    map.can_build(origin, footprint)
}

/// Search outward in rings from `near` for a tile where a building of
/// `footprint` can be placed.
pub fn find_spot(map: &GameMap, near: Tile, footprint: (i32, i32)) -> Option<Tile> {
    if can_place(map, near, footprint) {
        return Some(near);
    }
    for radius in 1i32..28 {
        for dr in -radius..=radius {
            for dc in -radius..=radius {
                if dr.abs() != radius && dc.abs() != radius {
                    continue; // only the outer ring
                }
                let origin = (near.0 + dc, near.1 + dr);
                if can_place(map, origin, footprint) {
                    return Some(origin);
                }
            }
        }
    }
    None
}

/// World-space centre of a footprint placed at `origin`.
pub fn footprint_center(map: &GameMap, origin: Tile, footprint: (i32, i32)) -> Vec2 {
    let a = map.tile_center(origin.0, origin.1);
    let b = map.tile_center(origin.0 + footprint.0 - 1, origin.1 + footprint.1 - 1);
    (a + b) * 0.5
}

pub fn spawn_building(
    commands: &mut Commands,
    map: &mut GameMap,
    kind: BuildingKind,
    faction: Faction,
    origin: Tile,
) -> Entity {
    let footprint = kind.footprint();
    // Reserve the tiles.
    for dr in 0..footprint.1 {
        for dc in 0..footprint.0 {
            map.set_blocked(origin.0 + dc, origin.1 + dr, true);
            map.set_built(origin.0 + dc, origin.1 + dr, true);
        }
    }

    let center = footprint_center(map, origin, footprint);
    let size = Vec2::new(
        footprint.0 as f32 * TILE - 3.0,
        footprint.1 as f32 * TILE - 3.0,
    );

    let mut e = commands.spawn((
        Building { kind, origin },
        faction,
        Armor(ArmorKind::Building),
        Health::new(kind.max_health()),
        Selectable { radius: size.length() * 0.4 },
        Sprite::from_color(kind.accent(), size),
        Transform::from_xyz(center.x, center.y, z::BUILDING),
        GameEntity,
    ));

    // Ownership marker (faction-coloured inner square) + a name label above.
    // Skip the marker for textured buildings so it doesn't cover the artwork;
    // the faction-coloured label still signals ownership.
    e.with_children(|p| {
        if kind.texture_path().is_none() {
            p.spawn((
                Sprite::from_color(faction.color(), size * 0.45),
                Transform::from_xyz(0.0, 0.0, 0.1),
            ));
        }
        // Building label, in world space, just above the footprint.
        let label_color = if faction == Faction::Enemy {
            Color::srgb(1.0, 0.75, 0.7)
        } else {
            Color::srgb(0.92, 0.95, 1.0)
        };
        p.spawn((
            Text2d::new(kind.short()),
            TextFont { font_size: 11.0, ..default() },
            TextColor(label_color),
            Transform::from_xyz(0.0, size.y * 0.5 + 9.0, z::LABEL - z::BUILDING),
        ));
    });

    if let Some(weapon) = kind.weapon() {
        e.insert(WeaponState::new(weapon));
    }

    // Production buildings get their own queue and a rally point.
    if is_producer(kind) {
        e.insert(ProductionQueue::default());
    }
    if matches!(kind, BuildingKind::Barracks | BuildingKind::WarFactory) {
        let rally_tile = (origin.0 + footprint.0 / 2, origin.1 + footprint.1 + 1);
        let rally = map.tile_center(rally_tile.0, rally_tile.1);
        e.insert(RallyPoint(rally));
    }

    e.id()
}

pub fn spawn_unit(
    commands: &mut Commands,
    kind: UnitKind,
    faction: Faction,
    pos: Vec2,
) -> Entity {
    let r = kind.radius();
    // Parent body: a torso-ish rectangle for infantry, a square hull for tanks.
    let body = match kind {
        UnitKind::Harvester => Vec2::new(r * 2.2, r * 1.7),
        UnitKind::Combat {
            class: UnitClass::Infantry,
            ..
        } => Vec2::new(r * 1.2, r * 1.9),
        UnitKind::Combat {
            class: UnitClass::Vehicle,
            ..
        } => Vec2::splat(r * 2.0),
    };

    let mut e = commands.spawn((
        Unit { kind },
        faction,
        Armor(kind.armor_kind()),
        Health::new(kind.max_health()),
        Selectable { radius: r + 3.0 },
        Mover::new(kind.speed()),
        Order::Idle,
        Sprite::from_color(faction.color(), body),
        Transform::from_xyz(pos.x, pos.y, z::UNIT),
        GameEntity,
    ));

    e.with_children(|p| {
        match kind {
            UnitKind::Harvester => {
                p.spawn((
                    Sprite::from_color(Color::srgb(0.85, 0.75, 0.2), body * 0.5),
                    Transform::from_xyz(0.0, 0.0, 0.1),
                ));
            }
            UnitKind::Combat {
                class: UnitClass::Vehicle,
                ..
            } => {
                // Dark hull detail.
                p.spawn((
                    Sprite::from_color(Color::srgb(0.15, 0.15, 0.18), body * 0.5),
                    Transform::from_xyz(0.0, 0.0, 0.1),
                ));
                // Rotating turret: a round-ish base plus a barrel that extends
                // forward from the hull centre. `rotate_turrets` spins it to
                // face the aim/move direction.
                p.spawn((
                    Turret,
                    Sprite::from_color(Color::srgb(0.22, 0.22, 0.26), Vec2::splat(r * 1.1)),
                    Transform::from_xyz(0.0, 0.0, 0.15),
                ))
                .with_children(|t| {
                    t.spawn((
                        Sprite::from_color(
                            Color::srgb(0.1, 0.1, 0.12),
                            Vec2::new(r * 1.6, r * 0.4),
                        ),
                        // Anchor the barrel at its left edge so it extends
                        // forward from the turret's pivot.
                        bevy::sprite::Anchor::CENTER_LEFT,
                        Transform::from_xyz(0.0, 0.0, 0.02),
                    ));
                });
            }
            UnitKind::Combat {
                class: UnitClass::Infantry,
                ..
            } => {
                // A pixel person: head on top, the faction torso is the parent
                // body, two dark legs below.
                let skin = Color::srgb(0.92, 0.78, 0.62);
                let head = Vec2::splat(r * 0.95);
                p.spawn((
                    Sprite::from_color(skin, head),
                    Transform::from_xyz(0.0, body.y * 0.5 + head.y * 0.35, 0.1),
                ));
                let leg = Vec2::new(r * 0.45, r * 0.9);
                let dark = Color::srgb(0.1, 0.1, 0.12);
                p.spawn((
                    Sprite::from_color(dark, leg),
                    Transform::from_xyz(-r * 0.35, -body.y * 0.5 - leg.y * 0.3, 0.05),
                ));
                p.spawn((
                    Sprite::from_color(dark, leg),
                    Transform::from_xyz(r * 0.35, -body.y * 0.5 - leg.y * 0.3, 0.05),
                ));
            }
        }

        // Two badge icons for combat units: weapon role (left) + armour (right).
        if let (Some(role), Some(weight)) = (kind.role(), kind.weight()) {
            let badge = Vec2::splat((r * 0.7).max(4.0));
            let y = body.y * 0.5 + r * 1.0;
            p.spawn((
                Sprite::from_color(role.color(), badge),
                Transform::from_xyz(-badge.x * 0.65, y, 0.2),
            ));
            p.spawn((
                Sprite::from_color(weight.color(), badge),
                Transform::from_xyz(badge.x * 0.65, y, 0.2),
            ));
        }
    });

    if let Some(weapon) = kind.weapon() {
        e.insert(WeaponState::new(weapon));
    }
    if kind == UnitKind::Harvester {
        e.insert(Harvester::default());
    }

    e.id()
}
