use crate::features::con_menu::components::{
    UnitConstructionProgressInfo, UnitTileTypeMenuCellInfo,
};
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use bevy::prelude::{
    BackgroundColor, Button, Changed, Color, Interaction, Query, Res, ResMut, With,
};

pub fn interact_with_unit_construction_menu(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &UnitTileTypeMenuCellInfo,
            &mut UnitConstructionProgressInfo,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    main_asset_info_resource: Res<MainAssetInfoResource>,
) {
    for (interaction, mut color, cell_info, mut progress_info) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if progress_info.is_idle() {
                    println!("INFO: {:?}", cell_info);
                    color.0 = Color::DARK_GREEN;

                    let unit_tile = main_asset_info_resource
                        .get_unit_tiles()
                        .get(&cell_info.unit_tile_type())
                        .expect("Unit tile not found")
                        .clone();
                    let price = unit_tile.get_price();
                    progress_info.start_from_price(price, unit_tile);
                }

                if progress_info.is_constructing() {
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
