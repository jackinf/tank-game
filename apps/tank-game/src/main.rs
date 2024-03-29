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
            // LoggerPlugin,
            CursorPlugin,
            TankMovementPlugin,
            TankInflationPlugin,
        ))
        .run()
}

mod common {
    pub mod constants;
    pub mod resources;
    pub mod tile;
}
mod game_setup;
mod cursor {
    pub mod cursor_coordinates;
    pub mod cursor_plugin;
}
mod logger {
    pub mod logger_plugin;
}
mod tank {
    pub mod tank;
    pub mod tank_gun;
    pub mod tank_id;
    pub mod tank_inflation_plugin;
    pub mod tank_movement_plugin;
}

use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::common::constants::{MAX_HEIGHT, MAX_WIDTH};
use crate::cursor::cursor_plugin::CursorPlugin;
use crate::logger::logger_plugin::LoggerPlugin;
use crate::tank::tank_inflation_plugin::TankInflationPlugin;
use crate::tank::tank_movement_plugin::TankMovementPlugin;
