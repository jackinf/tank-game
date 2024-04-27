use crate::building::building_type::BuildingType;
use crate::building::components::building_placement_tiles::BuildingPlacementTiles;
use crate::cursor::resources::cursor_coordinates::CursorCoordinates;
use crate::debug::resources::tank_log_timer::TankLogTimer;
use bevy::app::Plugin;
use bevy::prelude::*;

use crate::con_menu::resources::menu_info::MenuInfo;
use crate::tank::components::tank::Tank;
use crate::tile::components::tile::Tile;
use crate::tile::tile_queries::TileQueries;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankLogTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, inflate_all_tanks)
            .add_systems(Update, buying_stuff)
            .add_systems(Update, damage_selected_tanks)
            .add_systems(Update, construction_complete);
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
    mut q_placement_building: Query<&mut BuildingPlacementTiles>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyM) {
        return;
    }

    let mut placement_building = q_placement_building.single_mut();
    placement_building.set_ready(Some(BuildingType::Base));
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

fn buying_stuff(mut q_menu_info: Query<&mut MenuInfo>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        let mut menu_info = q_menu_info.single_mut();
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
