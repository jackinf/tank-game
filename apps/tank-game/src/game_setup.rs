use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Camera2dBundle, Commands, Res, ResMut};
use bevy_rapier2d::na::Quaternion;

use crate::common::constants::{OFFSET_X, OFFSET_Y, TILE_SIZE, TILE_TANK, TILE_WALL, TILE_GRASS};
use crate::common::resources::TankIdCounter;
use crate::common::tile::Tile;
use crate::tank::tank::Tank;
use crate::tank::tank_gun::TankGun;
use crate::tank::tank_id::TankId;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tank_id_counter: ResMut<TankIdCounter>,
) {
    commands.spawn(Camera2dBundle::default());

    // 0 - empty, 1 - tank, 2 - wall
    let tilemap = vec![
        vec![0, 0, 1, 0, 2, 0, 0, 1],
        vec![0, 0, 1, 0, 2, 0, 0, 1],
        vec![0, 0, 1, 0, 2, 0, 0, 1],
        vec![0, 0, 1, 0, 2, 0, 0, 1],
        vec![1, 0, 0, 0, 2, 0, 0, 1],
        vec![1, 0, 0, 0, 2, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
    ];

    // let tilemap_small = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 1, 0]];

    // draw_tiles(&mut commands, &asset_server, tilemap);
    tilemap
        .into_iter()
        .enumerate()
        .for_each(|(col_index, row_on_row)| {
            row_on_row
                .into_iter()
                .enumerate()
                .for_each(|(row_index, cell)| {
                    let x = row_index as f32 * TILE_SIZE + OFFSET_X;
                    let y = col_index as f32 * TILE_SIZE + OFFSET_Y;
                    let pos = Vec2::new(x, y);

                    match cell {
                        TILE_WALL => spawn_simple_tile(&mut commands, &asset_server, pos, TILE_WALL),
                        TILE_TANK => {
                            spawn_simple_tile(&mut commands, &asset_server, pos, TILE_GRASS);
                            spawn_tank(&mut commands, &asset_server, pos, &mut tank_id_counter);
                        },
                        TILE_GRASS => spawn_simple_tile(&mut commands, &asset_server, pos, TILE_GRASS),
                        _ => spawn_simple_tile(&mut commands, &asset_server, pos, TILE_GRASS),
                    }
                });
        });
}

fn spawn_simple_tile(commands: &mut Commands, asset_server: &Res<AssetServer>, translation: Vec2, tile_type: usize) {
    let center_position = Vec2::new(translation.x, translation.y);
    let path: String = if tile_type == TILE_WALL {
        "wall.png".into()
    } else {
        "grass3.png".into()
    };
    let layer: f32 = if tile_type == TILE_WALL { 10.0 } else { 0.0 };

    commands
        .spawn((SpriteBundle {
            transform: Transform::default().with_translation(translation.extend(layer)),
            texture: asset_server.load(path),
            ..default()
        },))
        .insert(Tile::new(center_position, TILE_SIZE, TILE_SIZE, tile_type));
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
            transform: Transform::default().with_translation(translation.extend(5.0)),
            texture: asset_server.load("tank3base.png"),
            ..default()
        },))
        .insert(Tank::new(tank_id, center_position))
        .id();

    commands
        .spawn((SpriteBundle {
            transform: Transform::default().with_rotation(Quat::from(Quaternion::identity())), // TODO: add rotation
            texture: asset_server.load("tank3gun.png"),
            ..default()
        },))
        .insert(TankGun::new(TankId(tank_id)))
        .set_parent(tank_base);
}
