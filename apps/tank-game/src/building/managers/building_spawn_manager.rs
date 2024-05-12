use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, AssetServer, Commands, EventReader, MouseButton, Query, Res, Sprite, Transform, With,
};
use bevy::sprite::{Anchor, SpriteBundle};

use crate::building::building_tile::BuildingTile;
use crate::building::components::building::Building;
use crate::building::components::building_placement_tiles::BuildingPlacementTiles;
use crate::common::constants::{RawGrid, TileCoord, SPRITE_SCALE, TILE_SIZE};
use crate::common::player::Player;
use crate::common::resources::me::Me;
use crate::common::utils::logger::Logger;
use crate::cursor::resources::cursor_coordinates::CursorCoordinates;
use crate::preparation::load_mission::BuildingsLayer;
use crate::tile::components::tile::Tile;
use crate::tile::tile_queries::TileQueries;

pub struct BuildingSpawnManager;

impl BuildingSpawnManager {
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

                BuildingSpawnManager::spawn_single(
                    &mut commands,
                    &asset_server,
                    // I'm not sure why I need this hack but the building is not placed correctly
                    Vec2::new(pos.x - TILE_SIZE / 2.0, pos.y + TILE_SIZE / 2.0),
                    building_tile,
                    coord,
                );
            });
    }

    pub fn spawn_single(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        translation: Vec2,
        building_tile: BuildingTile,
        map_coord: (usize, usize),
    ) {
        let sprite_path = building_tile.get_image_path();
        let layer = building_tile.get_layer();

        let player = building_tile.get_player();
        let color = match player {
            Player::P1 => crate::common::constants::P1_COLOR,
            Player::P2 => crate::common::constants::P2_COLOR,
        };
        let building = Building::new(building_tile, map_coord, player);

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

    pub fn draw_construction_tiles(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        q_tiles: Query<&Tile>,
        mut q_placement: Query<
            (&mut Transform, &mut Sprite, &mut BuildingPlacementTiles),
            With<BuildingPlacementTiles>,
        >,
        cursor: Res<CursorCoordinates>,
        mut mouse_button_events: EventReader<MouseButtonInput>,
        res_me: Res<Me>,
    ) {
        match (
            q_placement.single_mut(),
            TileQueries::find_accessible_tile(&q_tiles, &cursor.get_world()),
        ) {
            ((mut transform, mut sprite, mut placement), Some(tile)) => {
                if !placement.is_ready() {
                    return;
                }
                sprite.color.set_a(0.5); // show tile
                let (world_x, world_y) = tile.get_world_coord();
                transform.translation = Vec3::new(world_x, world_y, transform.translation.z);

                for mouse_button_event in mouse_button_events.read() {
                    if mouse_button_event.button == MouseButton::Left
                        && mouse_button_event.state == ButtonState::Pressed
                    {
                        // validate if all tiles in layout.x * layout.y are accessible
                        // TODO: broken
                        let (tile_x, tile_y) = tile.get_tile_coord();
                        let (layout_x, layout_y) = placement.get_layout();
                        let mut all_accessible = true;
                        for i in 0..layout_x {
                            for j in 0..layout_y {
                                let map_coord = (tile_x + i, tile_y + j);
                                // TODO: tile might not have info on other objects placed there. Create a map of free & occupied cells
                                let tile = TileQueries::find_tile(&q_tiles, map_coord);
                                if tile.is_none() || !tile.unwrap().accessible() {
                                    all_accessible = false;
                                    break;
                                }
                            }
                        }

                        if !all_accessible {
                            continue;
                        }

                        // is ready check makes sure that there's a building type
                        let building_tile = placement.get_building_tile().unwrap().clone();
                        sprite.color.set_a(0.0);
                        placement.set_ready(None);

                        // spawn a building
                        Logger::log(&format!("Placed on tiles: {tile_x} and {tile_y}"));
                        BuildingSpawnManager::spawn_single(
                            &mut commands,
                            &asset_server,
                            // TODO: why -TILE_SIZE & +TILE_SIZE?
                            Vec2::new(world_x - TILE_SIZE, world_y + TILE_SIZE),
                            building_tile,
                            (tile_x, tile_y),
                        );
                    }
                }
            }
            ((_, mut sprite, _), None) => {
                sprite.color.set_a(0.0); // hide placement tile(s)
            }
        }
    }
}
