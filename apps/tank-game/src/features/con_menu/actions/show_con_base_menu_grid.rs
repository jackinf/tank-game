use crate::features::con_menu::actions::show_menu_grid::show_menu_grid;
use crate::features::con_menu::types::{ConMenuBuildingType, ConMenuType};
use crate::features::con_menu::SubMenuType;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Res;

pub fn show_con_base_menu_grid(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let items: Vec<Box<dyn ConMenuType>> = vec![
        Box::new(ConMenuBuildingType::Base),
        Box::new(ConMenuBuildingType::Factory),
        Box::new(ConMenuBuildingType::PowerPlant),
    ];

    show_menu_grid(parent, asset_server, SubMenuType::Base, items);
}
