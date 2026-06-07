//! A Red Alert / Dune style real-time strategy game, built with Bevy and
//! rendered with simple shapes.

// The game keeps a deliberately broad helper/data API surface (stat lookups,
// faction colours, z-layers, etc.); not every entry is wired up yet.
#![allow(dead_code)]

mod ai;
mod camera;
mod combat;
mod components;
mod config;
mod cursor;
mod defs;
mod economy;
mod faction;
mod fx;
mod grid;
mod gridview;
mod harvester;
mod health;
mod maps;
mod movement;
mod production;
mod selection;
mod setup;
mod spawn;
mod state;
mod terrain;
#[cfg(test)]
mod tests;
mod ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::ai::AiPlugin;
use crate::camera::CameraPlugin;
use crate::combat::CombatPlugin;
use crate::config::*;
use crate::cursor::CursorPlugin;
use crate::economy::EconomyPlugin;
use crate::fx::FxPlugin;
use crate::gridview::GridViewPlugin;
use crate::harvester::HarvesterPlugin;
use crate::health::HealthPlugin;
use crate::movement::MovementPlugin;
use crate::production::ProductionPlugin;
use crate::selection::SelectionPlugin;
use crate::setup::SetupPlugin;
use crate::state::GameState;
use crate::terrain::TerrainPlugin;
use crate::ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            WINDOW_WIDTH as u32,
                            WINDOW_HEIGHT as u32,
                        ),
                        title: "Tank Game — Red Alert RTS".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.06, 0.07, 0.09)))
        .init_state::<GameState>()
        .add_plugins((
            CameraPlugin,
            CursorPlugin,
            TerrainPlugin,
            EconomyPlugin,
            MovementPlugin,
            CombatPlugin,
            HealthPlugin,
            HarvesterPlugin,
            ProductionPlugin,
            SelectionPlugin,
            AiPlugin,
            FxPlugin,
            UiPlugin,
            GridViewPlugin,
            SetupPlugin,
        ))
        .run();
}
