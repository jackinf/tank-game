use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res, ResMut};
use bevy::sprite::Anchor;
use bevy_rapier2d::na::Quaternion;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::common::constants::{
    OFFSET_X, OFFSET_Y, SPRITE_SCALE, TANK_FULL_HEALTH_BAR_WIDTH, TANK_HEALTH_BAR_HEIGHT,
    TANK_HEALTH_BAR_SIZE, TANK_MAX_HEALTH, TILE_GRASS, TILE_SIZE, TILE_TANK, TILE_WALL, TILE_WATER,
};
use crate::common::game_map::GameMap;
use crate::common::tile::Tile;
use crate::setup::tank_id_counter::TankIdCounter;
use crate::tank::tank::Tank;
use crate::tank::tank_gun::TankGun;
use crate::tank::tank_health::{HealthBar, TankHealth};
use crate::tank::tank_id::TankId;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankIdCounter(1))
            .add_systems(PreStartup, setup);
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tank_id_counter: ResMut<TankIdCounter>,
    mut game_map: ResMut<GameMap>,
) {
    // read file "map1.txt" into a 2d array
    let tilemap = read_map_from_file();

    let mut grid = vec![];
    let mut grid_to_tilemap = HashMap::new();

    tilemap
        .into_iter()
        .enumerate()
        .for_each(|(row_index, row_on_row)| {
            let mut row = vec![];
            row_on_row
                .into_iter()
                .enumerate()
                .for_each(|(col_index, cell)| {
                    let x = row_index as f32 * TILE_SIZE + OFFSET_X;
                    let y = col_index as f32 * TILE_SIZE + OFFSET_Y;
                    let pos = Vec2::new(x, y);
                    let map_coord = (row_index, col_index);
                    grid_to_tilemap.insert(map_coord, (x, y));

                    // let tank stand on a grass by default
                    let cell2 = if cell == TILE_TANK { TILE_GRASS } else { cell };
                    let tile = spawn_tile(&mut commands, &asset_server, pos, cell2, map_coord);
                    row.push(tile);

                    if TILE_TANK == cell {
                        spawn_tank(&mut commands, &asset_server, pos, &mut tank_id_counter);
                    }
                });
            grid.push(row);
        });

    game_map.set_map(grid, grid_to_tilemap);
}

fn read_map_from_file() -> Vec<Vec<usize>> {
    let map_file = File::open("apps/tank-game/assets/map0.txt").unwrap();
    let reader = BufReader::new(map_file);

    // 0 - empty, 1 - tank, 2 - wall, 3 - water
    let mut tilemap: Vec<Vec<usize>> = vec![];
    for line_result in reader.lines() {
        if let Err(_) = line_result {
            continue;
        }
        let line = line_result.unwrap();
        if line.is_empty() {
            continue;
        }

        let cells: Vec<usize> = line
            .split(' ')
            .map(|letter| letter.parse::<usize>().unwrap())
            .collect();
        tilemap.push(cells.clone());
    }
    tilemap
}

fn spawn_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec2,
    tile_type: usize,
    map_coord: (usize, usize),
) -> Tile {
    assert_eq!(
        tile_type == TILE_WALL || tile_type == TILE_WATER || tile_type == TILE_GRASS,
        true
    );

    let center_position = Vec2::new(translation.x, translation.y);
    let path: String = if tile_type == TILE_WALL {
        "sprites/tiles/wall.png".into()
    } else if tile_type == TILE_WATER {
        "sprites/tiles/water.png".into()
    } else if tile_type == TILE_GRASS {
        "sprites/tiles/grass3.png".into()
    } else {
        panic!("Invalid tile type: {}", tile_type)
    };

    let layer: f32 = if tile_type == TILE_WALL { 10.0 } else { 0.0 };

    let tile = Tile::new(center_position, TILE_SIZE, TILE_SIZE, tile_type, map_coord);
    commands
        .spawn((SpriteBundle {
            transform: Transform::default()
                .with_translation(translation.extend(layer))
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            texture: asset_server.load(path),
            ..default()
        },))
        .insert(tile.clone());

    tile
}

fn spawn_tank(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec2,
    tank_id_counter: &mut ResMut<TankIdCounter>,
) {
    let tank_id = tank_id_counter.0;
    tank_id_counter.0 += 1;

    let tank_texture = asset_server.load("sprites/tank3base.png");
    let gun_texture = asset_server.load("sprites/tank3gun.png");
    // let health_bar_texture = asset_server.load("pixels/white.png");

    // generate a random number between 5.0 and 6.0 with 4 decimal places
    let layer = (5.0 + (rand::random::<f32>() * 1.0)).round() * 10000.0 / 10000.0;

    let tank_base: Entity = commands
        .spawn((SpriteBundle {
            transform: Transform::default()
                .with_translation(translation.extend(layer))
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            texture: tank_texture,
            ..default()
        },))
        .insert(Tank::new(tank_id, translation))
        .insert(TankHealth::new(TANK_MAX_HEALTH as f32))
        .id();

    // Spawn the tank gun as a child of the tank base
    commands.entity(tank_base).with_children(|parent| {
        // Spawn the turret as a child of the tank
        parent
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::splat(SPRITE_SCALE)), // Ensure it's positioned correctly relative to the base
                texture: gun_texture,
                ..default()
            })
            .insert(TankGun::new(TankId(tank_id)));

        // Spawn the health bar as a child of the tank
        parent
            .spawn(SpriteBundle {
                // Position the health bar above the tank
                transform: Transform::from_xyz(-50.0, 40.0, 0.2),
                sprite: Sprite {
                    color: Color::PURPLE, // Health bar color
                    rect: Some(Rect {
                        min: Vec2::new(0.0, 0.0),
                        max: TANK_HEALTH_BAR_SIZE,
                    }),
                    anchor: Anchor::CenterLeft, // Anchor the health bar to the left of the tank
                    ..default()
                },
                ..default()
            })
            .insert(HealthBar);
    });
}
