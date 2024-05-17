use crate::features::preparation::actions::load_mission::load_mission;
use crate::features::preparation::actions::read_main_assets::read_main_assets;
use crate::features::preparation::resources::mission_info_resource::MissionInfoResource;
use bevy::prelude::ResMut;

use crate::features::preparation::types::main_asset_info::MainAssetInfo;
use crate::features::preparation::types::main_asset_info_resource::MainAssetInfoResource;
use crate::features::preparation::types::mission_info::MissionInfo;

pub fn setup1(
    mut main_asset_info_resource: ResMut<MainAssetInfoResource>,
    mut mission_info_resource: ResMut<MissionInfoResource>,
) {
    let assets_result = read_main_assets("apps/tank-game/assets/main_assets.tsj");
    if let Err(err) = assets_result {
        panic!("Failed to read assets: {}", err);
    }
    let assets: MainAssetInfo = assets_result.unwrap();
    main_asset_info_resource.initialize(assets.get_tiles().clone());

    let mission_info = load_mission(
        &main_asset_info_resource,
        "apps/tank-game/assets/mission01.tmj",
    );
    if let Err(err) = mission_info {
        panic!("Failed to load mission: {:?}", err);
    }
    let mission_info: MissionInfo = mission_info.unwrap();
    mission_info_resource.initialize(mission_info);
}
