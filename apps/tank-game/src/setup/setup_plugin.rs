use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res, ResMut};
use bevy_rapier2d::na::Quaternion;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::common::constants::{
    OFFSET_X, OFFSET_Y, SPRITE_SCALE, TILE_GRASS, TILE_SIZE, TILE_TANK, TILE_WALL, TILE_WATER,
};
use crate::common::game_map::GameMap;
use crate::common::tile::Tile;
use crate::setup::tank_id_counter::TankIdCounter;
use crate::tank::tank::Tank;
use crate::tank::tank_gun::TankGun;
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
    // 0 - empty, 1 - tank, 2 - wall, 3 - water
    let map_file = File::open("apps/tank-game/assets/map1.txt").unwrap();
    let reader = BufReader::new(map_file);

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
    game_map.0 = tilemap.clone();

    let mut grid_to_tilemap = HashMap::new();

    tilemap
        .into_iter()
        .enumerate()
        .for_each(|(row_index, row_on_row)| {
            row_on_row
                .into_iter()
                .enumerate()
                .for_each(|(col_index, cell)| {
                    let x = (row_index) as f32 * TILE_SIZE + OFFSET_X;
                    let y = (col_index) as f32 * TILE_SIZE + OFFSET_Y;
                    let pos = Vec2::new(x, y);
                    let map_coord = (row_index, col_index);

                    match cell {
                        TILE_WALL => spawn_simple_tile(
                            &mut commands,
                            &asset_server,
                            pos,
                            TILE_WALL,
                            map_coord,
                            &mut grid_to_tilemap,
                        ),
                        TILE_TANK => {
                            spawn_simple_tile(
                                &mut commands,
                                &asset_server,
                                pos,
                                TILE_GRASS,
                                map_coord,
                                &mut grid_to_tilemap,
                            );
                            spawn_tank(&mut commands, &asset_server, pos, &mut tank_id_counter);
                        }
                        TILE_GRASS => spawn_simple_tile(
                            &mut commands,
                            &asset_server,
                            pos,
                            TILE_GRASS,
                            map_coord,
                            &mut grid_to_tilemap,
                        ),
                        TILE_WATER => spawn_simple_tile(
                            &mut commands,
                            &asset_server,
                            pos,
                            TILE_WATER,
                            map_coord,
                            &mut grid_to_tilemap,
                        ),
                        _ => spawn_simple_tile(
                            &mut commands,
                            &asset_server,
                            pos,
                            TILE_GRASS,
                            map_coord,
                            &mut grid_to_tilemap,
                        ),
                    }
                });
        });

    game_map.1 = grid_to_tilemap;
}

fn spawn_simple_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec2,
    tile_type: usize,
    map_coord: (usize, usize),
    grid_to_tilemap: &mut HashMap<(usize, usize), (f32, f32)>,
) {
    let center_position = Vec2::new(translation.x, translation.y);
    let path: String = if tile_type == TILE_WALL {
        "wall.png".into()
    } else if tile_type == TILE_WATER {
        "water.png".into()
    } else {
        "grass3.png".into()
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
        .insert(tile);

    grid_to_tilemap.insert(map_coord, (translation.x, translation.y));
}

fn spawn_tank(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec2,
    tank_id_counter: &mut ResMut<TankIdCounter>,
) {
    let tank_id = tank_id_counter.0;
    tank_id_counter.0 += 1;

    let center_position = Vec2::new(
        translation.x - (TILE_SIZE / 2.0),
        translation.y - (TILE_SIZE / 2.0),
    );
    let tank_base: Entity = commands
        .spawn((SpriteBundle {
            transform: Transform::default()
                .with_translation(translation.extend(5.0))
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            texture: asset_server.load("tank3base.png"),
            ..default()
        },))
        .insert(Tank::new(tank_id, translation))
        .id();

    commands
        .spawn((SpriteBundle {
            transform: Transform::default()
                .with_rotation(Quat::from(Quaternion::identity()))
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            texture: asset_server.load("tank3gun.png"),
            ..default()
        },))
        .insert(TankGun::new(TankId(tank_id)))
        .set_parent(tank_base);
}