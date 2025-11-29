fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(MAX_WIDTH as u32, MAX_HEIGHT as u32),
            title: "Tank Game".into(),
            ..default()
        }),
        ..default()
    }))
    .init_state::<AppState>()
    .insert_resource(UnitIdCounter(1, 100000))
    .insert_resource(MainAssetInfoResource::new())
    .insert_resource(MissionInfoResource::new())
    .init_resource::<SimpleState>()
    .init_asset::<SimpleText>()
    .init_asset_loader::<SimpleTextAssetLoader>()
    .add_systems(PreStartup, setup_simple)
    .add_systems(
        Update,
        (sys_setup_main_assets, sys_setup_mission)
            .chain()
            .run_if(in_state(AppState::Loading)),
    )
    // .add_systems(PreStartup, (setup_main_assets, setup_mission).chain())
    .add_plugins((
        ExplosionPlugin,
        // ShapePlugin removed - bevy_prototype_lyon has lyon compatibility issues
        DebugPlugin,
        CursorPlugin,
        TankPlugin,
        UnitSelectionPlugin,
        MenuPlugin,
        HarvesterPlugin,
        BuildingPlugin,
        MonitoringPlugin,
    ))
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
    .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin::default())
    .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default());

    app.run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    PreparingUsingDynamicAssets,
    Playing,
}

#[derive(Resource, Default)]
struct SimpleState {
    simple_text: Handle<SimpleText>,
    simple_text2: Handle<SimpleText>,
    ready: bool,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
struct SimpleText {
    content: String,
}

#[derive(Default)]
struct SimpleTextAssetLoader;

/// Possible errors that can be produced by [`SimpleTextAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SimpleTextAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// A FromUtf8Error Error
    #[error("Could not parse UTF8: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

impl AssetLoader for SimpleTextAssetLoader {
    type Asset = SimpleText;
    type Settings = ();
    type Error = SimpleTextAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let content = String::from_utf8(bytes)?;

        Ok(SimpleText { content })
    }

    fn extensions(&self) -> &[&str] {
        &["tsj", "tmj"]
    }
}

fn setup_simple(
    mut state: ResMut<SimpleState>,
    asset_server: Res<AssetServer>,
) {
    state.simple_text = asset_server.load("main_assets.tsj");
    state.simple_text2 = asset_server.load("mission01.tmj");
}

pub mod actions;
mod components;
pub mod constants;
pub mod features;
pub mod resources;
pub mod systems;
pub mod types;
pub mod utils;

use crate::constants::{MAX_HEIGHT, MAX_WIDTH};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use thiserror::Error;

use crate::features::building::building_plugin::BuildingPlugin;
use crate::features::con_menu::MenuPlugin;
use crate::features::cursor::CursorPlugin;
use crate::features::debug::debug_plugin::DebugPlugin;
use crate::features::explosion::ExplosionPlugin;
use crate::features::harvester::HarvesterPlugin;
use crate::features::monitoring::MonitoringPlugin;
use crate::features::tank::TankPlugin;
use crate::features::unit::UnitIdCounter;
use crate::features::unit::UnitSelectionPlugin;
use crate::resources::mission_info_resource::MissionInfoResource;
use crate::systems::sys_setup_main_assets;
use crate::systems::sys_setup_mission;
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use crate::types::player::Player;
