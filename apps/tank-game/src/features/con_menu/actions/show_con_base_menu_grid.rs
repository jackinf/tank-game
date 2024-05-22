use crate::features::con_menu::actions::show_menu_grid::show_menu_grid;
use crate::features::con_menu::SubMenuType;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Res;

pub fn show_con_base_menu_grid(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let items: Vec<(f32, String)> = vec![
        (1000.0, "sprites/building_b_tr.png".to_string()),
        (500.0, "sprites/building_c_tr.png".to_string()),
        (300.0, "sprites/building_d_tr.png".to_string()),
    ];

    show_menu_grid(parent, asset_server, SubMenuType::Base, items);
}
