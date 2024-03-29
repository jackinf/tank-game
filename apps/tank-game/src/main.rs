fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(MAX_WIDTH as f32, MAX_HEIGHT as f32)
                        .with_scale_factor_override(1.0),
                    title: "Tank Game".into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins((
            LoggerPlugin,
            CursorPlugin,
            TankMovementPlugin,
            TankInflationPlugin,
        ))
        .run()
}

mod components;
mod common {
    pub mod constants;
    pub mod resources;
}
mod game_setup;
mod plugins {
    pub mod cursor_plugin;
    pub mod logger_plugin;
    pub mod tank_inflation_plugin;
    pub mod tank_movement_plugin;
}

use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::common::constants::{MAX_HEIGHT, MAX_WIDTH};
use crate::plugins::cursor_plugin::CursorPlugin;
use crate::plugins::logger_plugin::LoggerPlugin;
use crate::plugins::tank_inflation_plugin::TankInflationPlugin;
use crate::plugins::tank_movement_plugin::TankMovementPlugin;
