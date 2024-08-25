mod models;

use bevy::app::App;
use bevy::asset::{AssetLoader, AssetServer, Assets, AsyncReadExt, BoxedFuture, Handle, LoadContext};
use bevy::asset::io::Reader;
use bevy::DefaultPlugins;
use bevy::prelude::{default, in_state, AssetApp, Commands, CursorIcon, IntoSystemConfigs, NextState, PluginGroup, PreStartup, Res, ResMut, Resource, States, Update, Window, WindowPlugin};
use bevy::reflect::erased_serde::__private::serde;
use bevy::utils::thiserror::Error;
use bevy::window::{Cursor, WindowResolution};
use crate::AppState::{LoadingTiledMission, Playing};
use crate::models::{TiledMainAssets, TiledMission};

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
        .init_asset::<TiledContent>()
        .init_asset_loader::<TiledAssetLoader>()
        .add_systems(PreStartup, setup_load_tile)
        .add_systems(
            Update, sys_setup_main_assets.run_if(in_state(AppState::LoadingTiledMainAssets)),
        )
        .add_systems(
            Update, sys_setup_main_mission.run_if(in_state(AppState::LoadingTiledMission)),
        )
        .add_systems(Update, sys_show_level.run_if(in_state(Playing)));

    app.run();
}

pub fn sys_setup_main_assets(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    mut tiled_state: ResMut<TiledState>,
    custom_assets: Res<Assets<TiledContent>>,
) {
    if custom_assets.get(&tiled_state.main_assets).is_none() {
        return;
    }

    let custom_asset = custom_assets.get(&tiled_state.main_assets).unwrap();
    let content = custom_asset.content.clone();

    let payload: TiledMainAssets = serde_json::from_str(&content).unwrap();

    commands.insert_resource(payload);

    state.set(LoadingTiledMission);
}

pub fn sys_setup_main_mission(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    mut tiled_state: ResMut<TiledState>,
    custom_assets: Res<Assets<TiledContent>>
) {
    if custom_assets.get(&tiled_state.mission_01).is_none() {
        return;
    }

    let custom_asset = custom_assets.get(&tiled_state.mission_01).unwrap();
    let content = custom_asset.content.clone();

    let payload: TiledMission = serde_json::from_str(&content).unwrap();

    commands.insert_resource(payload);

    state.set(Playing);
}

pub fn sys_show_level(
    main_assets: Res<TiledMainAssets>,
    mission: Res<TiledMission>,
) {
    println!("{:?}", main_assets);
    println!("{:?}", mission);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    LoadingTiledMainAssets,
    LoadingTiledMission,
    Playing,
}

#[derive(Resource, Default)]
struct TiledState {
    main_assets: Handle<TiledContent>,
    mission_01: Handle<TiledContent>,
    ready: bool,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
struct TiledContent {
    content: String,
}

#[derive(Default)]
struct TiledAssetLoader;

/// Possible errors that can be produced by [`TiledAssetLoader`]
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

impl AssetLoader for TiledAssetLoader {
    type Asset = TiledContent;
    type Settings = ();
    type Error = SimpleTextAssetLoaderError;
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

            Ok(TiledContent { content })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tsj", "tmj"]
    }
}

pub fn setup_load_tile(
    mut state: ResMut<TiledState>,
    asset_server: Res<AssetServer>,
) {
    state.main_assets = asset_server.load("main_assets.tsj");
    state.mission_01 = asset_server.load("mission01.tmj");
}

