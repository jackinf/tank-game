//! Simplified Tiled2D JSON types.
//!
//! Provides direct deserialization from Tiled JSON files (.tsj and .tmj)
//! following the cleaner pattern from apps/tiled.
//!
//! Usage:
//! ```ignore
//! let main_assets: TiledTileset = serde_json::from_str(&content)?;
//! let mission: TiledMap = serde_json::from_str(&content)?;
//! ```

use bevy::prelude::{Asset, TypePath};
use serde::{Deserialize, Serialize};

/// A property on a tile (e.g., "type", "subtype")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledTileProperty {
    pub name: String,
    #[serde(rename = "type")]
    pub property_type: Option<String>,
    pub value: String,
}

/// A single tile definition in a tileset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledTile {
    pub id: u32,
    pub image: String,
    #[serde(rename = "imageheight")]
    pub image_height: u32,
    #[serde(rename = "imagewidth")]
    pub image_width: u32,
    #[serde(default)]
    pub properties: Vec<TiledTileProperty>,
}

impl TiledTile {
    /// Get a property value by name
    pub fn get_property(&self, name: &str) -> Option<&str> {
        self.properties
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.value.as_str())
    }

    /// Get the "type" property
    pub fn tile_type(&self) -> Option<&str> {
        self.get_property("type")
    }

    /// Get the "subtype" property
    pub fn tile_subtype(&self) -> Option<&str> {
        self.get_property("subtype")
    }
}

/// Grid configuration in a tileset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledGrid {
    pub height: u32,
    pub orientation: String,
    pub width: u32,
}

/// A tileset file (.tsj) - contains tile definitions
#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct TiledTileset {
    pub columns: u32,
    #[serde(default)]
    pub grid: Option<TiledGrid>,
    #[serde(default)]
    pub margin: u32,
    pub name: String,
    #[serde(default)]
    pub spacing: u32,
    #[serde(rename = "tilecount")]
    pub tile_count: u32,
    #[serde(rename = "tiledversion")]
    pub tiled_version: String,
    #[serde(rename = "tileheight")]
    pub tile_height: u32,
    #[serde(rename = "tilewidth")]
    pub tile_width: u32,
    pub tiles: Vec<TiledTile>,
}

impl TiledTileset {
    /// Find a tile by its ID
    pub fn get_tile(&self, id: u32) -> Option<&TiledTile> {
        self.tiles.iter().find(|t| t.id == id)
    }

    /// Find a tile by its ID (adjusted for Tiled's 1-based indexing in maps)
    pub fn get_tile_from_map_id(&self, map_id: u32) -> Option<&TiledTile> {
        if map_id == 0 {
            None
        } else {
            self.get_tile(map_id - 1)
        }
    }
}

/// Reference to a tileset in a map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledTilesetRef {
    #[serde(rename = "firstgid")]
    pub first_gid: u32,
    pub source: String,
}

/// A layer in a map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledLayer {
    /// Tile data (flat array, row-major order)
    pub data: Vec<u32>,
    pub height: u32,
    pub id: u32,
    pub name: String,
    pub opacity: f32,
    #[serde(rename = "type")]
    pub layer_type: Option<String>,
    pub visible: bool,
    pub width: u32,
    pub x: u32,
    pub y: u32,
}

impl TiledLayer {
    /// Get the tile ID at a specific coordinate
    pub fn get_tile_at(&self, x: usize, y: usize) -> Option<u32> {
        if x >= self.width as usize || y >= self.height as usize {
            return None;
        }
        let index = y * self.width as usize + x;
        self.data.get(index).copied()
    }

    /// Iterate over all tiles with their coordinates
    pub fn iter_tiles(&self) -> impl Iterator<Item = (usize, usize, u32)> + '_ {
        self.data.iter().enumerate().map(move |(index, &tile_id)| {
            let x = index % self.width as usize;
            let y = index / self.width as usize;
            (x, y, tile_id)
        })
    }

    /// Iterate over non-empty tiles with their coordinates
    pub fn iter_non_empty_tiles(&self) -> impl Iterator<Item = (usize, usize, u32)> + '_ {
        self.iter_tiles().filter(|(_, _, tile_id)| *tile_id > 0)
    }
}

/// A map file (.tmj) - contains layers and references to tilesets
#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct TiledMap {
    #[serde(rename = "compressionlevel")]
    pub compression_level: i32,
    pub height: u32,
    pub infinite: bool,
    pub layers: Vec<TiledLayer>,
    #[serde(rename = "nextlayerid")]
    pub next_layer_id: u32,
    #[serde(rename = "nextobjectid")]
    pub next_object_id: u32,
    pub orientation: String,
    #[serde(rename = "renderorder")]
    pub render_order: String,
    #[serde(rename = "tiledversion")]
    pub tiled_version: String,
    #[serde(rename = "tileheight")]
    pub tile_height: u32,
    #[serde(rename = "tilewidth")]
    pub tile_width: u32,
    #[serde(rename = "tilesets")]
    pub tilesets: Vec<TiledTilesetRef>,
    pub version: String,
    pub width: u32,
}

impl TiledMap {
    /// Find a layer by name
    pub fn get_layer(&self, name: &str) -> Option<&TiledLayer> {
        self.layers.iter().find(|l| l.name == name)
    }

    /// Get all layer names
    pub fn layer_names(&self) -> Vec<&str> {
        self.layers.iter().map(|l| l.name.as_str()).collect()
    }

    /// Get map size as (width, height) tuple
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tileset_deserialization() {
        let json = r#"{
            "columns": 0,
            "name": "test",
            "tilecount": 1,
            "tiledversion": "1.10.0",
            "tileheight": 64,
            "tilewidth": 64,
            "tiles": [{
                "id": 0,
                "image": "grass.png",
                "imageheight": 64,
                "imagewidth": 64,
                "properties": [
                    {"name": "type", "type": "string", "value": "ground"},
                    {"name": "subtype", "type": "string", "value": "grass"}
                ]
            }]
        }"#;

        let tileset: TiledTileset = serde_json::from_str(json).unwrap();
        assert_eq!(tileset.tiles.len(), 1);
        assert_eq!(tileset.tiles[0].tile_type(), Some("ground"));
        assert_eq!(tileset.tiles[0].tile_subtype(), Some("grass"));
    }

    #[test]
    fn test_map_layer_iteration() {
        let layer = TiledLayer {
            data: vec![0, 1, 2, 0, 3, 0],
            height: 2,
            id: 1,
            name: "test".to_string(),
            opacity: 1.0,
            layer_type: Some("tilelayer".to_string()),
            visible: true,
            width: 3,
            x: 0,
            y: 0,
        };

        let non_empty: Vec<_> = layer.iter_non_empty_tiles().collect();
        assert_eq!(non_empty.len(), 3);
        assert_eq!(non_empty[0], (1, 0, 1)); // x=1, y=0, tile_id=1
    }
}

