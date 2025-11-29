use crate::features::con_menu::components::{
    UnitConstructionProgressInfo, UnitTileTypeMenuCellInfo,
};
use crate::features::con_menu::{SubMenuInfo, SubMenuType};
use crate::features::unit::UnitTileType;
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::ui::Val::Px;

pub fn show_factory_menu_grid(
    parent: &mut ChildSpawnerCommands,
    asset_server: &Res<AssetServer>,
    dynamic_resources: &ResMut<MainAssetInfoResource>,
) {
    let items: Vec<UnitTileType> = vec![UnitTileType::Tank, UnitTileType::Harvester];

    let cols = 2;
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
            SubMenuInfo::new(SubMenuType::Factory),
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
                                            UnitTileTypeMenuCellInfo::new(unit_tile_type.clone()),
                                            UnitConstructionProgressInfo::new(),
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
