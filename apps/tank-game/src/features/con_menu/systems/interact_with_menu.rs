use crate::features::building::components::BuildingPlacementTiles;
use crate::features::building::types::BuildingTileType;
use crate::features::con_menu::components::MenuCellInfo;
use crate::features::con_menu::resources::BuildingProgressInfo;
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use bevy::prelude::{
    BackgroundColor, Button, Changed, Color, Commands, Entity, Interaction, NextState, Query, Res,
    ResMut, With,
};

pub fn interact_with_menu(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &MenuCellInfo,
            &mut BuildingProgressInfo,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    main_asset_info_resource: Res<MainAssetInfoResource>,
    mut q_placement_building: Query<&mut BuildingPlacementTiles>,
) {
    for (interaction, mut color, cell_info, mut progress_info) in &mut interaction_query {
        let price = cell_info.price().unwrap_or(0);
        match *interaction {
            Interaction::Pressed => {
                if progress_info.is_idle() {
                    println!("INFO: {:?}", cell_info);
                    color.0 = Color::DARK_GREEN;

                    let building_tile_type = BuildingTileType::from_con_menu_building_type(
                        &cell_info
                            .get_building_type()
                            .expect("Building type not found"),
                    );
                    let building_tiles = main_asset_info_resource.get_building_tiles();
                    let building_tile = building_tiles
                        .get(&building_tile_type)
                        .expect("Building tile not found")
                        .clone();
                    progress_info.start_from_price(price, building_tile);
                }

                if progress_info.is_building() {
                    continue;
                }

                if progress_info.is_placing() {
                    let building_tile_type = BuildingTileType::from_con_menu_building_type(
                        &cell_info
                            .get_building_type()
                            .expect("Building type not found"),
                    );
                    let mut placement_building = q_placement_building.single_mut();
                    let building_tiles = main_asset_info_resource.get_building_tiles();
                    let building_tile = building_tiles
                        .get(&building_tile_type)
                        .expect("Building tile not found")
                        .clone();
                    placement_building.set_ready(Some(building_tile));
                    continue;
                }
            }
            Interaction::Hovered => {
                if progress_info.is_idle() {
                    color.0 = Color::YELLOW;
                } else if progress_info.is_placing() {
                    color.0 = Color::GREEN;
                }
            }
            Interaction::None => {
                if progress_info.is_building() {
                    color.0 = Color::GRAY;
                } else if progress_info.is_placing() {
                    color.0 = Color::GREEN;
                } else if progress_info.is_idle() {
                    color.0 = Color::WHITE;
                } else {
                    color.0 = Color::WHITE;
                }
            }
        }
    }
}
