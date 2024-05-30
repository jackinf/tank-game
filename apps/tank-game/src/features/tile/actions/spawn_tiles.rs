use crate::actions::calculate_tile_world_position::calculate_tile_to_world_position;
use crate::features::tile::systems::spawn_tile;
use crate::features::tile::GroundLayer;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};

pub fn spawn_tiles(
    mut commands: &mut Commands,
    assets: &Res<AssetServer>,
    ground_layer: &GroundLayer,
) {
    ground_layer
        .get_tiles()
        .into_iter()
        .for_each(|(coord, ground)| {
            let pos = calculate_tile_to_world_position(&coord);
            spawn_tile(&mut commands, &assets, pos, ground.clone(), coord.clone());
        });
}
