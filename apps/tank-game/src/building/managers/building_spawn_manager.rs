use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, AssetServer, Commands, Res, Sprite, Transform};
use bevy::sprite::{Anchor, SpriteBundle};

use crate::building::building_type::BuildingType;
use crate::building::components::building::Building;
use crate::common::constants::{RawGrid, SPRITE_SCALE, TILE_SIZE};
use crate::common::player::Player;

pub struct BuildingSpawnManager;

impl BuildingSpawnManager {
    pub fn spawn_buildings(
        mut commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        all_building_maps: Vec<(RawGrid, Player)>,
        calculate_world_position: fn(usize, usize) -> Vec2,
    ) {
        all_building_maps
            .into_iter()
            .for_each(|(unit_map, player)| {
                unit_map
                    .iter()
                    .enumerate()
                    .for_each(|(row_index, row_on_row)| {
                        row_on_row.iter().enumerate().for_each(|(col_index, cell)| {
                            let pos = calculate_world_position(row_index, col_index);
                            let map_coord = (row_index, col_index);

                            if let Ok(building_type) = BuildingType::try_from(*cell) {
                                BuildingSpawnManager::spawn_single(
                                    &mut commands,
                                    &asset_server,
                                    // I'm not sure why I need this hack but the building is not placed correctly
                                    Vec2::new(pos.x - TILE_SIZE / 2.0, pos.y + TILE_SIZE / 2.0),
                                    building_type,
                                    map_coord,
                                    player.clone(),
                                );
                            }
                        });
                    });
            });
    }

    pub fn spawn_single(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        translation: Vec2,
        building_type: BuildingType,
        map_coord: (usize, usize),
        player: Player,
    ) {
        let sprite_path = building_type.get_building_type_sprite();
        let layer = building_type.get_building_type_layer();

        let color = match player {
            Player::P1 => crate::common::constants::P1_COLOR,
            Player::P2 => crate::common::constants::P2_COLOR,
        };
        let building = Building::new(building_type, map_coord, player);

        commands
            .spawn((SpriteBundle {
                transform: Transform::default()
                    .with_translation(translation.extend(layer))
                    .with_scale(Vec3::splat(SPRITE_SCALE)),
                texture: asset_server.load(sprite_path),
                sprite: Sprite {
                    color,
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },))
            .insert(building.clone());
    }
}
