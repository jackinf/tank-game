use crate::constants::{TileCoord, TILE_SIZE};
use crate::features::building::actions::spawn_building;
use crate::types::buildings_layer::BuildingsLayer;
use bevy::prelude::{AssetServer, Commands, Res, Vec2};

pub fn spawn_buildings(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    layer: BuildingsLayer,
    calculate_world_position: fn(&TileCoord) -> Vec2,
) {
    layer
        .enumerate()
        .into_iter()
        .for_each(|(coord, building_tile)| {
            let pos = calculate_world_position(&coord);

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
