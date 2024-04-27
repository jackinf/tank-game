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
        .insert_resource(Msaa::Sample4)
        .insert_resource(UnitIdCounter(1))
        .insert_resource(Me::new(Player::P1))
        .insert_resource(GameMap::default())
        .add_systems(PreStartup, setup)
        .add_plugins((
            ShapePlugin,
            DebugPlugin,
            CursorPlugin,
            TankPlugin,
            UnitSelectionPlugin,
            MenuPlugin,
            HarvesterPlugin,
        ))
        .add_systems(Update, CursorManager::convert_cursor_to_world_position)
        .run()
}

pub mod unit {
    pub mod components {
        pub mod unit_id;
    }
    pub mod managers {
        pub mod unit_spawn_manager;
    }
    pub mod resources {
        pub mod unit_id_counter;
    }
    pub mod unit_selection_plugin;
    pub mod unit_type;
}
pub mod tile {
    pub mod components {
        pub mod gold;
        pub mod tile;
    }
    pub mod managers {
        pub mod tile_manager;
        pub mod tile_spawn_manager;
    }
    pub mod tile_queries;
    pub mod tile_type;
}
pub mod common {
    pub mod resources {
        pub mod game_map;
        pub mod me;
    }
    pub mod managers {
        pub mod cursor_manager;
    }
    pub mod constants;
    pub mod player;
    pub mod utils {
        pub mod astar;
        pub mod common_helpers;
        pub mod file_helpers;
        pub mod logger;
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
        pub mod tank_shooting_manager;
        pub mod tank_spawn_manager;
    }
    pub mod components {
        pub mod tank;
        pub mod tank_bullet;
        pub mod tank_gun;
        pub mod tank_health;
    }
    pub mod resources {
        pub mod tank_monitoring_timer;
    }
    pub mod tank_plugin;
    pub mod tank_queries;
}

pub mod con_menu {
    mod components {
        pub mod money_text;
    }
    pub mod resources {
        pub mod menu_info;
    }
    pub mod menu_plugin;
}
pub mod building {
    pub mod components {
        pub mod building;
    }
    pub mod managers {
        pub mod building_spawn_manager;
    }
    pub mod building_type;
}
pub mod harvester {
    pub mod components {
        pub mod harvester;
    }
    pub mod resources {
        pub mod harvester_timer;
    }
    pub mod managers {
        pub mod harvester_spawn_manager;
        pub mod harvester_state_manager;
    }
    pub mod harvester_plugin;
}

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::common::constants::{MAX_HEIGHT, MAX_WIDTH};
use crate::common::managers::cursor_manager::CursorManager;
use crate::common::player::Player;
use crate::common::resources::game_map::GameMap;
use crate::common::resources::me::Me;

use crate::con_menu::menu_plugin::MenuPlugin;
use crate::cursor::cursor_plugin::CursorPlugin;
use crate::debug::debug_plugin::DebugPlugin;
use crate::harvester::harvester_plugin::HarvesterPlugin;
use crate::setup::setup;
use crate::tank::tank_plugin::TankPlugin;
use crate::unit::resources::unit_id_counter::UnitIdCounter;
use crate::unit::unit_selection_plugin::UnitSelectionPlugin;
