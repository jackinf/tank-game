//! HUD: top bar (credits / power), a selection-driven sidebar build menu that
//! shows the queue of whichever production building is selected, and the
//! game-over overlay.
//!
//! The marker components, small resources and panel colours shared across the
//! HUD live here; each submodule owns one slice of behaviour:
//!
//! - [`hud`]: the static layout (top bar, power gauge, sidebar) + credit text.
//! - [`build_menu`]: the per-building list of things to build.
//! - [`ready_menu`]: finished structures waiting to be placed.
//! - [`power_bar`]: the vertical power gauge fill.
//! - [`info_panel`]: the queue line and the strengths/weaknesses blurb.
//! - [`game_over`]: the victory/defeat overlay and restart input.

mod build_menu;
mod game_over;
mod hud;
mod info_panel;
mod power_bar;
mod ready_menu;

use crate::defs::{BuildingKind, Producible};
use crate::state::GameState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuState>()
            .init_resource::<ReadyShown>()
            .add_systems(Startup, hud::spawn_hud)
            .add_systems(
                Update,
                (
                    hud::update_hud_text,
                    power_bar::update_power_bar,
                    build_menu::sync_build_menu,
                    build_menu::update_button_states,
                    build_menu::handle_build_buttons,
                    ready_menu::sync_ready_menu,
                    ready_menu::handle_place_buttons,
                    info_panel::update_queue_text,
                    info_panel::update_description,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::GameOver), game_over::spawn_game_over)
            .add_systems(OnExit(GameState::GameOver), game_over::despawn_game_over)
            .add_systems(
                Update,
                game_over::restart_input.run_if(in_state(GameState::GameOver)),
            );
    }
}

// --- Shared marker components --------------------------------------------

#[derive(Component)]
pub(super) struct CreditsText;
#[derive(Component)]
pub(super) struct PowerText;
#[derive(Component)]
pub(super) struct BuildButton(pub Producible);
/// Container whose children are the (dynamic) build buttons.
#[derive(Component)]
pub(super) struct BuildMenuRoot;
#[derive(Component)]
pub(super) struct MenuHeaderText;
#[derive(Component)]
pub(super) struct MenuQueueText;
#[derive(Component)]
pub(super) struct GameOverUi;
/// A button that enters placement mode for a finished structure.
#[derive(Component)]
pub(super) struct PlaceButton(pub BuildingKind);
/// Container whose children are the (dynamic) "ready to place" buttons.
#[derive(Component)]
pub(super) struct ReadyMenuRoot;
/// The fill element of the vertical power gauge.
#[derive(Component)]
pub(super) struct PowerBarFill;
/// The strengths/weaknesses blurb shown at the bottom of the sidebar.
#[derive(Component)]
pub(super) struct DescriptionText;

// --- Shared resources ----------------------------------------------------

/// The set of ready-to-place buildings shown last frame, so we only rebuild the
/// buttons when it actually changes.
#[derive(Resource, Default)]
pub(super) struct ReadyShown(pub Vec<BuildingKind>);

/// Which production building's menu is currently displayed.
#[derive(Resource, Default)]
pub(super) struct MenuState {
    pub shown: Option<Entity>,
}

// --- Panel theme ---------------------------------------------------------

pub(super) const PANEL_BG: Color = Color::srgb(0.10, 0.11, 0.13);
pub(super) const BUTTON_BG: Color = Color::srgb(0.20, 0.22, 0.26);
pub(super) const BUTTON_DISABLED: Color = Color::srgb(0.14, 0.14, 0.16);
