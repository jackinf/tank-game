use crate::actions::calculate_tile_world_position::calculate_tile_to_world_position;
use crate::constants::TILE_SIZE;
use crate::features::building::actions::spawn_building;
use crate::features::building::types::buildings_layer::BuildingsLayer;
use bevy::prelude::{AssetServer, Commands, Res, Vec2};

pub fn spawn_buildings(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    layer: BuildingsLayer,
) {
    layer
        .enumerate()
        .into_iter()
        .for_each(|(coord, building_tile)| {
            let pos = calculate_tile_to_world_position(&coord);

            spawn_building(
                &mut commands,
                &asset_server,
                // I'm not sure why I need this hack but the building is not placed correctly
                Vec2::new(
                    pos.x - TILE_SIZE / 2.0,
                    (pos.y + TILE_SIZE) + TILE_SIZE / 2.0,
                ),
                building_tile,
                coord,
            );
        });
}
