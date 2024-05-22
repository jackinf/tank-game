use crate::features::con_menu::actions::show_menu_grid::show_menu_grid;
use crate::features::con_menu::SubMenuType;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Res;

pub fn show_factory_menu_grid(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let items: Vec<(f32, String)> = vec![
        (100.0, "sprites/tank_b_tr.png".to_string()),
        (200.0, "sprites/tank_c_tr.png".to_string()),
        (300.0, "sprites/tank_d_tr.png".to_string()),
    ];

    show_menu_grid(parent, asset_server, SubMenuType::Factory, items);
}
