//! Helpers to spawn buildings and units with all their components and child
//! sprites. Shared by setup, production and the AI.

use crate::components::*;
use crate::config::{z, TILE};
use crate::defs::*;
use crate::faction::Faction;
use crate::grid::{GameMap, Tile};
use crate::harvester::Harvester;
use crate::state::GameEntity;
use bevy::prelude::*;

/// Can a building of `footprint` (w,h) be placed with top-left at `origin`?
pub fn can_place(map: &GameMap, origin: Tile, footprint: (i32, i32)) -> bool {
    for dr in 0..footprint.1 {
        for dc in 0..footprint.0 {
            let (c, r) = (origin.0 + dc, origin.1 + dr);
            if !map.in_bounds(c, r) || map.is_blocked(c, r) || map.ore_at(c, r) > 0 {
                return false;
            }
        }
    }
    true
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
        Health::new(kind.max_health()),
        Selectable { radius: size.length() * 0.4 },
        Sprite::from_color(kind.accent(), size),
        Transform::from_xyz(center.x, center.y, z::BUILDING),
        GameEntity,
    ));

    // Ownership marker (faction-coloured inner square).
    e.with_children(|p| {
        p.spawn((
            Sprite::from_color(faction.color(), size * 0.45),
            Transform::from_xyz(0.0, 0.0, 0.1),
        ));
    });

    if let Some(weapon) = kind.weapon() {
        e.insert(WeaponState::new(weapon));
    }

    // Production buildings get a rally point just below the footprint.
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
    let body = match kind {
        UnitKind::Soldier => Vec2::splat(r * 2.0),
        UnitKind::Tank => Vec2::splat(r * 2.0),
        UnitKind::Harvester => Vec2::new(r * 2.2, r * 1.7),
    };

    let mut e = commands.spawn((
        Unit { kind },
        faction,
        Health::new(kind.max_health()),
        Selectable { radius: r + 3.0 },
        Mover::new(kind.speed()),
        Order::Idle,
        Sprite::from_color(faction.color(), body),
        Transform::from_xyz(pos.x, pos.y, z::UNIT),
        GameEntity,
    ));

    // Inner detail square for a bit of contrast.
    let detail = match kind {
        UnitKind::Tank => Color::srgb(0.15, 0.15, 0.18),
        UnitKind::Harvester => Color::srgb(0.85, 0.75, 0.2),
        UnitKind::Soldier => Color::srgb(0.1, 0.1, 0.1),
    };
    e.with_children(|p| {
        p.spawn((
            Sprite::from_color(detail, body * 0.5),
            Transform::from_xyz(0.0, 0.0, 0.1),
        ));
    });

    if let Some(weapon) = kind.weapon() {
        e.insert(WeaponState::new(weapon));
    }
    if kind == UnitKind::Harvester {
        e.insert(Harvester::default());
    }

    e.id()
}
