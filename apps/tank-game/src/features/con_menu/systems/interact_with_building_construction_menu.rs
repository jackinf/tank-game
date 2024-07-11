use crate::features::building::components::GlobalBuildingPlacementTiles;
use crate::features::building::types::BuildingTileType;
use crate::features::con_menu::components::BuildingTileTypeMenuCellInfo;
use crate::features::con_menu::resources::BuildingConstructionProgressInfo;
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use bevy::prelude::{
    BackgroundColor, Button, Changed, Color, Interaction, Query, Res, ResMut, With,
};

pub fn interact_with_building_construction_menu(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &BuildingTileTypeMenuCellInfo,
            &mut BuildingConstructionProgressInfo,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    main_asset_info_resource: Res<MainAssetInfoResource>,
    mut q_placement_building: Query<&mut GlobalBuildingPlacementTiles>,
) {
    for (interaction, mut color, cell_info, mut progress_info) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if progress_info.is_idle() {
                    println!("INFO: {:?}", cell_info);
                    color.0 = Color::DARK_GREEN;

                    let building_tile_type = &cell_info.get_building_tile_type();
                    let building_tile = main_asset_info_resource
                        .get_building_tiles()
                        .get(&building_tile_type)
                        .expect("Building tile not found")
                        .clone();
                    let price = building_tile.get_price();
                    progress_info.start_from_price(price, building_tile);
                }

                if progress_info.is_constructing() {
                    continue;
                }

                if progress_info.is_placing() {
                    let building_tile = main_asset_info_resource
                        .get_building_tiles()
                        .get(&cell_info.get_building_tile_type())
                        .expect("Building tile not found")
                        .clone();
                    q_placement_building
                        .single_mut()
                        .set_ready(Some(building_tile));
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
                if progress_info.is_constructing() {
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
