use crate::features::con_menu::components::{
    UnitConstructionProgressInfo, UnitTileTypeMenuCellInfo,
};
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use bevy::color::palettes::css;
use bevy::prelude::{
    BackgroundColor, Button, Changed, Color, Interaction, Query, Res, With,
};

pub fn sys_interact_with_unit_construction_menu(
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
                    *color = Color::from(css::DARK_GREEN).into();

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
                    *color = Color::from(css::YELLOW).into();
                } else if progress_info.is_placing() {
                    *color = Color::from(css::GREEN).into();
                }
            }
            Interaction::None => {
                if progress_info.is_constructing() {
                    *color = Color::from(css::GRAY).into();
                } else if progress_info.is_placing() {
                    *color = Color::from(css::GREEN).into();
                } else if progress_info.is_idle() {
                    *color = Color::from(css::WHITE).into();
                } else {
                    *color = Color::from(css::WHITE).into();
                }
            }
        }
    }
}
