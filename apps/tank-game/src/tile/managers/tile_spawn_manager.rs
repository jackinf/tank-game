use crate::common::constants::{RawGrid, TileCoord, TileGrid, WorldCoord};
use crate::preparation::load_mission::GroundLayer;
use crate::tile::managers::tile_manager::TileManager;
use bevy::prelude::{AssetServer, Commands, Res, Vec2};
use std::collections::HashMap;

#[derive(Debug)]
pub enum TileSpawnManagerErrors {
    TileSpawnError,
}

pub struct TileSpawnManager;

impl TileSpawnManager {
    pub fn spawn_tiles(
        mut commands: &mut Commands,
        assets: &Res<AssetServer>,
        ground_layer: &GroundLayer,
        // grid_to_tilemap: &mut HashMap<TileCoord, (f32, f32)>,
        calculate_world_position: fn(&TileCoord) -> Vec2,
    ) {
        ground_layer
            .get_tiles()
            .into_iter()
            .for_each(|(coord, ground)| {
                let pos = calculate_world_position(&coord);
                TileManager::spawn_tile(&mut commands, &assets, pos, ground.clone(), coord.clone());
            });
    }

    pub fn create_tile_to_world_coordinates(
        ground_layer: &GroundLayer,
        calculate_world_position: fn(&TileCoord) -> Vec2,
    ) -> HashMap<TileCoord, WorldCoord> {
        let mut tile_to_world_coordinates = HashMap::new();
        ground_layer.get_tiles().into_iter().for_each(|(coord, _)| {
            let pos = calculate_world_position(coord);
            tile_to_world_coordinates.insert(coord.clone(), (pos.x, pos.y));
        });
        tile_to_world_coordinates
    }
}
