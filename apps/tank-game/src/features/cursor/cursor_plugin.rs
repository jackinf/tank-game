use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::constants::TILE_SIZE;
use crate::features::building::components::building::Building;
use crate::features::cursor::managers::camera_manager::CameraManager;
use crate::features::cursor::managers::cursor_manager::CursorManager;
use crate::features::cursor::resources::click_info::ClickInfo;
use crate::features::cursor::resources::cursor_coordinates::CursorCoordinates;
use crate::features::tank::components::tank::Tank;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorCoordinates::new())
            .insert_resource(ClickInfo::new(None))
            .add_systems(PreStartup, setup)
            // .add_systems(PreStartup, setup_cursor)
            .add_systems(FixedUpdate, cursor_hovered_over)
            .add_systems(Update, CursorManager::convert_cursor_to_world_position)
            .add_systems(PreStartup, CameraManager::spawn_camera)
            .add_systems(Update, CameraManager::move_camera_with_keys)
            .add_systems(FixedUpdate, CameraManager::move_camera_with_cursor)
            .add_systems(Update, show_cursor_coordinates_in_ui);
    }
}

#[derive(Component)]
struct WorldCoordText;

#[derive(Component)]
struct TileCoordText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
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
                        .spawn((
                            TextBundle::from_section(
                                "Cursor: (##, ##)",
                                TextStyle {
                                    font: asset_server.load("fonts/AmericanCaptain.ttf"),
                                    font_size: 20.0,
                                    ..default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            }),
                            Label,
                        ))
                        .insert(WorldCoordText);

                    parent
                        .spawn((
                            TextBundle::from_section(
                                "Tile: (##, ##)",
                                TextStyle {
                                    font: asset_server.load("fonts/AmericanCaptain.ttf"),
                                    font_size: 20.0,
                                    ..default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            }),
                            Label,
                        ))
                        .insert(TileCoordText);
                });
        });
}

fn show_cursor_coordinates_in_ui(
    cursor_coordinates: Res<CursorCoordinates>,
    mut q_world_coord_text: Query<&mut Text, (With<WorldCoordText>, Without<TileCoordText>)>,
    mut q_tile_coord_text: Query<&mut Text, (With<TileCoordText>, Without<WorldCoordText>)>,
) {
    let mut world_coord_text = q_world_coord_text.single_mut();
    world_coord_text.sections[0].value = format!(
        "Cursor: ({:.2}, {:.2})",
        cursor_coordinates.get_world().x,
        cursor_coordinates.get_world().y
    );

    let mut tile_coord_text = q_tile_coord_text.single_mut();
    let tile_value = if let Some(tile) = cursor_coordinates.get_tile() {
        format!("Tile: ({}, {})", tile.0, tile.1)
    } else {
        "Tile: None".to_string()
    };
    tile_coord_text.sections[0].value = tile_value;
}

fn cursor_hovered_over(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    q_buildings: Query<&Building>,
    q_tanks: Query<&Transform, With<Tank>>,
    cursor_info: Res<CursorCoordinates>,
) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.icon = CursorIcon::Default;

    let cursor_world = cursor_info.get_world();
    let cursor_tile = cursor_info.get_tile();
    if cursor_tile.is_none() {
        return;
    }
    let cursor_tile = cursor_tile.unwrap();

    // check if the cursor is hovered over any building
    for building in q_buildings.iter() {
        if building.contains(cursor_tile) {
            primary_window.cursor.icon = CursorIcon::Grabbing;
            return;
        }
    }

    // check if the cursor is hovered over any tank
    for tank_transform in q_tanks.iter() {
        let tank_position = tank_transform.translation.truncate();
        if tank_position.distance(cursor_world) < TILE_SIZE / 2. {
            primary_window.cursor.icon = CursorIcon::Grabbing;
            return;
        }
    }

    // primary_window.cursor.grab_mode = CursorGrabMode::Confined;
    // primary_window.cursor.visible = false;
}
