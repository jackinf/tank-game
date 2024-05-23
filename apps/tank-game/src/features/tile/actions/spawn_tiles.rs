use crate::constants::TileCoord;
use crate::features::tile::systems::spawn_tile;
use crate::features::tile::GroundLayer;
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Res};

pub fn spawn_tiles(
    mut commands: &mut Commands,
    assets: &Res<AssetServer>,
    ground_layer: &GroundLayer,
    calculate_world_position: fn(&TileCoord) -> Vec2,
) {
    ground_layer
        .get_tiles()
        .into_iter()
        .for_each(|(coord, ground)| {
            let pos = calculate_world_position(&coord);
            spawn_tile(&mut commands, &assets, pos, ground.clone(), coord.clone());
        });
}
