//! The static HUD layout (top bar, vertical power gauge, sidebar) spawned once
//! at startup, plus the credits / power readout text.

use super::*;
use crate::config::*;
use crate::economy::Economy;
use crate::faction::Faction;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

pub(super) fn spawn_hud(mut commands: Commands) {
    // Top bar.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(TOPBAR_HEIGHT),
                align_items: AlignItems::Center,
                column_gap: Val::Px(24.0),
                padding: UiRect::horizontal(Val::Px(14.0)),
                ..default()
            },
            BackgroundColor(PANEL_BG),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Credits: 5000"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(1.0, 0.9, 0.4)),
                CreditsText,
            ));
            p.spawn((
                Text::new("Power: 0/0"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.5, 0.8, 1.0)),
                PowerText,
            ));
            p.spawn((
                Text::new(
                    "LMB select / place   RMB move (Ctrl=attack-move) / rally / cancel   Esc cancel placement   G grid   +/- zoom   WASD pan",
                ),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });

    // Vertical power gauge, fixed to the left edge below the top bar.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(TOPBAR_HEIGHT + 12.0),
                width: Val::Px(18.0),
                height: Val::Px(150.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.10)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.35)),
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.9, 0.2)),
                PowerBarFill,
            ));
        });

    // Sidebar.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(SIDEBAR_WIDTH),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                row_gap: Val::Px(4.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(PANEL_BG),
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    margin: UiRect::bottom(Val::Px(4.0)),
                    ..default()
                },
                Text::new("Select a building"),
                TextFont { font_size: 15.0, ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.6)),
                MenuHeaderText,
            ));
            // Dynamic build-button area.
            p.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    row_gap: Val::Px(4.0),
                    ..default()
                },
                BuildMenuRoot,
            ));
            section_title(p, "QUEUE");
            p.spawn((
                Text::new("—"),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                MenuQueueText,
            ));
            // Finished structures waiting to be placed on the map.
            section_title(p, "READY TO PLACE");
            p.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    row_gap: Val::Px(4.0),
                    ..default()
                },
                ReadyMenuRoot,
            ));
            // Spacer pushes the description to the bottom of the sidebar.
            p.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });
            section_title(p, "INFO");
            p.spawn((
                Text::new("Hover a build option or select a unit to see its strengths and weaknesses."),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.75, 0.78, 0.82)),
                DescriptionText,
            ));
        });
}

/// A sidebar section heading ("QUEUE", "READY TO PLACE", "INFO").
pub(super) fn section_title(parent: &mut RelatedSpawnerCommands<ChildOf>, title: &str) {
    parent.spawn((
        Node {
            margin: UiRect::top(Val::Px(8.0)),
            ..default()
        },
        Text::new(title),
        TextFont { font_size: 15.0, ..default() },
        TextColor(Color::srgb(0.9, 0.9, 0.6)),
    ));
}

pub(super) fn update_hud_text(
    economy: Res<Economy>,
    mut credits: Query<&mut Text, (With<CreditsText>, Without<PowerText>)>,
    mut power: Query<&mut Text, (With<PowerText>, Without<CreditsText>)>,
) {
    let eco = economy.get(Faction::Player);
    if let Ok(mut text) = credits.single_mut() {
        text.0 = format!("Credits: {}", eco.credits);
    }
    if let Ok(mut text) = power.single_mut() {
        let warn = if eco.has_power() { "" } else { "  (LOW!)" };
        text.0 = format!("Power: {}/{}{}", eco.power_produced, eco.power_consumed, warn);
    }
}
