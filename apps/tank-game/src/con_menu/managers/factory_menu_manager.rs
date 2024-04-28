use crate::con_menu::components::submenu_info::SubMenuType;
use crate::con_menu::managers::menu_manager::MenuManager;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Res;

pub struct FactoryMenuManager;

impl FactoryMenuManager {
    pub fn new() -> Self {
        FactoryMenuManager
    }

    pub fn show_menu(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        let items: Vec<(f32, String)> = vec![
            (100.0, "sprites/tank_b_tr.png".to_string()),
            (200.0, "sprites/tank_c_tr.png".to_string()),
            (300.0, "sprites/tank_d_tr.png".to_string()),
        ];

        MenuManager::show_menu_grid(parent, asset_server, SubMenuType::Factory, items);
    }
}
