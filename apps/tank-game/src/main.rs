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
        .insert_resource(UnitIdCounter(1))
        .add_systems(PreStartup, setup)
        .insert_resource(GameMap::default())
        .add_plugins((
            DebugPlugin,
            CursorPlugin,
            TankPlugin,
            UnitSelectionPlugin,
            MenuPlugin,
        ))
        .run()
}

mod common {
    pub mod constants;
    pub mod game_map;
    pub mod tile;
    pub mod tile_queries;
    pub mod unit_id;
    pub mod unit_id_counter;
    pub mod unit_selection_plugin;
}

pub mod setup;
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
    pub mod tank_health_manager;
    pub mod tank_movement_manager;
    pub mod tank_plugin;
    pub mod tank_queries;
    pub mod tank_spawn_manager;
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
use crate::common::unit_selection_plugin::UnitSelectionPlugin;
use crate::cursor::cursor_plugin::CursorPlugin;
use crate::debug::debug_plugin::DebugPlugin;
use crate::menu::menu_plugin::MenuPlugin;
use crate::setup::setup;
use crate::tank::tank_plugin::TankPlugin;
use common::unit_id_counter::UnitIdCounter;
