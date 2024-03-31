fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(MAX_WIDTH as f32, MAX_HEIGHT as f32),
                // .with_scale_factor_override(1.0),
                title: "Tank Game".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameMap(vec![], HashMap::new()))
        .add_plugins((
            // LoggerPlugin,
            // CameraPlugin,
            SetupPlugin,
            CursorPlugin,
            TankMovementPlugin,
            TankInflationPlugin,
            TankSelectionPlugin,
            UiMenuPlugin,
        ))
        .run()
}

mod common {
    pub mod constants;
    pub mod game_map;
    pub mod tile;
}
mod setup {
    pub mod setup_plugin;
    pub mod tank_id_counter;
}
mod cursor {
    pub mod cursor_coordinates;
    pub mod cursor_plugin;
}
mod logger {
    pub mod logger_plugin;
    pub mod tank_log_timer;
}
mod tank {
    pub mod tank;
    pub mod tank_gun;
    pub mod tank_id;
    pub mod tank_inflation_plugin;
    pub mod tank_movement_plugin;
    pub mod tank_selection_plugin;
}

mod ui_menu {
    pub mod ui_menu_plugin;
}
mod utils {
    pub mod astar;
}

use bevy::prelude::*;
use bevy::window::WindowResolution;
use std::collections::HashMap;

use crate::common::constants::{MAX_HEIGHT, MAX_WIDTH};
use crate::common::game_map::GameMap;
use crate::cursor::cursor_plugin::CursorPlugin;
use crate::setup::setup_plugin::SetupPlugin;
use crate::tank::tank_inflation_plugin::TankInflationPlugin;
use crate::tank::tank_movement_plugin::TankMovementPlugin;
use crate::tank::tank_selection_plugin::TankSelectionPlugin;
use crate::ui_menu::ui_menu_plugin::UiMenuPlugin;
