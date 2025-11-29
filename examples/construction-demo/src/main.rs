//! Construction Demo
//!
//! Demonstrates building construction mechanics:
//! - Press 1-4 to select a building type from the menu
//! - Click on the grid to place the building
//! - Buildings cost money and take time to construct
//! - Cannot place buildings on top of each other or on obstacles
//!
//! Building Types:
//! 1. Construction Yard (2x2) - $500
//! 2. Power Plant (2x2) - $300
//! 3. Ore Refinery (3x2) - $1000
//! 4. War Factory (3x3) - $800

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashSet;

// Constants
const TILE_SIZE: f32 = 32.0;
const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 15;
const STARTING_MONEY: u32 = 5000;
const CONSTRUCTION_SPEED: f32 = 50.0; // Progress per second

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Construction Demo - Press 1-4 to select building, click to place".into(),
                resolution: bevy::window::WindowResolution::new(800, 600),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_systems(
            Startup,
            (setup_camera, setup_grid, setup_ui).chain(),
        )
        .add_systems(
            Update,
            (
                handle_building_selection,
                handle_placement,
                update_construction_progress,
                update_preview,
                update_ui,
            ),
        )
        .run();
}

// Building types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BuildingType {
    ConstructionYard,
    PowerPlant,
    OreRefinery,
    WarFactory,
}

impl BuildingType {
    fn size(&self) -> (usize, usize) {
        match self {
            BuildingType::ConstructionYard => (2, 2),
            BuildingType::PowerPlant => (2, 2),
            BuildingType::OreRefinery => (3, 2),
            BuildingType::WarFactory => (3, 3),
        }
    }

    fn cost(&self) -> u32 {
        match self {
            BuildingType::ConstructionYard => 500,
            BuildingType::PowerPlant => 300,
            BuildingType::OreRefinery => 1000,
            BuildingType::WarFactory => 800,
        }
    }

    fn color(&self) -> Color {
        match self {
            BuildingType::ConstructionYard => Color::srgb(0.6, 0.6, 0.2),
            BuildingType::PowerPlant => Color::srgb(0.8, 0.6, 0.1),
            BuildingType::OreRefinery => Color::srgb(0.2, 0.6, 0.6),
            BuildingType::WarFactory => Color::srgb(0.4, 0.4, 0.6),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            BuildingType::ConstructionYard => "Construction Yard",
            BuildingType::PowerPlant => "Power Plant",
            BuildingType::OreRefinery => "Ore Refinery",
            BuildingType::WarFactory => "War Factory",
        }
    }
}

// Components
#[derive(Component)]
struct Building {
    building_type: BuildingType,
    tiles: HashSet<(usize, usize)>,
    construction_progress: f32, // 0-100
    completed: bool,
}

#[derive(Component)]
struct BuildingProgressBar;

#[derive(Component)]
struct PlacementPreview;

#[derive(Component)]
struct Tile {
    coord: (usize, usize),
}

#[derive(Component)]
struct InfoText;

#[derive(Component)]
struct MenuText;

// Resources
#[derive(Resource)]
struct GameState {
    money: u32,
    selected_building: Option<BuildingType>,
    occupied_tiles: HashSet<(usize, usize)>,
    preview_valid: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            money: STARTING_MONEY,
            selected_building: None,
            occupied_tiles: HashSet::new(),
            preview_valid: false,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let center_x = (GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
    let center_y = (GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;

    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(center_x, center_y, 1000.0)),
    ));
}

fn setup_grid(mut commands: Commands) {
    // Spawn grid tiles
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let world_pos = tile_to_world((x, y));

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.2, 0.35, 0.2),
                    custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                    ..default()
                },
                Transform::from_translation(world_pos.extend(0.0)),
                Tile { coord: (x, y) },
            ));
        }
    }

    // Spawn placement preview (initially hidden)
    commands.spawn((
        Sprite {
            color: Color::srgba(0.5, 0.5, 0.5, 0.5),
            custom_size: Some(Vec2::splat(TILE_SIZE * 2.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-1000.0, -1000.0, 50.0)),
        PlacementPreview,
        Visibility::Hidden,
    ));
}

fn setup_ui(mut commands: Commands) {
    // Info text (top left)
    commands.spawn((
        Text::new("Money: $5000"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        InfoText,
    ));

    // Menu text (top right)
    commands.spawn((
        Text::new(
            "Buildings:\n\
             [1] Construction Yard - $500 (2x2)\n\
             [2] Power Plant - $300 (2x2)\n\
             [3] Ore Refinery - $1000 (3x2)\n\
             [4] War Factory - $800 (3x3)\n\
             [ESC] Cancel selection",
        ),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        MenuText,
    ));
}

fn handle_building_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        state.selected_building = Some(BuildingType::ConstructionYard);
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        state.selected_building = Some(BuildingType::PowerPlant);
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        state.selected_building = Some(BuildingType::OreRefinery);
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        state.selected_building = Some(BuildingType::WarFactory);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        state.selected_building = None;
    }
}

fn handle_placement(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<GameState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(building_type) = state.selected_building else {
        return;
    };

    if !state.preview_valid {
        return;
    }

    // Check if we can afford it
    if state.money < building_type.cost() {
        return;
    }

    let Ok(window) = window_query.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    let tile_coord = world_to_tile(world_pos);
    let (width, height) = building_type.size();

    // Calculate all tiles this building will occupy
    let mut tiles = HashSet::new();
    for dy in 0..height {
        for dx in 0..width {
            let tx = tile_coord.0 + dx;
            let ty = tile_coord.1 + dy;
            tiles.insert((tx, ty));
        }
    }

    // Mark tiles as occupied
    for tile in &tiles {
        state.occupied_tiles.insert(*tile);
    }

    // Deduct cost
    state.money -= building_type.cost();

    // Calculate building center position
    let center_x = tile_coord.0 as f32 * TILE_SIZE + (width as f32 * TILE_SIZE) / 2.0;
    let center_y = tile_coord.1 as f32 * TILE_SIZE + (height as f32 * TILE_SIZE) / 2.0;

    // Spawn building entity (semi-transparent during construction)
    let building_entity = commands
        .spawn((
            Sprite {
                color: building_type.color().with_alpha(0.5),
                custom_size: Some(Vec2::new(
                    width as f32 * TILE_SIZE - 4.0,
                    height as f32 * TILE_SIZE - 4.0,
                )),
                ..default()
            },
            Transform::from_translation(Vec3::new(center_x, center_y, 10.0)),
            Building {
                building_type,
                tiles,
                construction_progress: 0.0,
                completed: false,
            },
        ))
        .id();

    // Spawn progress bar
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.6, 0.1),
            custom_size: Some(Vec2::new(0.0, 4.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            center_x,
            center_y + (height as f32 * TILE_SIZE) / 2.0 + 6.0,
            15.0,
        )),
        BuildingProgressBar,
        ChildOf(building_entity),
    ));

    // Clear selection after placing
    state.selected_building = None;
}

#[derive(Component)]
struct ChildOf(Entity);

fn update_construction_progress(
    time: Res<Time>,
    mut buildings_query: Query<(Entity, &mut Building, &mut Sprite)>,
    mut progress_bars: Query<(&ChildOf, &mut Sprite, &mut Transform), (With<BuildingProgressBar>, Without<Building>)>,
) {
    for (entity, mut building, mut sprite) in buildings_query.iter_mut() {
        if building.completed {
            continue;
        }

        building.construction_progress += CONSTRUCTION_SPEED * time.delta_secs();

        if building.construction_progress >= 100.0 {
            building.construction_progress = 100.0;
            building.completed = true;
            sprite.color = building.building_type.color();
        }

        // Update progress bar
        for (child_of, mut bar_sprite, _) in progress_bars.iter_mut() {
            if child_of.0 == entity {
                let (width, _) = building.building_type.size();
                let max_width = width as f32 * TILE_SIZE - 8.0;
                let progress_width = max_width * (building.construction_progress / 100.0);
                bar_sprite.custom_size = Some(Vec2::new(progress_width, 4.0));

                if building.completed {
                    bar_sprite.color = Color::NONE;
                }
            }
        }
    }
}

fn update_preview(
    mut state: ResMut<GameState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut preview_query: Query<(&mut Transform, &mut Sprite, &mut Visibility), With<PlacementPreview>>,
) {
    let Ok((mut transform, mut sprite, mut visibility)) = preview_query.single_mut() else {
        return;
    };

    let Some(building_type) = state.selected_building else {
        *visibility = Visibility::Hidden;
        return;
    };

    let Ok(window) = window_query.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        *visibility = Visibility::Hidden;
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    let tile_coord = world_to_tile(world_pos);
    let (width, height) = building_type.size();

    // Check if placement is valid
    let mut valid = true;

    // Check bounds
    if tile_coord.0 + width > GRID_WIDTH || tile_coord.1 + height > GRID_HEIGHT {
        valid = false;
    }

    // Check for overlapping buildings
    if valid {
        for dy in 0..height {
            for dx in 0..width {
                let tx = tile_coord.0 + dx;
                let ty = tile_coord.1 + dy;
                if state.occupied_tiles.contains(&(tx, ty)) {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }
    }

    // Check if we can afford it
    if state.money < building_type.cost() {
        valid = false;
    }

    state.preview_valid = valid;

    // Update preview appearance
    *visibility = Visibility::Visible;

    let center_x = tile_coord.0 as f32 * TILE_SIZE + (width as f32 * TILE_SIZE) / 2.0;
    let center_y = tile_coord.1 as f32 * TILE_SIZE + (height as f32 * TILE_SIZE) / 2.0;

    transform.translation = Vec3::new(center_x, center_y, 50.0);

    sprite.custom_size = Some(Vec2::new(
        width as f32 * TILE_SIZE - 4.0,
        height as f32 * TILE_SIZE - 4.0,
    ));

    if valid {
        sprite.color = Color::srgba(0.3, 0.8, 0.3, 0.5);
    } else {
        sprite.color = Color::srgba(0.8, 0.3, 0.3, 0.5);
    }
}

fn update_ui(
    state: Res<GameState>,
    buildings_query: Query<&Building>,
    mut info_text: Query<&mut Text, With<InfoText>>,
) {
    let Ok(mut text) = info_text.single_mut() else {
        return;
    };

    let selected_info = if let Some(bt) = state.selected_building {
        format!("Selected: {} (${}) - Click to place", bt.name(), bt.cost())
    } else {
        "Press 1-4 to select a building".to_string()
    };

    let buildings_info: Vec<String> = buildings_query
        .iter()
        .map(|b| {
            if b.completed {
                format!("{} - Complete", b.building_type.name())
            } else {
                format!("{} - {:.0}%", b.building_type.name(), b.construction_progress)
            }
        })
        .collect();

    text.0 = format!(
        "Money: ${}\n\
         {}\n\
         \n\
         Built:\n\
         {}",
        state.money,
        selected_info,
        if buildings_info.is_empty() {
            "None".to_string()
        } else {
            buildings_info.join("\n")
        }
    );
}

// Helper functions
fn tile_to_world(coord: (usize, usize)) -> Vec2 {
    Vec2::new(
        coord.0 as f32 * TILE_SIZE + TILE_SIZE / 2.0,
        coord.1 as f32 * TILE_SIZE + TILE_SIZE / 2.0,
    )
}

fn world_to_tile(pos: Vec2) -> (usize, usize) {
    (
        (pos.x / TILE_SIZE).floor().max(0.0) as usize,
        (pos.y / TILE_SIZE).floor().max(0.0) as usize,
    )
}

