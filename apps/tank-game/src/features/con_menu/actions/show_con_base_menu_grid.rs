use crate::features::building::types::BuildingTileType;
use crate::features::con_menu::components::BuildingTileTypeMenuCellInfo;
use crate::features::con_menu::{BuildingConstructionProgressInfo, SubMenuInfo, SubMenuType};
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::ui::Val::Px;

pub fn show_con_base_menu_grid(
    parent: &mut ChildSpawnerCommands,
    asset_server: &Res<AssetServer>,
    dynamic_resources: &ResMut<MainAssetInfoResource>,
) {
    let items: Vec<BuildingTileType> = vec![
        BuildingTileType::Base,
        BuildingTileType::Factory,
        BuildingTileType::PowerPlant,
    ];

    let cols = 2; // Fixed number of columns
    let rows = (items.len() as f32 / cols as f32).ceil() as usize;
    let padding = 10.0;
    let cell_width = 100.0;
    let cell_height = 100.0;

    parent
        .spawn((
            Node {
                height: Val::Px(1500.),
                width: Val::Px((cell_width + padding) * 2.),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgb(0.75, 0.15, 0.15)),
            Visibility::Hidden,
            SubMenuInfo::new(SubMenuType::Base),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        width: Val::Px(cell_width * 3.),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.75, 0.15, 0.15)),
                ))
                .with_children(|parent| {
                    for row in 0..rows {
                        // Create a row container
                        parent
                            .spawn(Node {
                                justify_content: JustifyContent::FlexStart,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::FlexStart,
                                padding: UiRect::all(Px(padding)),
                                ..default()
                            })
                            .with_children(|row_parent| {
                                for col in 0..cols {
                                    let index = row * cols + col;
                                    if index >= items.len() {
                                        break;
                                    }
                                    let building_tile_type = items.get(index);
                                    if building_tile_type.is_none() {
                                        break;
                                    }
                                    let building_tile_type = building_tile_type.unwrap();
                                    let building_tile = dynamic_resources
                                        .get_building_tiles()
                                        .get(&building_tile_type);
                                    if building_tile.is_none() {
                                        break;
                                    }
                                    let building_tile = building_tile.unwrap();
                                    let image_path = building_tile.get_image_path();
                                    let price = building_tile.get_price();

                                    // Spawn each cell as a button with a price label
                                    row_parent
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(cell_width),
                                                height: Val::Px(cell_height),
                                                margin: UiRect::all(Px(padding)),
                                                ..default()
                                            },
                                            ImageNode::new(asset_server.load(image_path.clone())),
                                            BuildingTileTypeMenuCellInfo::new(
                                                building_tile_type.clone(),
                                            ),
                                            BuildingConstructionProgressInfo::new(),
                                        ))
                                        .with_children(|cell| {
                                            // Add price text
                                            cell.spawn((
                                                Text::new(format!("Price: {:.2}", price)),
                                                TextFont {
                                                    font: asset_server
                                                        .load("fonts/AmericanCaptain.ttf"),
                                                    font_size: 20.0,
                                                    ..default()
                                                },
                                                TextColor(Color::from(bevy::color::palettes::css::WHITE)),
                                                Node {
                                                    position_type: PositionType::Absolute,
                                                    bottom: Val::Px(-padding - padding),
                                                    left: Val::Px(5.0),
                                                    ..default()
                                                },
                                            ));
                                        });
                                }
                            });
                    }
                });
        });
}
