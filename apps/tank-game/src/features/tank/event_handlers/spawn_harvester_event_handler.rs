use crate::features::harvester::spawn_harvester;
use crate::features::tank::events::SpawnHarvesterEvent;
use crate::features::unit::{UnitIdCounter, UnitTileType};
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use bevy::prelude::{AssetServer, Commands, EventReader, Res, ResMut, Vec2};

pub fn spawn_harvester_event_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut harvester_id_counter: ResMut<UnitIdCounter>,
    mut spawn_harvester_event_reader: EventReader<SpawnHarvesterEvent>,
    main_asset_info_resource: Res<MainAssetInfoResource>,
) {
    for spawn_harvester_event in spawn_harvester_event_reader.read() {
        if let Some(harvester_info) = main_asset_info_resource
            .get_unit_tiles()
            .get(&UnitTileType::Harvester)
        {
            spawn_harvester(
                &mut commands,
                &asset_server,
                harvester_info.get_image_path().clone(),
                spawn_harvester_event.position,
                &mut harvester_id_counter,
                Some(spawn_harvester_event.player.clone()),
            );
        }
    }
}
