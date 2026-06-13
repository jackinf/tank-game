//! The "ready to place" menu: finished structures from the Construction Yard
//! waiting for the player to drop them on the map.

use super::*;
use crate::components::Building;
use crate::defs::BuildingKind;
use crate::faction::Faction;
use crate::production::{PlacementMode, ProductionQueue};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

/// A button that puts a finished structure into placement mode.
fn place_button(parent: &mut RelatedSpawnerCommands<ChildOf>, kind: BuildingKind, count: usize) {
    parent
        .spawn((
            Button,
            Node {
                height: Val::Px(30.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.30, 0.20)),
            PlaceButton(kind),
        ))
        .with_children(|b| {
            b.spawn((
                Text::new(format!("Place {}", kind.short())),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(0.6, 1.0, 0.7)),
            ));
            if count > 1 {
                b.spawn((
                    Text::new(format!("x{count}")),
                    TextFont { font_size: 12.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.95, 0.9)),
                ));
            }
        });
}

/// Rebuild the "ready to place" buttons whenever the player's set of finished
/// structures changes.
pub(super) fn sync_ready_menu(
    mut commands: Commands,
    mut shown: ResMut<ReadyShown>,
    root: Query<Entity, With<ReadyMenuRoot>>,
    buttons: Query<Entity, With<PlaceButton>>,
    yards: Query<(&Building, &Faction, &ProductionQueue)>,
) {
    // Collect the player's ready structures (with their build order preserved).
    let mut ready: Vec<BuildingKind> = Vec::new();
    for (b, f, q) in &yards {
        if *f == Faction::Player && b.kind == BuildingKind::ConstructionYard {
            ready.extend(q.ready.iter().copied());
        }
    }
    if ready == shown.0 {
        return;
    }
    shown.0 = ready.clone();

    for e in &buttons {
        commands.entity(e).despawn();
    }
    let Ok(root) = root.single() else { return };

    // Collapse duplicates into "Place X  x2", keeping first-seen order.
    let mut order: Vec<BuildingKind> = Vec::new();
    for k in &ready {
        if !order.contains(k) {
            order.push(*k);
        }
    }
    commands.entity(root).with_children(|p| {
        for kind in order {
            let count = ready.iter().filter(|&&k| k == kind).count();
            place_button(p, kind, count);
        }
    });
}

/// Clicking a "ready to place" button toggles placement mode for that building.
pub(super) fn handle_place_buttons(
    mut placement: ResMut<PlacementMode>,
    buttons: Query<(&Interaction, &PlaceButton), Changed<Interaction>>,
) {
    for (interaction, button) in &buttons {
        if *interaction == Interaction::Pressed {
            placement.0 = if placement.0 == Some(button.0) {
                None
            } else {
                Some(button.0)
            };
        }
    }
}
