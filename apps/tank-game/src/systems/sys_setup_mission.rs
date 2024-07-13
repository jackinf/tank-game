use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res, ResMut};
use iyes_perf_ui::PerfUiCompleteBundle;

use crate::features::building::actions::spawn_buildings::spawn_buildings;
use crate::features::tile::spawn_tiles;
use crate::features::unit::{spawn_units, UnitIdCounter};
use crate::resources::mission_info_resource::MissionInfoResource;
use crate::SimpleState;

pub fn sys_setup_mission(
    mission_info_resource: Res<MissionInfoResource>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut unit_id_counter: ResMut<UnitIdCounter>,
    mut simple_state: ResMut<SimpleState>,
) {
    println!("once 2");
    if mission_info_resource.is_empty() {
        return;
    }

    if simple_state.ready {
        return;
    }

    /*
       Prepare Ground layer
    */

    let ground_layer = mission_info_resource.get_ground_layer();
    spawn_tiles(&mut commands, &asset_server, &ground_layer);

    /*
       Prepare Resource layer
    */

    let resource_layer = mission_info_resource.get_resource_layer();
    spawn_tiles(&mut commands, &asset_server, &resource_layer);

    /*
       Prepare Units layer
    */

    let units_layer = mission_info_resource.get_units_layer();
    spawn_units(
        &mut commands,
        &asset_server,
        &mut unit_id_counter,
        units_layer,
    );

    /*
       Prepare Buildings layer
    */

    let buildings_layer = mission_info_resource.get_buildings_layer();
    spawn_buildings(
        &mut commands,
        &asset_server,
        buildings_layer,
        &mut unit_id_counter,
    );

    commands.spawn(PerfUiCompleteBundle::default());
    simple_state.ready = true;
}
