use crate::features::con_menu::components::{MenuCellInfo, SubMenuInfo, SubMenuType};
use crate::features::con_menu::resources::BuildingProgressInfo;
use crate::features::con_menu::types::{ConMenuBuildingType, ConMenuType};
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Val::Px;
use bevy::prelude::{
    default, AlignItems, BuildChildren, ButtonBundle, Color, FlexDirection, JustifyContent,
    NodeBundle, PositionType, Res, Style, TextBundle, TextStyle, UiImage, UiRect, Val, Visibility,
};

pub fn show_menu_grid(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    sub_menu_type: SubMenuType,
    items: Vec<Box<dyn ConMenuType>>,
) {
    let cols = 2; // Fixed number of columns
    let rows = (items.len() as f32 / cols as f32).ceil() as usize; // Calculate the number of rows based on the number of items and columns
    let padding = 10.0; // Padding between cells
    let cell_width = 100.0;
    let cell_height = 100.0;

    parent
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(SubMenuInfo::new(sub_menu_type))
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
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row_parent| {
                        for col in 0..cols {
                            let index = row * cols + col;
                            if index >= items.len() {
                                break; // Break the loop if we've run out of items
                            }
                            let con_menu_type = items.get(index);
                            if con_menu_type.is_none() {
                                break;
                            }
                            let con_menu_type = con_menu_type.unwrap();
                            let image_path = con_menu_type.image_path();
                            let price = con_menu_type.price();

                            // Spawn each cell as a sprite with a price label
                            row_parent
                                .spawn((
                                    ButtonBundle {
                                        image: UiImage::new(asset_server.load(image_path.clone())),
                                        style: Style {
                                            width: Val::Px(cell_width),
                                            height: Val::Px(cell_height),
                                            margin: UiRect::all(Px(padding)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    MenuCellInfo::new(con_menu_type),
                                    BuildingProgressInfo::new(), // todo: discriminate between building and vehicle
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
                                        .with_style(
                                            Style {
                                                position_type: PositionType::Absolute,
                                                bottom: Val::Px(-padding - padding),
                                                left: Val::Px(5.0),
                                                ..default()
                                            },
                                        ),
                                    );
                                });
                        }
                    });
            }
        });
}
