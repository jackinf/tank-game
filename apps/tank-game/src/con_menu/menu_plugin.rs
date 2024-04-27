use crate::common::constants::{SPRITE_SCALE, TILE_SIZE};

use crate::building::building_type::BuildingType;
use crate::building::components::building::Building;
use crate::common::player::Player;
use crate::con_menu::components::money_text::MoneyText;
use crate::con_menu::resources::menu_info::MenuInfo;
use crate::cursor::resources::cursor_coordinates::CursorCoordinates;
use crate::tile::components::tile::Tile;
use crate::tile::tile_queries::TileQueries;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::Val::Px;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .insert_resource(MenuInfo::new())
            .insert_resource(ConstructionInfo::new())
            .add_systems(Update, detect_mouse_over_container)
            .add_systems(FixedUpdate, draw_construction_tiles)
            .add_systems(Update, MoneyText::update);
    }
}

#[derive(Resource)]
pub struct ConstructionInfo {
    ready: bool,
}

impl ConstructionInfo {
    pub fn set_ready(&mut self, ready: bool) {
        self.ready = ready;
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

impl ConstructionInfo {
    pub fn new() -> Self {
        Self { ready: false }
    }
}

#[derive(Component)]
pub struct PlacementBuilding {
    layout: (usize, usize),
}

impl PlacementBuilding {
    pub fn new() -> Self {
        Self { layout: (2, 2) }
    }

    pub fn get_layout(&self) -> (usize, usize) {
        self.layout
    }
}

fn draw_construction_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_tiles: Query<&Tile>,
    mut q_placement: Query<
        (&mut Transform, &mut Sprite, &PlacementBuilding),
        With<PlacementBuilding>,
    >,
    cursor: Res<CursorCoordinates>,
    mut construction_info: ResMut<ConstructionInfo>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
) {
    if !construction_info.ready {
        return;
    }

    match (
        q_placement.single_mut(),
        TileQueries::find_accessible_tile(&q_tiles, &cursor.0),
    ) {
        ((mut transform, mut sprite, placement), Some(tile)) => {
            sprite.color.set_a(0.5); // show tile
            let (x, y) = tile.get_world_coord();
            transform.translation = Vec3::new(x, y, transform.translation.z);

            for mouse_button_event in mouse_button_events.read() {
                if mouse_button_event.button == MouseButton::Left
                    && mouse_button_event.state == ButtonState::Pressed
                {
                    // validate if all tiles in layout.x * layout.y are accessible
                    let (layout_x, layout_y) = placement.get_layout();
                    let mut all_accessible = true;
                    for i in 0..layout_x {
                        for j in 0..layout_y {
                            let (x, y) = tile.get_tile_coord();
                            let tile = TileQueries::find_tile(&q_tiles, (x + i, y + j));
                            if tile.is_none() || !tile.unwrap().accessible() {
                                all_accessible = false;
                                break;
                            }
                        }
                    }

                    if !all_accessible {
                        continue;
                    }

                    sprite.color.set_a(0.0);
                    construction_info.set_ready(false);

                    // spawn a building
                    commands
                        .spawn(SpriteBundle {
                            texture: asset_server.load("sprites/building_a.png"),
                            transform: Transform::default()
                                .with_translation(Vec2::new(x, y).extend(100.))
                                .with_scale(Vec3::splat(SPRITE_SCALE)),
                            ..default()
                        })
                        .insert(Building::new(
                            BuildingType::Base,
                            tile.get_tile_coord(),
                            Player::P1,
                        ));
                }
            }
        }
        ((_, mut sprite, _), None) => {
            sprite.color.set_a(0.0); // hide tile
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, menu_info: Res<MenuInfo>) {
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
        .insert(Interaction::None)
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
                    MoneyText::spawn(&asset_server, row_parent, menu_info);
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

    // selector entity for placing buildings
    commands
        .spawn((SpriteBundle {
            texture: asset_server.load("pixels/white.png"),
            transform: Transform::default()
                .with_translation(Vec3::new(0., 0., 100.))
                .with_scale(Vec2::new(2.0 * TILE_SIZE, 2.0 * TILE_SIZE).extend(1.0)),
            sprite: Sprite {
                color: Color::PINK.with_a(0.0), // hide by default
                ..default()
            },
            ..default()
        },))
        .insert(PlacementBuilding::new());
}

fn detect_mouse_over_container(
    query: Query<&Interaction, (Changed<Interaction>, Without<Button>)>,
    mut menu_info: ResMut<MenuInfo>,
) {
    for interaction in query.iter() {
        match *interaction {
            Interaction::Hovered => menu_info.set_hovered(true),
            Interaction::None => menu_info.set_hovered(false),
            _ => {} // Handle other states as needed
        }
    }
}
