use crate::features::building::components::BuildingPlacementTiles;
use crate::features::building::types::BuildingTileType;
use crate::features::con_menu::resources::BuildingConstructionProgressInfo;
use crate::features::con_menu::MenuInfo;
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use bevy::prelude::{Query, Res, ResMut, Time};

pub fn building_process(
    mut time: Res<Time>,
    mut q_building_progress_info: Query<&mut BuildingConstructionProgressInfo>,
    mut q_menu_info: Query<&mut MenuInfo>,
) {
    let mut me = q_menu_info.single_mut();
    q_building_progress_info.iter_mut().for_each(|mut info| {
        if info.is_idle() {
            return;
        }

        if info.is_building() && me.has_enough_money(info.get_price_per_tick()) {
            if info.tick(time.delta()) {
                me.subtract_money(info.get_price_per_tick());
            }

            if info.is_placing() {
                println!("Building is ready!");

                // todo: play voice that the building is ready
            }
            return;
        }

        // todo: update sprite to increase square vertically by % ready
    });
}
