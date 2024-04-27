use crate::con_menu::components::money_text::MoneyText;
use crate::con_menu::resources::menu_info::MenuInfo;
use bevy::prelude::Val::Px;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Update, detect_mouse_over_container)
            .add_systems(Update, MoneyText::update)
            .add_systems(Update, toggle_menu_visibility);
    }
}

#[derive(Component)]
struct Target<T> {
    id: Entity,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Target<T> {
    fn new(id: Entity) -> Self {
        Self {
            id,
            phantom: std::marker::PhantomData,
        }
    }
}

fn toggle_menu_visibility(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<MenuInfo>>,
) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        let mut visibility = query.single_mut();
        *visibility = Visibility::Visible;
    } else if keyboard.just_pressed(KeyCode::KeyB) {
        let mut visibility = query.single_mut();
        *visibility = Visibility::Hidden;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Assume you have your sprite sheet or individual sprites ready
    let menu_item_image: Handle<Image> = asset_server.load("sprites/tank.png");

    // Grid settings
    let rows = 5;
    let cols = 2;
    let cell_width = 100.0;
    let cell_height = 100.0;
    let padding = 10.0; // Padding between cells

    let menu_info = MenuInfo::new();

    // Create a parent entity for the grid
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        })
        .insert(Interaction::None)
        .insert(menu_info.clone())
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
                    MoneyText::spawn(&asset_server, row_parent, menu_info.get_money());
                });

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
                            // Spawn each cell as a sprite with a price label
                            row_parent
                                .spawn(ImageBundle {
                                    image: UiImage::new(menu_item_image.clone()),
                                    style: Style {
                                        width: Val::Px(cell_width),
                                        height: Val::Px(cell_height),
                                        ..default()
                                    },
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

fn detect_mouse_over_container(
    query: Query<&Interaction, (Changed<Interaction>, Without<Button>)>,
    mut q_menu_info: Query<&mut MenuInfo>,
) {
    let mut menu_info = q_menu_info.single_mut();
    for interaction in query.iter() {
        match *interaction {
            Interaction::Hovered => menu_info.set_hovered(true),
            Interaction::None => menu_info.set_hovered(false),
            _ => {} // Handle other states as needed
        }
    }
}
