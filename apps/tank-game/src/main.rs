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
        .insert_resource(Me::new(Player::P1))
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

pub mod common {
    pub mod resources {
        pub mod game_map;
        pub mod me;
        pub mod unit_id_counter;
    }
    pub mod components {
        pub mod tile;
        pub mod unit_id;
    }
    pub mod managers {
        pub mod tile_manager;
    }
    pub mod constants;
    pub mod tile_queries;
    pub mod unit_selection_plugin;
    pub mod utils {
        pub mod astar;
        pub mod enum_helpers;
        pub mod file_helpers;
    }
}

pub mod setup;
pub mod cursor {
    pub mod resources {
        pub mod cursor_coordinates;
    }
    pub mod cursor_plugin;
}
mod debug {
    mod resources {
        pub mod tank_log_timer;
    }
    pub mod debug_plugin;
}

pub mod tank {
    pub mod managers {
        pub mod tank_health_manager;
        pub mod tank_movement_manager;
        pub mod tank_spawn_manager;
    }
    pub mod components {
        pub mod tank;
        pub mod tank_gun;
        pub mod tank_health;
    }
    pub mod tank_plugin;
    pub mod tank_queries;
}

pub mod menu {
    mod components {
        pub mod money_text;
    }
    pub mod resources {
        pub mod menu_info;
    }
    pub mod menu_plugin;
}

use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::common::constants::{Player, MAX_HEIGHT, MAX_WIDTH};
use crate::common::resources::game_map::GameMap;
use crate::common::resources::me::Me;
use crate::common::resources::unit_id_counter::UnitIdCounter;
use crate::common::unit_selection_plugin::UnitSelectionPlugin;
use crate::cursor::cursor_plugin::CursorPlugin;
use crate::debug::debug_plugin::DebugPlugin;
use crate::menu::menu_plugin::MenuPlugin;
use crate::setup::setup;
use crate::tank::tank_plugin::TankPlugin;
