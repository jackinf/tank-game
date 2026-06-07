//! Player selection (click + drag box) and issuing move/attack orders.

use crate::components::*;
use crate::config::{SELECT_COLOR, TILE};
use crate::cursor::CursorWorld;
use crate::faction::Faction;
use crate::grid::{find_path, GameMap};
use crate::production::PlacementMode;
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DragSelect {
    pub active: bool,
    pub start: Vec2,
    pub current: Vec2,
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragSelect>().add_systems(
            Update,
            (left_click_select, right_click_command, draw_selection, draw_attack_cursor)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

const DRAG_THRESHOLD: f32 = 6.0;

#[allow(clippy::too_many_arguments)]
fn left_click_select(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<CursorWorld>,
    placement: Res<PlacementMode>,
    mut drag: ResMut<DragSelect>,
    selectables: Query<(Entity, &Transform, &Selectable, &Faction, Option<&Unit>)>,
    selected: Query<Entity, With<Selected>>,
) {
    // Don't select while placing a building.
    if placement.0.is_some() {
        return;
    }

    if mouse.just_pressed(MouseButton::Left) && cursor.valid && !cursor.over_ui {
        drag.active = true;
        drag.start = cursor.pos;
        drag.current = cursor.pos;
    }

    if drag.active && mouse.pressed(MouseButton::Left) && cursor.valid {
        drag.current = cursor.pos;
    }

    if drag.active && mouse.just_released(MouseButton::Left) {
        drag.active = false;
        let box_size = (drag.current - drag.start).abs();

        // Clear current selection.
        for e in &selected {
            commands.entity(e).remove::<Selected>();
        }

        if box_size.x < DRAG_THRESHOLD && box_size.y < DRAG_THRESHOLD {
            // Single click: pick the nearest player-owned thing under the cursor.
            let mut best: Option<(Entity, f32)> = None;
            for (entity, tf, sel, faction, _unit) in &selectables {
                if *faction != Faction::Player {
                    continue;
                }
                let d = tf.translation.truncate().distance(cursor.pos);
                if d <= sel.radius && best.map_or(true, |(_, bd)| d < bd) {
                    best = Some((entity, d));
                }
            }
            if let Some((entity, _)) = best {
                commands.entity(entity).insert(Selected);
            }
        } else {
            // Box select: all player units within the rectangle.
            let min = drag.start.min(drag.current);
            let max = drag.start.max(drag.current);
            for (entity, tf, _sel, faction, unit) in &selectables {
                if *faction != Faction::Player || unit.is_none() {
                    continue;
                }
                let p = tf.translation.truncate();
                if p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y {
                    commands.entity(entity).insert(Selected);
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn right_click_command(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    cursor: Res<CursorWorld>,
    map: Res<GameMap>,
    placement: Res<PlacementMode>,
    targets: Query<(Entity, &Transform, &Selectable, &Faction)>,
    mut selected_buildings: Query<&mut RallyPoint, With<Selected>>,
    mut selected_units: Query<
        (&Transform, &mut Mover, &mut Order),
        (With<Selected>, With<Unit>),
    >,
) {
    // While placing a building a right-click just cancels placement mode
    // (handled in production), so don't also issue movement orders here.
    if placement.0.is_some() {
        return;
    }
    if !mouse.just_pressed(MouseButton::Right) || !cursor.valid || cursor.over_ui {
        return;
    }

    // Set rally point for any selected production buildings.
    for mut rally in &mut selected_buildings {
        rally.0 = cursor.pos;
    }

    // Did we click on an enemy?
    let mut enemy_target: Option<Entity> = None;
    let mut best_d = f32::MAX;
    for (entity, tf, sel, faction) in &targets {
        if *faction == Faction::Enemy {
            let d = tf.translation.truncate().distance(cursor.pos);
            if d <= sel.radius && d < best_d {
                best_d = d;
                enemy_target = Some(entity);
            }
        }
    }

    let attack_move = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    // Gather selected units to assign formation offsets.
    let count = selected_units.iter().count();
    let cols = (count as f32).sqrt().ceil().max(1.0) as usize;

    // A group moves at the pace of its slowest member, so the formation stays
    // together instead of strung out.
    let group_speed = selected_units
        .iter()
        .map(|(_, m, _)| m.base_speed)
        .fold(f32::MAX, f32::min);

    for (i, (transform, mut mover, mut command)) in selected_units.iter_mut().enumerate() {
        if let Some(target) = enemy_target {
            // Attacking units move at their own top speed.
            mover.speed = mover.base_speed;
            *command = Order::Attack(target);
            continue;
        }

        // Formation offset so units don't stack on one tile.
        let row = i / cols;
        let col = i % cols;
        let offset = Vec2::new(
            (col as f32 - (cols as f32 - 1.0) / 2.0) * TILE,
            (row as f32) * TILE,
        );
        let dest = cursor.pos + offset;

        let from = transform.translation.truncate();
        let start = map.world_to_tile(from);
        let goal = map.world_to_tile(dest);
        mover.speed = group_speed;
        mover.path.clear();
        if let Some(path) = find_path(&map, start, goal) {
            // Keep every waypoint on a tile centre so a forced move ends snapped
            // to the grid (the endpoint is the centre of the reachable tile).
            mover.path = path.into_iter().map(|(c, r)| map.tile_center(c, r)).collect();
        }
        *command = if attack_move {
            Order::AttackMove(dest)
        } else {
            Order::Move(dest)
        };
    }
}

/// When the player has units selected and hovers an enemy, draw a red attack
/// reticle at the cursor so it's clear a right-click will open fire.
#[allow(clippy::type_complexity)]
fn draw_attack_cursor(
    mut gizmos: Gizmos,
    cursor: Res<CursorWorld>,
    placement: Res<PlacementMode>,
    selected: Query<(), (With<Selected>, With<Unit>)>,
    enemies: Query<(&Transform, &Selectable, &Faction)>,
) {
    if !cursor.valid || cursor.over_ui || placement.0.is_some() || selected.is_empty() {
        return;
    }
    let hovering_enemy = enemies.iter().any(|(tf, sel, faction)| {
        *faction == Faction::Enemy && tf.translation.truncate().distance(cursor.pos) <= sel.radius
    });
    if !hovering_enemy {
        return;
    }
    let p = cursor.pos;
    let red = Color::srgb(1.0, 0.2, 0.2);
    gizmos.circle_2d(Isometry2d::from_translation(p), 15.0, red);
    // Corner brackets for a targeting-reticle look.
    let r = 15.0;
    for (sx, sy) in [(-1.0, 1.0), (1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)] {
        let corner = p + Vec2::new(sx * r, sy * r);
        gizmos.line_2d(corner, corner - Vec2::new(sx * 6.0, 0.0), red);
        gizmos.line_2d(corner, corner - Vec2::new(0.0, sy * 6.0), red);
    }
}

fn draw_selection(
    mut gizmos: Gizmos,
    drag: Res<DragSelect>,
    selected: Query<(&Transform, &Selectable), With<Selected>>,
) {
    for (transform, sel) in &selected {
        let pos = transform.translation.truncate();
        gizmos.circle_2d(Isometry2d::from_translation(pos), sel.radius + 2.0, SELECT_COLOR);
    }

    if drag.active {
        let min = drag.start.min(drag.current);
        let max = drag.start.max(drag.current);
        let center = (min + max) * 0.5;
        let size = max - min;
        gizmos.rect_2d(Isometry2d::from_translation(center), size, SELECT_COLOR);
    }
}
