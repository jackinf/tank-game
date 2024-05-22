fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(MAX_WIDTH as f32, MAX_HEIGHT as f32),
            title: "Tank Game".into(),
            cursor: Cursor {
                icon: CursorIcon::Default,
                ..default()
            },
            ..default()
        }),
        ..default()
    }))
    .insert_resource(Msaa::Sample4)
    .insert_resource(UnitIdCounter(1))
    .insert_resource(Me::new(Player::P1))
    .insert_resource(GameMap::default())
    .insert_resource(MainAssetInfoResource::new())
    .insert_resource(MissionInfoResource::new())
    .add_systems(PreStartup, (setup_main_assets, setup_mission).chain())
    .add_plugins((
        ShapePlugin,
        DebugPlugin,
        CursorPlugin,
        TankPlugin,
        UnitSelectionPlugin,
        MenuPlugin,
        HarvesterPlugin,
        BuildingPlugin,
        MonitoringPlugin,
    ))
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
    .add_plugins(PerfUiPlugin);

    // use bevy::diagnostic::LogDiagnosticsPlugin;
    // app.add_plugins(LogDiagnosticsPlugin::default());

    app.run()
}

pub mod actions;
pub mod constants;
pub mod features;
pub mod resources;
pub mod systems;
pub mod types;
pub mod utils;

use crate::constants::{MAX_HEIGHT, MAX_WIDTH};
use bevy::prelude::*;
use bevy::window::{Cursor, WindowResolution};
use bevy_prototype_lyon::prelude::ShapePlugin;
use iyes_perf_ui::PerfUiPlugin;

use crate::features::building::building_plugin::BuildingPlugin;
use crate::features::con_menu::MenuPlugin;
use crate::features::cursor::CursorPlugin;
use crate::features::debug::debug_plugin::DebugPlugin;
use crate::features::harvester::HarvesterPlugin;
use crate::features::monitoring::monitoring_plugin::MonitoringPlugin;
use crate::features::tank::tank_plugin::TankPlugin;
use crate::features::unit::resources::unit_id_counter::UnitIdCounter;
use crate::features::unit::unit_selection_plugin::UnitSelectionPlugin;
use crate::resources::game_map::GameMap;
use crate::resources::me::Me;
use crate::resources::mission_info_resource::MissionInfoResource;
use crate::systems::setup_main_assets;
use crate::systems::setup_mission;
use crate::types::player::Player;
use types::main_asset_info_resource::MainAssetInfoResource;
