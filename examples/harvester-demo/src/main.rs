//! Harvester Demo
//!
//! Demonstrates the harvester state machine:
//! - Harvester automatically wakes up and searches for ore (gold)
//! - Moves to ore, collects it, then returns to base (refinery)
//! - Unloads gold at the refinery, then repeats
//! - Right-click to manually send the harvester somewhere (interrupts auto behavior)
//! - After manual movement completes, harvester resumes automatic cycle
//!
//! State Machine:
//! Idle -> SearchingForGold -> MovingToGold -> Harvesting -> FindBaseToReturn -> ReturningToBase -> Unloading -> Idle

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use petgraph::Graph;
use std::collections::{HashSet, VecDeque};

// Constants
const TILE_SIZE: f32 = 32.0;
const GRID_WIDTH: usize = 25;
const GRID_HEIGHT: usize = 18;
const HARVESTER_SPEED: f32 = 100.0;
const HARVEST_COOLDOWN: f32 = 0.5;
const GOLD_PER_HARVEST: u32 = 10;
const MAX_GOLD_CAPACITY: u32 = 50;
const UNLOAD_RATE: u32 = 25; // gold per second

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Harvester Demo - Right-click to manually control, watch auto behavior".into(),
                resolution: bevy::window::WindowResolution::new(900, 700),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<PlayerMoney>()
        .add_systems(
            Startup,
            (setup_camera, setup_grid, setup_entities, setup_ui).chain(),
        )
        .add_systems(
            Update,
            (
                handle_manual_command,
                harvester_state_machine,
                move_harvester,
                update_ui,
            ),
        )
        .run();
}

// Harvester States
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum HarvesterState {
    Idle,
    SearchingForGold,
    MovingToGold,
    Harvesting,
    FindBaseToReturn,
    ReturningToBase,
    Unloading,
    ManualMove, // Player-controlled movement
}

impl HarvesterState {
    fn name(&self) -> &'static str {
        match self {
            HarvesterState::Idle => "Idle",
            HarvesterState::SearchingForGold => "Searching for Gold",
            HarvesterState::MovingToGold => "Moving to Gold",
            HarvesterState::Harvesting => "Harvesting",
            HarvesterState::FindBaseToReturn => "Looking for Refinery",
            HarvesterState::ReturningToBase => "Returning to Base",
            HarvesterState::Unloading => "Unloading",
            HarvesterState::ManualMove => "Manual Control",
        }
    }
}

// Tile types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileType {
    Grass,
    Gold,
    Wall,
}

impl TileType {
    fn is_passable(&self) -> bool {
        matches!(self, TileType::Grass | TileType::Gold)
    }

    fn color(&self) -> Color {
        match self {
            TileType::Grass => Color::srgb(0.2, 0.4, 0.2),
            TileType::Gold => Color::srgb(0.85, 0.75, 0.15),
            TileType::Wall => Color::srgb(0.3, 0.3, 0.35),
        }
    }
}

type TileCoord = (usize, usize);

// Components
#[derive(Component)]
struct Harvester {
    state: HarvesterState,
    gold_capacity: u32,
    max_capacity: u32,
    movement_path: VecDeque<Vec2>,
    target_position: Vec2,
    harvest_timer: Timer,
    unload_timer: Timer,
}

#[derive(Component)]
struct Tile {
    coord: TileCoord,
    tile_type: TileType,
}

#[derive(Component)]
struct GoldTile {
    amount: u32,
}

#[derive(Component)]
struct Refinery;

#[derive(Component)]
struct InfoText;

#[derive(Component)]
struct PathMarker;

// Resources
#[derive(Resource, Default)]
struct PlayerMoney(u32);

#[derive(Resource)]
struct GameGrid {
    tiles: Vec<Vec<TileType>>,
    blocking_cells: HashSet<TileCoord>,
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
    let mut tiles = vec![vec![TileType::Grass; GRID_WIDTH]; GRID_HEIGHT];
    let mut blocking_cells = HashSet::new();

    // Add gold patches on the right side
    for x in 18..23 {
        for y in 3..8 {
            tiles[y][x] = TileType::Gold;
        }
    }

    for x in 19..22 {
        for y in 11..15 {
            tiles[y][x] = TileType::Gold;
        }
    }

    // Add some walls
    for y in 5..13 {
        tiles[y][12] = TileType::Wall;
        blocking_cells.insert((12, y));
    }

    // Gap in wall
    tiles[9][12] = TileType::Grass;
    blocking_cells.remove(&(12, 9));

    // Spawn tile sprites
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let tile_type = tiles[y][x];
            let world_pos = tile_to_world((x, y));

            let entity = commands
                .spawn((
                    Sprite {
                        color: tile_type.color(),
                        custom_size: Some(Vec2::splat(TILE_SIZE - 1.0)),
                        ..default()
                    },
                    Transform::from_translation(world_pos.extend(0.0)),
                    Tile {
                        coord: (x, y),
                        tile_type,
                    },
                ))
                .id();

            // Add GoldTile component to gold tiles
            if tile_type == TileType::Gold {
                commands.entity(entity).insert(GoldTile { amount: 100 });
            }
        }
    }

    commands.insert_resource(GameGrid {
        tiles,
        blocking_cells,
    });
}

fn setup_entities(mut commands: Commands) {
    // Spawn Refinery (base) on the left side
    let refinery_pos = tile_to_world((3, 9));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.5, 0.6),
            custom_size: Some(Vec2::new(TILE_SIZE * 3.0 - 4.0, TILE_SIZE * 2.0 - 4.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            refinery_pos.x + TILE_SIZE,
            refinery_pos.y + TILE_SIZE / 2.0,
            5.0,
        )),
        Refinery,
    ));

    // Spawn Harvester near the refinery
    let harvester_pos = tile_to_world((5, 9));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.2),
            custom_size: Some(Vec2::splat(TILE_SIZE - 6.0)),
            ..default()
        },
        Transform::from_translation(harvester_pos.extend(10.0)),
        Harvester {
            state: HarvesterState::Idle,
            gold_capacity: 0,
            max_capacity: MAX_GOLD_CAPACITY,
            movement_path: VecDeque::new(),
            target_position: harvester_pos,
            harvest_timer: Timer::from_seconds(HARVEST_COOLDOWN, TimerMode::Repeating),
            unload_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
    ));
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Harvester Demo"),
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

fn handle_manual_command(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut harvester_query: Query<(&mut Harvester, &Transform)>,
    grid: Res<GameGrid>,
    path_markers: Query<Entity, With<PathMarker>>,
) {
    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    let Ok(window) = window_query.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok((mut harvester, harvester_transform)) = harvester_query.single_mut() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    let goal = world_to_tile(world_pos);

    // Check bounds
    if goal.0 >= GRID_WIDTH || goal.1 >= GRID_HEIGHT {
        return;
    }

    // Check if passable
    if !grid.tiles[goal.1][goal.0].is_passable() {
        return;
    }

    let start = world_to_tile(harvester_transform.translation.truncate());

    let path = calculate_path(
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

    // Spawn path markers
    for coord in &path {
        let pos = tile_to_world(*coord);
        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 0.5, 0.0, 0.4),
                custom_size: Some(Vec2::splat(TILE_SIZE * 0.3)),
                ..default()
            },
            Transform::from_translation(pos.extend(8.0)),
            PathMarker,
        ));
    }

    // Set harvester to manual mode
    let world_path: VecDeque<Vec2> = path.iter().map(|c| tile_to_world(*c)).collect();
    harvester.movement_path = world_path;
    harvester.state = HarvesterState::ManualMove;
    if let Some(first) = harvester.movement_path.pop_front() {
        harvester.target_position = first;
    }
}

fn harvester_state_machine(
    mut commands: Commands,
    time: Res<Time>,
    grid: Res<GameGrid>,
    mut money: ResMut<PlayerMoney>,
    mut harvester_query: Query<(&mut Harvester, &Transform)>,
    gold_tiles: Query<(&Tile, &GoldTile)>,
    refinery_query: Query<&Transform, With<Refinery>>,
    path_markers: Query<Entity, With<PathMarker>>,
) {
    let Ok((mut harvester, transform)) = harvester_query.single_mut() else {
        return;
    };

    let harvester_tile = world_to_tile(transform.translation.truncate());

    match harvester.state {
        HarvesterState::Idle => {
            // After being idle, start searching for gold
            harvester.state = HarvesterState::SearchingForGold;
        }

        HarvesterState::SearchingForGold => {
            // Find nearest gold tile
            let gold_positions: HashSet<TileCoord> = gold_tiles
                .iter()
                .filter(|(_, gold)| gold.amount > 0)
                .map(|(tile, _)| tile.coord)
                .collect();

            if gold_positions.is_empty() {
                // No gold left, stay idle
                harvester.state = HarvesterState::Idle;
                return;
            }

            // BFS to find nearest gold
            if let Some(path) = find_nearest_target(
                (GRID_WIDTH, GRID_HEIGHT),
                harvester_tile,
                &gold_positions,
                &grid.blocking_cells,
            ) {
                let world_path: VecDeque<Vec2> = path.iter().map(|c| tile_to_world(*c)).collect();
                harvester.movement_path = world_path;
                harvester.state = HarvesterState::MovingToGold;
                if let Some(first) = harvester.movement_path.pop_front() {
                    harvester.target_position = first;
                }
            }
        }

        HarvesterState::MovingToGold => {
            // Check if arrived at gold
            if harvester.movement_path.is_empty()
                && transform.translation.truncate().distance(harvester.target_position) < 5.0
            {
                // Check if we're on a gold tile
                let is_on_gold = gold_tiles
                    .iter()
                    .any(|(tile, gold)| tile.coord == harvester_tile && gold.amount > 0);

                if is_on_gold {
                    harvester.state = HarvesterState::Harvesting;
                } else {
                    // Gold depleted, search again
                    harvester.state = HarvesterState::SearchingForGold;
                }
            }
        }

        HarvesterState::Harvesting => {
            harvester.harvest_timer.tick(time.delta());

            if harvester.harvest_timer.just_finished() {
                // Check if there's gold at current position
                let has_gold = gold_tiles
                    .iter()
                    .any(|(tile, gold)| tile.coord == harvester_tile && gold.amount > 0);

                if has_gold && harvester.gold_capacity < harvester.max_capacity {
                    harvester.gold_capacity =
                        (harvester.gold_capacity + GOLD_PER_HARVEST).min(harvester.max_capacity);
                }

                // Check if full or no more gold here
                if harvester.gold_capacity >= harvester.max_capacity || !has_gold {
                    harvester.state = HarvesterState::FindBaseToReturn;
                }
            }
        }

        HarvesterState::FindBaseToReturn => {
            // Find path to refinery
            if let Ok(refinery_transform) = refinery_query.single() {
                let refinery_tile = world_to_tile(refinery_transform.translation.truncate());

                // Find path to area near refinery
                let mut refinery_area = HashSet::new();
                for dx in 0..=3 {
                    for dy in 0..=2 {
                        let tx = refinery_tile.0.saturating_sub(1) + dx;
                        let ty = refinery_tile.1.saturating_sub(1) + dy;
                        if tx < GRID_WIDTH && ty < GRID_HEIGHT {
                            refinery_area.insert((tx, ty));
                        }
                    }
                }

                if let Some(path) = find_nearest_target(
                    (GRID_WIDTH, GRID_HEIGHT),
                    harvester_tile,
                    &refinery_area,
                    &grid.blocking_cells,
                ) {
                    let world_path: VecDeque<Vec2> = path.iter().map(|c| tile_to_world(*c)).collect();
                    harvester.movement_path = world_path;
                    harvester.state = HarvesterState::ReturningToBase;
                    if let Some(first) = harvester.movement_path.pop_front() {
                        harvester.target_position = first;
                    }
                }
            }
        }

        HarvesterState::ReturningToBase => {
            // Check if arrived at base
            if harvester.movement_path.is_empty()
                && transform.translation.truncate().distance(harvester.target_position) < 5.0
            {
                harvester.state = HarvesterState::Unloading;
            }
        }

        HarvesterState::Unloading => {
            harvester.unload_timer.tick(time.delta());

            if harvester.unload_timer.just_finished() {
                let unload_amount = (UNLOAD_RATE as f32 * 0.1) as u32;
                if harvester.gold_capacity > 0 {
                    let actual_unload = harvester.gold_capacity.min(unload_amount);
                    harvester.gold_capacity -= actual_unload;
                    money.0 += actual_unload;
                }

                if harvester.gold_capacity == 0 {
                    // Done unloading, go back to searching
                    harvester.state = HarvesterState::Idle;
                }
            }
        }

        HarvesterState::ManualMove => {
            // Clear path markers when done
            if harvester.movement_path.is_empty()
                && transform.translation.truncate().distance(harvester.target_position) < 5.0
            {
                for entity in path_markers.iter() {
                    commands.entity(entity).despawn();
                }
                // Resume automatic behavior
                harvester.state = HarvesterState::Idle;
            }
        }
    }
}

fn move_harvester(time: Res<Time>, mut harvester_query: Query<(&mut Harvester, &mut Transform)>) {
    let Ok((mut harvester, mut transform)) = harvester_query.single_mut() else {
        return;
    };

    let current_pos = transform.translation.truncate();
    let direction = harvester.target_position - current_pos;
    let distance = direction.length();
    let move_distance = HARVESTER_SPEED * time.delta_secs();

    if distance <= move_distance {
        transform.translation = harvester.target_position.extend(transform.translation.z);

        // Get next waypoint
        if let Some(next) = harvester.movement_path.pop_front() {
            harvester.target_position = next;
        }
    } else if distance > 0.1 {
        let movement = direction.normalize() * move_distance;
        transform.translation += movement.extend(0.0);
    }
}

fn update_ui(
    harvester_query: Query<&Harvester>,
    money: Res<PlayerMoney>,
    mut text_query: Query<&mut Text, With<InfoText>>,
) {
    let Ok(harvester) = harvester_query.single() else {
        return;
    };
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    text.0 = format!(
        "Harvester Demo\n\
         Right-click to manually send harvester\n\
         \n\
         State: {}\n\
         Gold carried: {}/{}\n\
         Path waypoints: {}\n\
         \n\
         Total Money: ${}",
        harvester.state.name(),
        harvester.gold_capacity,
        harvester.max_capacity,
        harvester.movement_path.len(),
        money.0
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

// BFS to find nearest target from a set
fn find_nearest_target(
    grid_size: (usize, usize),
    start: TileCoord,
    targets: &HashSet<TileCoord>,
    blocking: &HashSet<TileCoord>,
) -> Option<Vec<TileCoord>> {
    use std::collections::VecDeque;

    let (width, height) = grid_size;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut came_from = std::collections::HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if targets.contains(&current) {
            // Reconstruct path
            let mut path = vec![current];
            let mut node = current;
            while let Some(&prev) = came_from.get(&node) {
                path.push(prev);
                node = prev;
            }
            path.reverse();
            return Some(path);
        }

        let neighbors = [
            (current.0.wrapping_sub(1), current.1),
            (current.0 + 1, current.1),
            (current.0, current.1.wrapping_sub(1)),
            (current.0, current.1 + 1),
        ];

        for next in neighbors {
            if next.0 < width
                && next.1 < height
                && !visited.contains(&next)
                && !blocking.contains(&next)
            {
                visited.insert(next);
                came_from.insert(next, current);
                queue.push_back(next);
            }
        }
    }

    None
}

// A* for manual path
fn calculate_path(
    grid_size: (usize, usize),
    start: TileCoord,
    goal: TileCoord,
    blocking_cells: &HashSet<TileCoord>,
) -> Vec<TileCoord> {
    let (grid_width, grid_height) = grid_size;
    let mut graph = Graph::<TileCoord, ()>::new();
    let mut node_indices = vec![vec![None; grid_width]; grid_height];

    for y in 0..grid_height {
        for x in 0..grid_width {
            if !blocking_cells.contains(&(x, y)) || (x, y) == start || (x, y) == goal {
                let node = graph.add_node((x, y));
                node_indices[y][x] = Some(node);
            }
        }
    }

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

