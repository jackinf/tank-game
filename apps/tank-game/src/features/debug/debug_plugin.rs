use bevy::app::Plugin;
use bevy::prelude::*;

use crate::features::building::components::building_placement_tiles::BuildingPlacementTiles;
use crate::features::building::types::building_tile_type::BuildingTileType;
use crate::features::con_menu::MenuInfo;
use crate::features::cursor::CursorCoordinates;
use crate::features::debug::resources::tank_log_timer::TankLogTimer;
use crate::features::tank::components::tank::Tank;
use crate::types::main_asset_info_resource::MainAssetInfoResource;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankLogTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, inflate_all_tanks)
            .add_systems(Update, buying_stuff)
            .add_systems(Update, damage_selected_tanks)
            .add_systems(Update, construction_complete)
            .add_systems(Update, logger);
    }
}

fn logger(mut timer: ResMut<TankLogTimer>, time: Res<Time>, q_coords: Res<CursorCoordinates>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!(
            "Cursor coordinates: {:?}. Tile coordinates: {:?}",
            q_coords.get_world(),
            q_coords.get_tile()
        );
    }
}

fn construction_complete(
    main_asset_info_resource: Res<MainAssetInfoResource>,
    mut q_placement_building: Query<&mut BuildingPlacementTiles>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyM) {
        return;
    }

    let mut placement_building = q_placement_building.single_mut();
    let building_tiles = main_asset_info_resource.get_building_tiles();
    let building_tile = building_tiles.get(&BuildingTileType::Base);
    if building_tile.is_none() {
        return;
    }
    let building_tile = building_tile.unwrap().clone();
    placement_building.set_ready(Some(building_tile));
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
