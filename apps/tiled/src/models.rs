use bevy::prelude::{Asset, TypePath};
use serde::{Deserialize, Serialize};

/// Main assets

#[derive(Debug, Serialize, Deserialize)]
pub struct TiledMainAssetsGrid {
    height: u32,
    orientation: String,
    width: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TiledMainAssetsTileProperty {
    name: String,
    // #[serde(rename = "type")]
    // property_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TiledMainAssetsTile {
    pub id: u32,
    pub image: String,
    #[serde(rename = "imageheight")]
    image_height: u32,
    #[serde(rename = "imagewidth")]
    image_width: u32,
    // #[serde(rename = "type")]
    // tile_type: String,
    properties: Vec<TiledMainAssetsTileProperty>,
}

#[derive(Debug, Serialize, Deserialize, Asset, TypePath)]
pub struct TiledMainAssets {
    columns: u32,
    grid: TiledMainAssetsGrid,
    margin: u32,
    name: String,
    spacing: u32,

    #[serde(rename = "tilecount")]
    tile_count: u32,
    #[serde(rename = "tiledversion")]
    tiled_version: String,
    #[serde(rename = "tileheight")]
    tile_height: u32,

    pub tiles: Vec<TiledMainAssetsTile>,
}

/// Mission layout

#[derive(Debug, Serialize, Deserialize)]
struct TiledMissionTileSet {
    #[serde(rename = "firstgid")]
    firstgid: u32,
    source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TiledMissionLayer {
    pub data: Vec<u32>,
    pub height: u32,
    pub id: u32,
    name: String,
    opacity: f32,
    // #[serde(rename = "type")]
    // layer_type: String,
    visible: bool,
    pub width: u32,
    x: u32,
    y: u32,
}

#[derive(Debug, Serialize, Deserialize, Asset, TypePath)]
pub struct TiledMission {
    #[serde(rename = "compressionlevel")]
    compression_level: i32,
    height: u32,
    infinite: bool,
    pub layers: Vec<TiledMissionLayer>,
    #[serde(rename = "nextlayerid")]
    next_layer_id: u32,
    #[serde(rename = "nextobjectid")]
    next_object_id: u32,
    orientation: String,
    #[serde(rename = "renderorder")]
    render_order: String,
    #[serde(rename = "tiledversion")]
    tiled_version: String,
    #[serde(rename = "tileheight")]
    tile_height: u32,
    #[serde(rename = "tilesets")]
    tile_sets: Vec<TiledMissionTileSet>,
    #[serde(rename = "tilewidth")]
    tile_width: u32,
    // #[serde(rename = "type")]
    // map_type: String,
    version: String,
    width: u32,
}
