use crate::constants::{TileCoord, TILE_SIZE};
use crate::features::building::actions::spawn_building;
use crate::features::building::components::{Building, GlobalBuildingPlacementTiles};
use crate::features::con_menu::BuildingConstructionProgressInfo;
use crate::features::cursor::CursorCoordinates;
use crate::features::tile::{find_accessible_tile, Tile};
use crate::features::unit::UnitIdCounter;
use bevy::color::palettes::css;
use bevy::color::Alpha;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    AssetServer, Color, Commands, EventReader, MouseButton, Mut, Query, Res, ResMut, Sprite,
    Transform, Vec2, Vec3, With,
};
use std::collections::HashSet;

pub fn sys_draw_construction_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_tiles: Query<&Tile>,
    q_buildings: Query<&Building>,
    mut q_placement: Query<
        (
            &mut Transform,
            &mut Sprite,
            &mut GlobalBuildingPlacementTiles,
        ),
        With<GlobalBuildingPlacementTiles>,
    >,
    cursor: Res<CursorCoordinates>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut unit_id_counter: ResMut<UnitIdCounter>,
    mut q_building_progress_info: Query<&mut BuildingConstructionProgressInfo>,
) {
    match (
        q_placement.single_mut().unwrap(),
        find_accessible_tile(&q_tiles, &cursor.get_world()),
    ) {
        ((mut transform, mut sprite, mut placement), Some(tile)) => {
            sprite.color = Color::NONE;

            if !placement.is_ready() {
                return;
            }

            let cursor_at = tile.get_tile_coord();
            let (world_x, world_y) = tile.get_world_coord();
            transform.translation = Vec3::new(world_x, world_y, transform.translation.z);

            // by default, assume that the placement is valid
            sprite.color = Color::from(css::GREEN).with_alpha(0.5);

            // check if the building is near the building
            if !is_placement_near_a_building(&q_buildings, &mut placement, &cursor_at) {
                sprite.color = Color::from(css::RED).with_alpha(0.5);
                return;
            }

            // check if placement overlaps with something else
            if does_placement_overlap_with_something_else(&q_buildings, &mut placement, &cursor_at)
            {
                sprite.color = Color::from(css::RED).with_alpha(0.5);
                return;
            }

            for mouse_button_event in mouse_button_events.read() {
                if mouse_button_event.button == MouseButton::Left
                    && mouse_button_event.state == ButtonState::Pressed
                {
                    // is ready check makes sure that there's a building type
                    let building_tile = placement.get_building_tile().unwrap().clone();
                    sprite.color = Color::NONE;
                    placement.set_ready(None);

                    // spawn a building
                    // Logger::log(&format!("Placed on tiles: {tile_x} and {tile_y}"));
                    spawn_building(
                        &mut commands,
                        &asset_server,
                        // TODO: why -TILE_SIZE & +TILE_SIZE?
                        Vec2::new(world_x - TILE_SIZE, world_y + TILE_SIZE),
                        building_tile.clone(),
                        cursor_at,
                        &mut unit_id_counter,
                    );

                    q_building_progress_info.iter_mut().for_each(|mut info| {
                        if let (Some(curr_tile)) = info.get_building_tile() {
                            if info.is_placing() && curr_tile == building_tile {
                                info.reset();
                            }
                        }
                    });
                }
            }
        }
        ((_, mut sprite, _), None) => {
            sprite.color = Color::NONE; // hide placement tile(s)
        }
    }
}

fn is_placement_near_a_building(
    q_buildings: &Query<&Building>,
    placement: &mut Mut<GlobalBuildingPlacementTiles>,
    cursor_at: &TileCoord,
) -> bool {
    let outer_tiles: HashSet<TileCoord> = q_buildings
        .iter()
        .map(|building| building.get_outer_tiles())
        .flatten()
        .collect();
    let placement_tiles = placement.get_placement_tiles(cursor_at);
    placement_tiles.intersection(&outer_tiles).count() > 0
}

fn does_placement_overlap_with_something_else(
    q_buildings: &Query<&Building>,
    placement: &mut Mut<GlobalBuildingPlacementTiles>,
    cursor_at: &TileCoord,
) -> bool {
    let building_tiles: HashSet<TileCoord> = q_buildings
        .iter()
        .map(|building| building.get_building_tiles())
        .flatten()
        .collect();
    let placement_tiles: HashSet<TileCoord> = placement.get_placement_tiles(cursor_at);
    placement_tiles.intersection(&building_tiles).count() > 0
}
