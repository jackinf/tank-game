//! HUD: top bar (credits / power), the sidebar build menu with a construction
//! queue, and the game-over overlay.

use crate::config::*;
use crate::defs::*;
use crate::economy::Economy;
use crate::faction::Faction;
use crate::production::{prerequisites_met, try_enqueue, OwnedBuildings, Production};
use crate::state::{GameResult, GameState};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(
                Update,
                (
                    update_hud_text,
                    update_button_states,
                    handle_build_buttons,
                    update_queue_text,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over)
            .add_systems(Update, restart_input.run_if(in_state(GameState::GameOver)));
    }
}

// --- Markers -------------------------------------------------------------

#[derive(Component)]
struct CreditsText;
#[derive(Component)]
struct PowerText;
#[derive(Component)]
struct BuildButton(Producible);
#[derive(Component)]
struct QueueText(usize);
#[derive(Component)]
struct GameOverUi;

const PANEL_BG: Color = Color::srgb(0.10, 0.11, 0.13);
const BUTTON_BG: Color = Color::srgb(0.20, 0.22, 0.26);
const BUTTON_DISABLED: Color = Color::srgb(0.14, 0.14, 0.16);

// --- HUD construction ----------------------------------------------------

fn spawn_hud(mut commands: Commands) {
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
                    "LMB select / drag-box   RMB move (Ctrl=attack-move) / set rally   +/- zoom   WASD pan",
                ),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
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
            section_title(p, "STRUCTURES");
            for kind in BuildingKind::ALL {
                if kind == BuildingKind::ConstructionYard {
                    continue;
                }
                build_button(p, Producible::Building(kind));
            }
            section_title(p, "UNITS");
            for kind in UnitKind::ALL {
                build_button(p, Producible::Unit(kind));
            }

            section_title(p, "PRODUCTION");
            for (lane, name) in [(0usize, "Structures"), (1, "Infantry"), (2, "Vehicles")] {
                p.spawn((
                    Text::new(format!("{name}: idle")),
                    TextFont { font_size: 13.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    QueueText(lane),
                ));
            }
        });
}

fn section_title(parent: &mut RelatedSpawnerCommands<ChildOf>, title: &str) {
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

fn build_button(parent: &mut RelatedSpawnerCommands<ChildOf>, item: Producible) {
    parent
        .spawn((
            Button,
            Node {
                height: Val::Px(34.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(BUTTON_BG),
            BuildButton(item),
        ))
        .with_children(|b| {
            b.spawn((
                Text::new(item.short()),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::WHITE),
            ));
            b.spawn((
                Text::new(format!("${}", item.cost())),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(1.0, 0.9, 0.4)),
            ));
        });
}

// --- HUD updates ---------------------------------------------------------

fn update_hud_text(
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

fn update_button_states(
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

fn handle_build_buttons(
    mut production: ResMut<Production>,
    mut economy: ResMut<Economy>,
    owned: Res<OwnedBuildings>,
    buttons: Query<(&Interaction, &BuildButton), Changed<Interaction>>,
) {
    for (interaction, button) in &buttons {
        if *interaction == Interaction::Pressed {
            try_enqueue(
                &mut production,
                &mut economy,
                &owned,
                Faction::Player,
                button.0,
            );
        }
    }
}

fn update_queue_text(production: Res<Production>, mut texts: Query<(&QueueText, &mut Text)>) {
    let names = ["Structures", "Infantry", "Vehicles"];
    for (lane, mut text) in &mut texts {
        let q = &production.player.queues[lane.0];
        let name = names[lane.0];
        let status = if let Some(b) = q.ready_structure {
            format!("READY: place {}", b.short())
        } else if let Some(front) = q.front() {
            let pct = (q.fraction() * 100.0) as i32;
            let extra = q.items.len().saturating_sub(1);
            if extra > 0 {
                format!("{} {}%  (+{})", front.short(), pct, extra)
            } else {
                format!("{} {}%", front.short(), pct)
            }
        } else {
            "idle".to_string()
        };
        text.0 = format!("{name}: {status}");
    }
}

// --- Game over -----------------------------------------------------------

fn spawn_game_over(mut commands: Commands, result: Option<Res<GameResult>>) {
    let (msg, color) = match result.as_deref() {
        Some(GameResult::Victory) => ("VICTORY!", Color::srgb(0.4, 1.0, 0.5)),
        _ => ("DEFEAT", Color::srgb(1.0, 0.4, 0.4)),
    };
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            GameOverUi,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(msg),
                TextFont { font_size: 72.0, ..default() },
                TextColor(color),
            ));
            p.spawn((
                Text::new("Press R to play again"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

fn despawn_game_over(mut commands: Commands, ui: Query<Entity, With<GameOverUi>>) {
    for e in &ui {
        commands.entity(e).despawn();
    }
}

fn restart_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::KeyR) {
        next.set(GameState::Loading);
    }
}
