use bevy::prelude::*;

pub struct UiMenuPlugin;

impl Plugin for UiMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Assume you have your sprite sheet or individual sprites ready
    let menu_item_image: Handle<Image> = asset_server.load("sprites/tank.png");

    // Grid settings
    let rows = 5;
    let cols = 2;
    let cell_width = 100.0;
    let cell_height = 100.0;
    let padding = 10.0; // Padding between cells

    // Create a parent entity for the grid
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row_parent| {
                    // Add money text
                    add_money_text(&asset_server, row_parent);
                });

            for row in 0..rows {
                // Create a row container
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexStart,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexStart,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row_parent| {
                        for col in 0..cols {
                            // Spawn each cell as a sprite with a price label
                            row_parent
                                .spawn(ImageBundle {
                                    image: UiImage::new(menu_item_image.clone()),
                                    style: Style {
                                        width: Val::Px(cell_width),
                                        height: Val::Px(cell_height),
                                        ..default()
                                    },
                                    // material: materials.add(sprite_handle.clone().into()),
                                    // sprite: Sprite::new(Vec2::new(cell_width, cell_height)),
                                    ..default()
                                })
                                .with_children(|cell| {
                                    // Add price text here if needed, e.g., as a child of the cell
                                    cell.spawn(
                                        TextBundle::from_section(
                                            // This could be dynamic based on the item or cell
                                            format!("Price: {}", 100 * (row * cols + col + 1)),
                                            TextStyle {
                                                font: asset_server
                                                    .load("fonts/AmericanCaptain.ttf"),
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                // Position your price text here, adjust as necessary
                                                position_type: PositionType::Absolute,
                                                bottom: Val::Px(5.0),
                                                right: Val::Px(5.0),
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

fn add_money_text(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
    parent.spawn((
        TextBundle::from_section(
            "CR: 5000",
            TextStyle {
                font: asset_server.load("fonts/AmericanCaptain.ttf"),
                font_size: 30.0,
                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(5.)),
            ..default()
        }),
        Label,
    ));
}
