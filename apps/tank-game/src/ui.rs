//! HUD: top bar (credits / power), a selection-driven sidebar build menu that
//! shows the queue of whichever production building is selected, and the
//! game-over overlay.

use crate::components::*;
use crate::config::*;
use crate::defs::*;
use crate::economy::Economy;
use crate::faction::Faction;
use crate::production::{
    prerequisites_met, producible_menu, try_enqueue, OwnedBuildings, PlacementMode,
    ProductionQueue,
};
use crate::state::{GameResult, GameState};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuState>()
            .init_resource::<ReadyShown>()
            .add_systems(Startup, spawn_hud)
            .add_systems(
                Update,
                (
                    update_hud_text,
                    update_power_bar,
                    sync_build_menu,
                    update_button_states,
                    handle_build_buttons,
                    sync_ready_menu,
                    handle_place_buttons,
                    update_queue_text,
                    update_description,
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
/// Container whose children are the (dynamic) build buttons.
#[derive(Component)]
struct BuildMenuRoot;
#[derive(Component)]
struct MenuHeaderText;
#[derive(Component)]
struct MenuQueueText;
#[derive(Component)]
struct GameOverUi;
/// A button that enters placement mode for a finished structure.
#[derive(Component)]
struct PlaceButton(BuildingKind);
/// Container whose children are the (dynamic) "ready to place" buttons.
#[derive(Component)]
struct ReadyMenuRoot;
/// The fill element of the vertical power gauge.
#[derive(Component)]
struct PowerBarFill;
/// The strengths/weaknesses blurb shown at the bottom of the sidebar.
#[derive(Component)]
struct DescriptionText;

/// The set of ready-to-place buildings shown last frame, so we only rebuild the
/// buttons when it actually changes.
#[derive(Resource, Default)]
struct ReadyShown(Vec<BuildingKind>);

/// Which production building's menu is currently displayed.
#[derive(Resource, Default)]
struct MenuState {
    shown: Option<Entity>,
}

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
            b.spawn((
                Node {
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(4.0),
                    ..default()
                },
            ))
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

/// Rebuild the build-button list whenever the selected production building
/// changes. Each building instance shows only what it can make.
#[allow(clippy::type_complexity)]
fn sync_build_menu(
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

// --- Power gauge ---------------------------------------------------------

/// Drive the vertical power bar. The fill height shows how much of produced
/// power is being consumed; green = plenty of headroom, yellow = getting tight,
/// red = over budget (production is throttled).
fn update_power_bar(
    economy: Res<Economy>,
    mut fill: Query<(&mut Node, &mut BackgroundColor), With<PowerBarFill>>,
) {
    let Ok((mut node, mut bg)) = fill.single_mut() else {
        return;
    };
    let eco = economy.get(Faction::Player);
    let produced = eco.power_produced.max(0) as f32;
    let consumed = eco.power_consumed as f32;
    let load = consumed / produced.max(1.0);
    // Fill = remaining headroom: a tall bar means plenty of spare power, an
    // empty bar means we're at (or over) capacity.
    let headroom = (1.0 - load).clamp(0.0, 1.0);
    node.height = Val::Percent((headroom * 100.0).max(4.0));
    *bg = BackgroundColor(if load < 0.75 {
        Color::srgb(0.2, 0.9, 0.2)
    } else if load < 1.0 {
        Color::srgb(0.95, 0.8, 0.2)
    } else {
        Color::srgb(0.95, 0.25, 0.2)
    });
}

// --- Ready-to-place menu -------------------------------------------------

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
fn sync_ready_menu(
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
fn handle_place_buttons(
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

// --- Info / description panel --------------------------------------------

/// Show a contextual strengths/weaknesses blurb: the building being placed, the
/// build option under the cursor, or the single selected unit.
#[allow(clippy::type_complexity)]
fn update_description(
    placement: Res<PlacementMode>,
    hovered: Query<(&BuildButton, &Interaction)>,
    selected_units: Query<&Unit, With<Selected>>,
    mut text: Query<&mut Text, With<DescriptionText>>,
) {
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    let blurb = if let Some(kind) = placement.0 {
        format!("Placing {}: {}", kind.name(), kind.description())
    } else if let Some((b, _)) = hovered
        .iter()
        .find(|(_, i)| **i == Interaction::Hovered || **i == Interaction::Pressed)
    {
        format!("{}: {}", b.0.name(), b.0.description())
    } else {
        let mut it = selected_units.iter();
        match (it.next(), it.next()) {
            (Some(u), None) => format!("{}: {}", u.kind.name(), u.kind.description()),
            _ => "Hover a build option or select a unit to see its strengths and weaknesses."
                .to_string(),
        }
    };
    text.0 = blurb;
}

fn update_queue_text(
    selected: Query<&ProductionQueue, With<Selected>>,
    mut text: Query<&mut Text, With<MenuQueueText>>,
) {
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    let status = match selected.iter().next() {
        Some(q) => {
            if let Some(front) = q.front() {
                let pct = (q.fraction() * 100.0) as i32;
                let extra = q.items.len().saturating_sub(1);
                if extra > 0 {
                    format!("{} {}%  (+{})", front.short(), pct, extra)
                } else {
                    format!("{} {}%", front.short(), pct)
                }
            } else {
                "idle".to_string()
            }
        }
        None => "—".to_string(),
    };
    text.0 = status;
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
