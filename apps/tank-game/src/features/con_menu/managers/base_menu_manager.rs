use crate::features::con_menu::components::submenu_info::SubMenuType;
use crate::features::con_menu::managers::menu_manager::MenuManager;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Res;

pub struct BaseMenuManager;

impl BaseMenuManager {
    pub fn new() -> Self {
        BaseMenuManager
    }

    pub fn show_menu(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        let items: Vec<(f32, String)> = vec![
            (1000.0, "sprites/building_b_tr.png".to_string()),
            (500.0, "sprites/building_c_tr.png".to_string()),
            (300.0, "sprites/building_d_tr.png".to_string()),
        ];

        MenuManager::show_menu_grid(parent, asset_server, SubMenuType::Base, items);
    }
}
