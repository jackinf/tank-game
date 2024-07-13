use crate::actions::load_mission::load_mission;
use crate::actions::read_main_assets::read_main_assets;
use crate::resources::mission_info_resource::MissionInfoResource;
use crate::{AppState, SimpleState, SimpleText};
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::{NextState, Res, ResMut};

use crate::types::main_asset_info::MainAssetInfo;
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use crate::types::mission_info::MissionInfo;

pub fn sys_setup_main_assets(
    mut main_asset_info_resource: ResMut<MainAssetInfoResource>,
    mut mission_info_resource: ResMut<MissionInfoResource>,
    mut state: ResMut<NextState<AppState>>,
    mut simple_state: ResMut<SimpleState>,
    custom_assets: Res<Assets<SimpleText>>,
) {
    if custom_assets.get(&simple_state.simple_text).is_none() {
        return;
    }

    println!("once");
    let custom_asset = custom_assets.get(&simple_state.simple_text).unwrap();
    let content = custom_asset.content.clone();

    let assets_result = read_main_assets(content);
    if let Err(err) = assets_result {
        panic!("Failed to read assets: {}", err);
    }
    let assets: MainAssetInfo = assets_result.unwrap();
    main_asset_info_resource.initialize(assets.get_tiles().clone());

    let custom_asset = custom_assets.get(&simple_state.simple_text2).unwrap();
    let content = custom_asset.content.clone();

    let mission_info = load_mission(&assets.get_tiles(), content);
    if let Err(err) = mission_info {
        panic!("Failed to load mission: {:?}", err);
    }
    let mission_info: MissionInfo = mission_info.unwrap();
    mission_info_resource.initialize(mission_info);

    state.set(AppState::PreparingUsingDynamicAssets);
}
