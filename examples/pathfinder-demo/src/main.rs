//! Pathfinder Demo
//!
//! Demonstrates A* pathfinding for tank navigation on a grid with obstacles.
//! - Click anywhere on the map to move the tank there
//! - Tank navigates around walls (gray) and water (blue)
//! - Green = grass (passable), Yellow = gold (passable)

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use petgraph::Graph;
use std::collections::{HashSet, VecDeque};

// Constants
const TILE_SIZE: f32 = 32.0;
const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 15;
const TANK_SPEED: f32 = 200.0;

// Tile types matching the main game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum TileType {
    Grass,
    Gold,
    Wall,
    Water,
}

impl TileType {
    fn is_passable(&self) -> bool {
        matches!(self, TileType::Grass | TileType::Gold)
    }

    fn color(&self) -> Color {
        match self {
            TileType::Grass => Color::srgb(0.3, 0.6, 0.3),
            TileType::Gold => Color::srgb(0.9, 0.8, 0.2),
            TileType::Wall => Color::srgb(0.4, 0.4, 0.4),
            TileType::Water => Color::srgb(0.2, 0.4, 0.8),
        }
    }
}

type TileCoord = (usize, usize);
type GridSize = (usize, usize);

// Components
#[derive(Component)]
struct Tank {
    movement_path: VecDeque<Vec2>,
    target_position: Vec2,
    moving: bool,
}

#[derive(Component)]
struct Tile {
    coord: TileCoord,
    tile_type: TileType,
}

#[derive(Component)]
struct PathMarker;

// Resources
#[derive(Resource)]
struct GameGrid {
    tiles: Vec<Vec<TileType>>,
    blocking_cells: HashSet<TileCoord>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pathfinder Demo - Click to move tank".into(),
                resolution: bevy::window::WindowResolution::new(
                    (GRID_WIDTH as f32 * TILE_SIZE + 200.0) as u32,
                    (GRID_HEIGHT as f32 * TILE_SIZE + 100.0) as u32,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_grid, setup_tank, setup_ui).chain())
        .add_systems(Update, (handle_click, move_tank, update_info_text))
        .run();
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
    // Create a grid with some obstacles
    let mut tiles = vec![vec![TileType::Grass; GRID_WIDTH]; GRID_HEIGHT];
    let mut blocking_cells = HashSet::new();

    // Add some walls (vertical barrier)
    for y in 2..12 {
        tiles[y][10] = TileType::Wall;
        blocking_cells.insert((10, y));
    }

    // Add some water (horizontal barrier)
    for x in 5..15 {
        tiles[7][x] = TileType::Water;
        blocking_cells.insert((x, 7));
    }

    // Gap in the wall
    tiles[6][10] = TileType::Grass;
    blocking_cells.remove(&(10, 6));

    // Add some gold patches
    for x in 2..5 {
        for y in 10..13 {
            tiles[y][x] = TileType::Gold;
        }
    }

    // Spawn tile sprites
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let tile_type = tiles[y][x];
            let world_pos = tile_to_world((x, y));

            commands.spawn((
                Sprite {
                    color: tile_type.color(),
                    custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                    ..default()
                },
                Transform::from_translation(world_pos.extend(0.0)),
                Tile {
                    coord: (x, y),
                    tile_type,
                },
            ));
        }
    }

    commands.insert_resource(GameGrid {
        tiles,
        blocking_cells,
    });
}

fn setup_tank(mut commands: Commands) {
    let start_pos = tile_to_world((2, 2));

    // Tank body
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.5, 0.8),
            custom_size: Some(Vec2::splat(TILE_SIZE - 8.0)),
            ..default()
        },
        Transform::from_translation(start_pos.extend(10.0)),
        Tank {
            movement_path: VecDeque::new(),
            target_position: start_pos,
            moving: false,
        },
    ));
}

#[derive(Component)]
struct InfoText;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Click anywhere to move the tank\nTank will navigate around obstacles"),
        TextFont {
            font_size: 18.0,
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
}

fn handle_click(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut tank_query: Query<(&mut Tank, &Transform)>,
    grid: Res<GameGrid>,
    path_markers: Query<Entity, With<PathMarker>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window_query.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok((mut tank, tank_transform)) = tank_query.single_mut() else {
        return;
    };

    // Get world position from cursor
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    // Convert to tile coordinates
    let goal = world_to_tile(world_pos);

    // Check bounds
    if goal.0 >= GRID_WIDTH || goal.1 >= GRID_HEIGHT {
        return;
    }

    // Check if goal is passable
    if !grid.tiles[goal.1][goal.0].is_passable() {
        return;
    }

    // Get tank's current tile
    let start = world_to_tile(tank_transform.translation.truncate());

    // Calculate path using A*
    let path = calculate_astar_path(
        (GRID_WIDTH, GRID_HEIGHT),
        start,
        goal,
        &grid.blocking_cells,
    );

    if path.is_empty() {
        return;
    }

    // Clear old path markers
    for entity in path_markers.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new path markers
    for (i, coord) in path.iter().enumerate() {
        if i == 0 {
            continue; // Skip start position
        }
        let pos = tile_to_world(*coord);
        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                custom_size: Some(Vec2::splat(TILE_SIZE * 0.3)),
                ..default()
            },
            Transform::from_translation(pos.extend(5.0)),
            PathMarker,
        ));
    }

    // Set tank path
    let world_path: VecDeque<Vec2> = path.iter().map(|c| tile_to_world(*c)).collect();
    tank.movement_path = world_path;
    tank.moving = true;
    if let Some(first) = tank.movement_path.pop_front() {
        tank.target_position = first;
    }
}

fn move_tank(
    time: Res<Time>,
    mut commands: Commands,
    mut tank_query: Query<(&mut Tank, &mut Transform)>,
    path_markers: Query<Entity, With<PathMarker>>,
) {
    let Ok((mut tank, mut transform)) = tank_query.single_mut() else {
        return;
    };

    if !tank.moving {
        return;
    }

    let current_pos = transform.translation.truncate();
    let direction = tank.target_position - current_pos;
    let distance = direction.length();
    let move_distance = TANK_SPEED * time.delta_secs();

    if distance <= move_distance {
        transform.translation = tank.target_position.extend(transform.translation.z);

        // Get next waypoint
        if let Some(next) = tank.movement_path.pop_front() {
            tank.target_position = next;
        } else {
            tank.moving = false;
            // Clear path markers when done
            for entity in path_markers.iter() {
                commands.entity(entity).despawn();
            }
        }
    } else {
        let movement = direction.normalize() * move_distance;
        transform.translation += movement.extend(0.0);
    }
}

fn update_info_text(
    tank_query: Query<(&Tank, &Transform)>,
    mut text_query: Query<&mut Text, With<InfoText>>,
) {
    let Ok((tank, transform)) = tank_query.single() else {
        return;
    };
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    let tile = world_to_tile(transform.translation.truncate());
    let status = if tank.moving { "Moving" } else { "Idle" };

    text.0 = format!(
        "Click anywhere to move the tank\n\
         Tank position: ({}, {})\n\
         Status: {}\n\
         Path waypoints: {}",
        tile.0,
        tile.1,
        status,
        tank.movement_path.len()
    );
}

// Helper functions
fn tile_to_world(coord: TileCoord) -> Vec2 {
    Vec2::new(
        coord.0 as f32 * TILE_SIZE + TILE_SIZE / 2.0,
        coord.1 as f32 * TILE_SIZE + TILE_SIZE / 2.0,
    )
}

fn world_to_tile(pos: Vec2) -> TileCoord {
    (
        (pos.x / TILE_SIZE).floor() as usize,
        (pos.y / TILE_SIZE).floor() as usize,
    )
}

// A* pathfinding (extracted from main game)
fn calculate_astar_path(
    grid_size: GridSize,
    start: TileCoord,
    goal: TileCoord,
    blocking_cells: &HashSet<TileCoord>,
) -> Vec<TileCoord> {
    let (grid_width, grid_height) = grid_size;
    let mut graph = Graph::<TileCoord, ()>::new();

    // Create a 2D vector to store node indices
    let mut node_indices = vec![vec![None; grid_width]; grid_height];

    // Add nodes to the graph
    for y in 0..grid_height {
        for x in 0..grid_width {
            if !blocking_cells.contains(&(x, y)) || (x, y) == start || (x, y) == goal {
                let node = graph.add_node((x, y));
                node_indices[y][x] = Some(node);
            }
        }
    }

    // Add edges between adjacent nodes
    for y in 0..grid_height {
        for x in 0..grid_width {
            if let Some(node) = node_indices[y][x] {
                let neighbors = [
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                ];

                for (nx, ny) in neighbors {
                    if nx < grid_width && ny < grid_height {
                        if let Some(neighbor_node) = node_indices[ny][nx] {
                            graph.add_edge(node, neighbor_node, ());
                        }
                    }
                }
            }
        }
    }

    if let (Some(start_node), Some(goal_node)) =
        (node_indices[start.1][start.0], node_indices[goal.1][goal.0])
    {
        let result = petgraph::algo::astar(
            &graph,
            start_node,
            |finish| finish == goal_node,
            |_| 1,
            |_| 0,
        );

        result
            .map(|(_cost, path)| {
                path.into_iter()
                    .map(|node| *graph.node_weight(node).unwrap())
                    .collect()
            })
            .unwrap_or_default()
    } else {
        vec![]
    }
}

