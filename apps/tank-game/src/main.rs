fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(MAX_WIDTH as f32, MAX_HEIGHT as f32),
                title: "Tank Game".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameMap::default())
        .add_plugins((
            DebugPlugin,
            SetupPlugin,
            CursorPlugin,
            TankMovementPlugin,
            TankSelectionPlugin,
            MenuPlugin,
        ))
        .run()
}

mod common {
    pub mod constants;
    pub mod game_map;
    pub mod tile;
    pub mod tile_queries;
}
mod setup {
    pub mod setup_plugin;
    pub mod tank_id_counter;
}
mod cursor {
    pub mod cursor_coordinates;
    pub mod cursor_plugin;
}
mod debug {
    pub mod debug_plugin;
    pub mod tank_log_timer;
}

mod tank {
    pub mod tank;
    pub mod tank_gun;
    pub mod tank_health;
    pub mod tank_id;
    pub mod tank_movement_plugin;
    pub mod tank_queries;
    pub mod tank_selection_plugin;
}

mod menu {
    pub mod menu_info;
    pub mod menu_plugin;
    pub mod money_text;
}

mod utils {
    pub mod astar;
}

use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::common::constants::{MAX_HEIGHT, MAX_WIDTH};
use crate::common::game_map::GameMap;
use crate::cursor::cursor_plugin::CursorPlugin;
use crate::debug::debug_plugin::DebugPlugin;
use crate::menu::menu_plugin::MenuPlugin;
use crate::setup::setup_plugin::SetupPlugin;
use crate::tank::tank_movement_plugin::TankMovementPlugin;
use crate::tank::tank_selection_plugin::TankSelectionPlugin;
