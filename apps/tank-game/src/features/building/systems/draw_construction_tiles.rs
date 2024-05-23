use crate::constants::TILE_SIZE;
use crate::features::building::actions::spawn_building;
use crate::features::building::components::BuildingPlacementTiles;
use crate::features::cursor::CursorCoordinates;
use crate::features::tile::{find_accessible_tile, find_tile, Tile};
use crate::utils::logger::Logger;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    AssetServer, Commands, EventReader, MouseButton, Query, Res, Sprite, Transform, Vec2, Vec3,
    With,
};

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
) {
    match (
        q_placement.single_mut(),
        find_accessible_tile(&q_tiles, &cursor.get_world()),
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
                            let tile = find_tile(&q_tiles, map_coord);
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
                    spawn_building(
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
