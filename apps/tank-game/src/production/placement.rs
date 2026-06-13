//! Placing finished structures on the map: the placement-mode resource, the
//! mouse/keyboard controls, and the gizmo preview that shows a valid spot.

use super::ProductionQueue;
use crate::components::*;
use crate::config::TILE;
use crate::cursor::CursorWorld;
use crate::defs::*;
use crate::faction::Faction;
use crate::grid::GameMap;
use crate::spawn::{can_place, footprint_center, spawn_building};
use bevy::prelude::*;

/// The building the player is currently placing.
#[derive(Resource, Default)]
pub struct PlacementMode(pub Option<BuildingKind>);

/// Keep placement mode valid and let the player cancel it. Escape (or a
/// right-click) exits placement mode; the building stays in the ready list so
/// it can be placed later. If the chosen building is no longer ready (e.g. it
/// was just placed), placement mode clears itself.
pub(super) fn placement_controls(
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
pub(super) fn placement_input(
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
fn origin_from_cursor(map: &GameMap, cursor: Vec2, footprint: (i32, i32)) -> (i32, i32) {
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

pub(super) fn draw_placement_preview(
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
