use crate::features::con_menu::actions::show_menu_grid::show_menu_grid;
use crate::features::con_menu::types::{ConMenuBuildingType, ConMenuType, ConMenuVehicleType};
use crate::features::con_menu::SubMenuType;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Res;

pub fn show_factory_menu_grid(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let items: Vec<Box<dyn ConMenuType>> = vec![
        Box::new(ConMenuVehicleType::TankA),
        Box::new(ConMenuVehicleType::TankB),
        Box::new(ConMenuVehicleType::TankC),
    ];

    show_menu_grid(parent, asset_server, SubMenuType::Factory, items);
}
