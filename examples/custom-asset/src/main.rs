use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
struct CustomAsset {
    #[allow(dead_code)]
    value: i32,
}

#[derive(Default)]
struct CustomAssetLoader;

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A FromUtf8Error
    #[error("Could not parse UTF8: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

impl AssetLoader for CustomAssetLoader {
    type Asset = CustomAsset;
    type Settings = ();
    type Error = CustomAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        // Simple parsing - assumes RON-like format
        let content = String::from_utf8(bytes)?;
        // For demo purposes, just return a default value
        Ok(CustomAsset { value: 42 })
    }

    fn extensions(&self) -> &[&str] {
        &["custom"]
    }
}

#[derive(Default)]
struct TsjAssetLoader;

/// Possible errors that can be produced by [`TsjAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TsjAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// A [Serde JSON](serde_json) Error
    #[error("Could not parse JSON: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl AssetLoader for TsjAssetLoader {
    type Asset = Tileset;
    type Settings = ();
    type Error = TsjAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        // deserialize into Tileset
        let tileset: Tileset = serde_json::from_slice(&bytes)?;

        Ok(tileset)
    }

    fn extensions(&self) -> &[&str] {
        &["tsj"]
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<State>()
        .init_asset::<Tileset>()
        .init_asset::<CustomAsset>()
        .init_asset_loader::<CustomAssetLoader>()
        .init_asset_loader::<TsjAssetLoader>()
        .add_systems(Startup, setup)
        // .add_systems(Update, print_on_load)
        .run();
}

#[derive(Resource, Default)]
struct State {
    handle: Handle<CustomAsset>,
    other_handle: Handle<CustomAsset>,
    main_assets: Handle<Tileset>,
    // text_handle: Handle<String>,
    // blob: Handle<Blob>,
    printed: bool,
}

fn setup(
    mut state: ResMut<State>,
    asset_server: Res<AssetServer>,
    main_assets: Res<Assets<Tileset>>,
) {
    // Recommended way to load an asset
    state.handle = asset_server.load("data/asset.custom");

    // File extensions are optional, but are recommended for project management and last-resort inference
    state.other_handle = asset_server.load("data/asset_no_extension");

    let main_assets_handler: Handle<Tileset> = asset_server.load("data/main_assets.tsj");
    let main_assets = main_assets.get(&main_assets_handler);

    if main_assets.is_none() {
        info!("Main Assets Not Ready");
        return;
    }
    info!("Main assets loaded: {:?}", main_assets.unwrap());
}

fn print_on_load(
    mut state: ResMut<State>,
    custom_assets: Res<Assets<CustomAsset>>,
    main_assets: Res<Assets<Tileset>>,
) {
    let custom_asset = custom_assets.get(&state.handle);
    let other_custom_asset = custom_assets.get(&state.other_handle);
    // let main_assets = main_assets.get(&state.main_assets);

    // Can't print results if the assets aren't ready
    if state.printed {
        return;
    }

    if custom_asset.is_none() {
        info!("Custom Asset Not Ready");
        return;
    }

    if other_custom_asset.is_none() {
        info!("Other Custom Asset Not Ready");
        return;
    }

    info!("Custom asset loaded: {:?}", custom_asset.unwrap());
    info!("Custom asset loaded: {:?}", other_custom_asset.unwrap());

    // Once printed, we won't print again
    state.printed = true;
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Tileset {
    pub columns: u32,
    pub grid: Grid,
    pub margin: u32,
    pub name: String,
    pub spacing: u32,
    pub tilecount: u32,
    pub tiledversion: String,
    pub tileheight: u32,
    pub tiles: Vec<Tile>,
    pub tilewidth: u32,
    #[serde(rename = "type")]
    pub tileset_type: String,
    pub version: String,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Grid {
    pub height: u32,
    pub orientation: String,
    pub width: u32,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Tile {
    pub id: u32,
    pub image: String,
    pub imageheight: u32,
    pub imagewidth: u32,
    pub properties: Vec<Property>,
    #[serde(rename = "type")]
    pub tile_type: Option<String>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Property {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub property_type: Option<String>,
    pub value: Option<String>,
}
