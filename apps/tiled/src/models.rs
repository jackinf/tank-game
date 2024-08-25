use bevy::prelude::{Asset, TypePath};
use serde::{Deserialize, Serialize};

/// Main assets

#[derive(Debug, Serialize, Deserialize)]
pub struct TiledMainAssetsGrid {
    height: u32,
    orientation: String,
    width: u32,
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
}

/// Mission layout

#[derive(Debug, Serialize, Deserialize)]
struct TiledMissionTileSet {
    #[serde(rename = "firstgid")]
    firstgid: u32,
    source: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TiledMissionLayer {
    data: Vec<u32>,
    height: u32,
    id: u32,
    name: String,
    opacity: f32,
    #[serde(rename = "type")]
    layer_type: String,
    visible: bool,
    width: u32,
    x: u32,
    y: u32,
}

#[derive(Debug, Serialize, Deserialize, Asset, TypePath)]
pub struct TiledMission {
    #[serde(rename = "compressionlevel")]
    compression_level: i32,
    height: u32,
    infinite: bool,
    layers: Vec<TiledMissionLayer>,
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
    #[serde(rename = "type")]
    map_type: String,
    version: String,
    width: u32,
}
