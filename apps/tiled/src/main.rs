mod models;

use crate::models::{TiledMainAssets, TiledMission};
use crate::AppState::{LoadingLevel, Playing};
use bevy::app::App;
use bevy::asset::io::Reader;
use bevy::asset::{
    AssetLoader, AssetServer, Assets, AsyncReadExt, BoxedFuture, Handle, LoadContext,
};
use bevy::prelude::{default, in_state, AssetApp, Camera2dBundle, Commands, CursorIcon, IntoSystemConfigs, NextState, PluginGroup, PreStartup, Res, ResMut, Resource, Sprite, SpriteBundle, States, Transform, Update, Vec3, Window, WindowPlugin};
use bevy::utils::thiserror::Error;
use bevy::window::{Cursor, WindowResolution};
use bevy::DefaultPlugins;
use bevy::math::Vec2;
use bevy::sprite::Anchor;

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
    .add_systems(Update, sys_draw_level.run_if(in_state(LoadingLevel)))
    .add_systems(Update, playing.run_if(in_state(Playing)));

    app.run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    LoadingLevel,
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

pub const TILE_SIZE: f32 = 64.0;
pub const SPRITE_SCALE: f32 = 0.5;

pub fn calculate_tile_to_world_position(xx: i32, yy: i32) -> Vec2 {
    let x = xx as f32 * TILE_SIZE;
    let y = yy as f32 * TILE_SIZE;
    Vec2::new(x, y)
}

pub fn sys_draw_level(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    tiled_state: Res<TiledState>,
    tiled_main_assets: Res<Assets<TiledMainAssets>>,
    tiled_mission_assets: Res<Assets<TiledMission>>,
    mut state: ResMut<NextState<AppState>>,
) {
    let main_assets = tiled_main_assets.get(&tiled_state.main_assets).unwrap();
    let mission_assets = tiled_mission_assets.get(&tiled_state.mission_01).unwrap();

    for layer in &mission_assets.layers {
        &layer.data.iter().enumerate().for_each(|(index, cell)| {
            if *cell <= 0 {
                return;
            }
            println!("{}: {}", index, cell);

            // find(x => x.id === * cell as usize - 1)
            let tile = &main_assets.tiles.iter().find(|x| x.id == *cell - 1).unwrap();
            let sprite_path = &tile.image;
            let xx = index as i32 % layer.width as i32;
            let yy = index as i32 / layer.width as i32;
            let translation = calculate_tile_to_world_position(xx, yy);

            commands.spawn((SpriteBundle {
                transform: Transform::default()
                    .with_translation(translation.extend(layer.id as f32))
                    .with_scale(Vec3::splat(SPRITE_SCALE)),
                texture: asset_server.load(sprite_path),
                sprite: Sprite {
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },));
        });
    }

    println!("{:?}", main_assets);
    println!("{:?}", mission_assets);

    // zoom out
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(MAX_WIDTH / 2., MAX_HEIGHT / 2., 1000.))
            .with_scale(Vec3::new(2., 2., 1.)),
        ..default()
    });

    state.set(AppState::Playing);
}

pub fn playing() {
    println!("Playing");
}