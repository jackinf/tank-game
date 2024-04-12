use crate::common::tile::Tile;
use crate::common::tile_queries::TileQueries;
use crate::cursor::cursor_coordinates::CursorCoordinates;
use bevy::app::Plugin;
use bevy::prelude::*;

use crate::debug::tank_log_timer::TankLogTimer;
use crate::menu::menu_info::MenuInfo;
use crate::menu::menu_plugin::ConstructionInfo;
use crate::tank::tank::Tank;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankLogTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, inflate_all_tanks)
            .add_systems(Update, buying_stuff)
            .add_systems(Update, damage_selected_tanks)
            .add_systems(Update, construction_complete)
            .add_systems(Update, log_construction_info)
            .add_systems(Update, logger);
    }
}

fn logger(
    mut timer: ResMut<TankLogTimer>,
    time: Res<Time>,
    q_coords: Res<CursorCoordinates>,
    q_tiles: Query<&Tile>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let tile_coordinates =
            TileQueries::find_accessible(&q_tiles, &q_coords.0).unwrap_or((999, 999));
        println!(
            "Cursor coordinates: {:?}. Tile coordinates: {:?}",
            q_coords.0, tile_coordinates
        );
    }
}

fn construction_complete(
    mut construction_info: ResMut<ConstructionInfo>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyM) {
        return;
    }

    construction_info.set_ready(true);
}

fn log_construction_info(
    mut timer: ResMut<TankLogTimer>,
    time: Res<Time>,
    construction_info: Res<ConstructionInfo>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    if construction_info.is_ready() {
        println!("Construction is ready!");
    }
}

fn inflate_all_tanks(
    mut query: Query<&mut Transform, With<Tank>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        for mut transform in &mut query {
            transform.scale *= 1.25;
        }
    }
}

fn buying_stuff(mut menu_info: ResMut<MenuInfo>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        menu_info.add_money(-100);
        println!("Buying stuff, money left: {}", menu_info.get_money());
    }
}

fn damage_selected_tanks(mut query: Query<&mut Tank>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        for mut tank in &mut query.iter_mut().filter(|tank| tank.selected) {
            tank.take_damage(30);
            println!("Tank health: {}", tank.health);
        }
    }
}
