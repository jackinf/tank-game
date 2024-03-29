use crate::components::{Tank, TankGun, TankId, TankTargetPosition, TilePosition};
use crate::constants::{OFFSET_X, OFFSET_Y, TILE_SIZE, TILE_TANK};
use crate::resources::TankIdCounter;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Camera2dBundle, Commands, Res, ResMut};
use bevy_rapier2d::na::Quaternion;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tank_id_counter: ResMut<TankIdCounter>,
) {
    commands.spawn(Camera2dBundle::default());

    // 0 - empty, 1 - tank
    let tilemap = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
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

                    spawn_grass(&mut commands, &asset_server, pos);

                    if cell == TILE_TANK {
                        println!("tank pos: {:?}", pos);
                        spawn_tank(&mut commands, &asset_server, pos, &mut tank_id_counter);
                    }
                });
        });
}

fn spawn_grass(commands: &mut Commands, asset_server: &Res<AssetServer>, translation: Vec2) {
    let center_position = Vec2::new(translation.x, translation.y);
    commands
        .spawn((SpriteBundle {
            transform: Transform::default().with_translation(translation.extend(0.0)),
            texture: asset_server.load("grass3.png"),
            ..default()
        },))
        .insert(TilePosition::new(center_position, TILE_SIZE, TILE_SIZE));
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
            transform: Transform::default().with_translation(translation.extend(1.0)),
            texture: asset_server.load("tank3base.png"),
            ..default()
        },))
        .insert(TankTargetPosition::new(center_position, 0.0))
        .insert(Tank::new(tank_id))
        .id();

    commands
        .spawn((SpriteBundle {
            transform: Transform::default().with_rotation(Quat::from(Quaternion::identity())), // TODO: add rotation
            texture: asset_server.load("tank3gun.png"),
            ..default()
        },))
        .insert(TankGun {
            parent_id: TankId(tank_id),
        })
        .set_parent(tank_base);
}
