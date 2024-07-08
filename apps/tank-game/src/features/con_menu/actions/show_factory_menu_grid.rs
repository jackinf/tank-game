use crate::features::con_menu::components::{
    ConMenuVehicleInfo, UnitConstructionProgressInfo, UnitTileTypeMenuCellInfo,
};
use crate::features::con_menu::{BuildingConstructionProgressInfo, SubMenuInfo, SubMenuType};
use crate::features::unit::UnitTileType;
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Val::Px;
use bevy::prelude::{
    default, AlignItems, BuildChildren, ButtonBundle, Color, FlexDirection, JustifyContent,
    NodeBundle, PositionType, Res, ResMut, Style, TextBundle, TextStyle, UiImage, UiRect, Val,
    Visibility,
};

pub fn show_factory_menu_grid(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    dynamic_resources: &ResMut<MainAssetInfoResource>,
) {
    let items: Vec<UnitTileType> = vec![UnitTileType::Tank, UnitTileType::Harvester];

    let cols = 2; // Fixed number of columns
    let rows = (items.len() as f32 / cols as f32).ceil() as usize; // Calculate the number of rows based on the number of items and columns
    let padding = 10.0; // Padding between cells
    let cell_width = 100.0;
    let cell_height = 100.0;

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Px(1500.),
                    width: Val::Px((cell_width + padding) * 2.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgb(0.75, 0.15, 0.15).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            SubMenuInfo::new(SubMenuType::Factory),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        width: Val::Px(cell_width * 3.),
                        ..default()
                    },
                    background_color: Color::rgb(0.75, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    for row in 0..rows {
                        // Create a row container
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::FlexStart,
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    padding: UiRect::all(Px(padding)),
                                    // width: Val::Percent(100.),
                                    ..default()
                                },
                                // background_color: Color::rgb(0.75, 0.15, 0.15).into(),
                                ..default()
                            })
                            .with_children(|row_parent| {
                                for col in 0..cols {
                                    let index = row * cols + col;
                                    if index >= items.len() {
                                        break; // Break the loop if we've run out of items
                                    }
                                    let unit_tile_type = items.get(index);
                                    if unit_tile_type.is_none() {
                                        break;
                                    }
                                    let unit_tile_type = unit_tile_type.unwrap();
                                    let unit_tile =
                                        dynamic_resources.get_unit_tiles().get(&unit_tile_type);
                                    if unit_tile.is_none() {
                                        break;
                                    }
                                    let unit_tile = unit_tile.unwrap();

                                    let image_path = unit_tile.get_image_path();
                                    let price = unit_tile.get_price();

                                    // Spawn each cell as a sprite with a price label
                                    row_parent
                                        .spawn((
                                            ButtonBundle {
                                                image: UiImage::new(
                                                    asset_server.load(image_path.clone()),
                                                ),
                                                style: Style {
                                                    width: Val::Px(cell_width),
                                                    height: Val::Px(cell_height),
                                                    margin: UiRect::all(Px(padding)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            UnitTileTypeMenuCellInfo::new(unit_tile_type.clone()),
                                            UnitConstructionProgressInfo::new(),
                                        ))
                                        .with_children(|cell| {
                                            // Add price text here, dynamic based on the item
                                            cell.spawn(
                                                TextBundle::from_section(
                                                    format!("Price: {:.2}", price),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/AmericanCaptain.ttf"),
                                                        font_size: 20.0,
                                                        color: Color::WHITE,
                                                    },
                                                )
                                                .with_style(Style {
                                                    position_type: PositionType::Absolute,
                                                    bottom: Val::Px(-padding - padding),
                                                    left: Val::Px(5.0),
                                                    ..default()
                                                }),
                                            );
                                        });
                                }
                            });
                    }
                });
        });
}
