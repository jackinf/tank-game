//! The build menu: the list of things the selected production building can
//! make, kept in sync with the current selection and the player's economy.

use super::*;
use crate::components::{Building, Selected};
use crate::defs::Producible;
use crate::economy::Economy;
use crate::faction::Faction;
use crate::production::{
    prerequisites_met, producible_menu, try_enqueue, OwnedBuildings, ProductionQueue,
};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

/// One row in the build menu: badges (units only), short name, and cost.
fn build_button(parent: &mut RelatedSpawnerCommands<ChildOf>, item: Producible) {
    parent
        .spawn((
            Button,
            Node {
                height: Val::Px(32.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(8.0)),
                column_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(BUTTON_BG),
            BuildButton(item),
        ))
        .with_children(|b| {
            // Left cluster: the two icon badges (units only) + short name.
            b.spawn(Node {
                align_items: AlignItems::Center,
                column_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|left| {
                if let Producible::Unit(u) = item {
                    if let (Some(role), Some(weight)) = (u.role(), u.weight()) {
                        badge(left, role.color());
                        badge(left, weight.color());
                    }
                }
                left.spawn((
                    Text::new(item.short()),
                    TextFont { font_size: 13.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
            b.spawn((
                Text::new(format!("${}", item.cost())),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(1.0, 0.9, 0.4)),
            ));
        });
}

/// A small square colour chip used as a role / armour icon.
fn badge(parent: &mut RelatedSpawnerCommands<ChildOf>, color: Color) {
    parent.spawn((
        Node {
            width: Val::Px(11.0),
            height: Val::Px(11.0),
            ..default()
        },
        BackgroundColor(color),
    ));
}

/// Rebuild the build-button list whenever the selected production building
/// changes. Each building instance shows only what it can make.
#[allow(clippy::type_complexity)]
pub(super) fn sync_build_menu(
    mut commands: Commands,
    mut menu: ResMut<MenuState>,
    root: Query<Entity, With<BuildMenuRoot>>,
    buttons: Query<Entity, With<BuildButton>>,
    selected: Query<(Entity, &Building), (With<Selected>, With<ProductionQueue>)>,
    mut header: Query<&mut Text, With<MenuHeaderText>>,
) {
    let current = selected.iter().next();
    let current_entity = current.map(|(e, _)| e);
    if current_entity == menu.shown {
        return;
    }
    menu.shown = current_entity;

    // Drop the old buttons.
    for b in &buttons {
        commands.entity(b).despawn();
    }

    if let Ok(mut text) = header.single_mut() {
        text.0 = match current {
            Some((_, b)) => b.kind.name().to_string(),
            None => "Select a building".to_string(),
        };
    }

    if let (Some((_, building)), Ok(root)) = (current, root.single()) {
        let kind = building.kind;
        commands.entity(root).with_children(|p| {
            for item in producible_menu(kind) {
                build_button(p, item);
            }
        });
    }
}

pub(super) fn update_button_states(
    economy: Res<Economy>,
    owned: Res<OwnedBuildings>,
    mut buttons: Query<(&BuildButton, &mut BackgroundColor, &Interaction)>,
) {
    let eco = economy.get(Faction::Player);
    for (button, mut bg, interaction) in &mut buttons {
        let item = button.0;
        let enabled =
            prerequisites_met(&owned, Faction::Player, item) && eco.can_afford(item.cost());
        *bg = BackgroundColor(if !enabled {
            BUTTON_DISABLED
        } else if *interaction == Interaction::Hovered {
            Color::srgb(0.28, 0.32, 0.40)
        } else {
            BUTTON_BG
        });
    }
}

pub(super) fn handle_build_buttons(
    mut economy: ResMut<Economy>,
    owned: Res<OwnedBuildings>,
    mut selected: Query<&mut ProductionQueue, With<Selected>>,
    buttons: Query<(&Interaction, &BuildButton), Changed<Interaction>>,
) {
    let Some(mut queue) = selected.iter_mut().next() else {
        return;
    };
    for (interaction, button) in &buttons {
        if *interaction == Interaction::Pressed {
            try_enqueue(&mut queue, &mut economy, &owned, Faction::Player, button.0);
        }
    }
}
