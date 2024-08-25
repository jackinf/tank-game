mod models;

use crate::models::{TiledMainAssets, TiledMission};
use crate::AppState::Playing;
use bevy::app::App;
use bevy::asset::io::Reader;
use bevy::asset::{
    AssetLoader, AssetServer, Assets, AsyncReadExt, BoxedFuture, Handle, LoadContext,
};
use bevy::prelude::{
    default, in_state, AssetApp, CursorIcon, IntoSystemConfigs, PluginGroup, PreStartup, Res,
    ResMut, Resource, States, Update, Window, WindowPlugin,
};
use bevy::utils::thiserror::Error;
use bevy::window::{Cursor, WindowResolution};
use bevy::DefaultPlugins;

pub const MAX_WIDTH: f32 = 1600.;
pub const MAX_HEIGHT: f32 = 1000.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(MAX_WIDTH, MAX_HEIGHT),
            title: "Tank Game".into(),
            cursor: Cursor {
                icon: CursorIcon::Default,
                ..default()
            },
            ..default()
        }),
        ..default()
    }))
    .init_state::<AppState>()
    .init_resource::<TiledState>()
    .init_asset::<TiledMainAssets>()
    .init_asset::<TiledMission>()
    .init_asset_loader::<TiledMainAssetLoader>()
    .init_asset_loader::<TiledMissionAssetLoader>()
    .add_systems(PreStartup, setup_load_tile)
    .add_systems(Update, sys_show_level.run_if(in_state(Playing)));

    app.run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    LoadingTiledMainAssets,
    LoadingTiledMission,
    #[default]
    Playing,
}

#[derive(Resource, Default)]
struct TiledState {
    main_assets: Handle<TiledMainAssets>,
    mission_01: Handle<TiledMission>,
    ready: bool,
}

#[derive(Default)]
struct TiledMainAssetLoader;
#[derive(Default)]
struct TiledMissionAssetLoader;

/// Possible errors that can be produced by [`TiledAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TiledAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// A FromUtf8Error Error
    #[error("Could not parse UTF8: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    /// A SerdeJsonError Error
    #[error("Could not parse JSON: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl AssetLoader for TiledMainAssetLoader {
    type Asset = TiledMainAssets;
    type Settings = ();
    type Error = TiledAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let content = String::from_utf8(bytes)?;
            let payload: TiledMainAssets = serde_json::from_str(&content)?;

            Ok(payload)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tsj"]
    }
}

impl AssetLoader for TiledMissionAssetLoader {
    type Asset = TiledMission;
    type Settings = ();
    type Error = TiledAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let content = String::from_utf8(bytes)?;
            let payload: TiledMission = serde_json::from_str(&content)?;

            Ok(payload)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmj"]
    }
}

pub fn setup_load_tile(mut state: ResMut<TiledState>, asset_server: Res<AssetServer>) {
    state.main_assets = asset_server.load("main_assets.tsj");
    state.mission_01 = asset_server.load("mission01.tmj");
    state.ready = true;
}

pub fn sys_show_level(
    tiled_state: Res<TiledState>,
    tiled_main_assets: Res<Assets<TiledMainAssets>>,
    tiled_mission_assets: Res<Assets<TiledMission>>,
) {
    let main_assets = tiled_main_assets.get(&tiled_state.main_assets).unwrap();
    let mission_assets = tiled_mission_assets.get(&tiled_state.mission_01).unwrap();

    println!("{:?}", main_assets);
    println!("{:?}", mission_assets);
}
