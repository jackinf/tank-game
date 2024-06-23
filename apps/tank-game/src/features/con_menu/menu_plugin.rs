use bevy::prelude::*;

use crate::features::con_menu::resources::{BuildingProgressInfo, GlobalConInfo};
use crate::features::con_menu::states::ConState;
use crate::features::con_menu::systems::{
    building_process, detect_mouse_over_container, interact_with_menu, setup,
    toggle_menu_visibility, update_money_text, update_power_text,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Update, detect_mouse_over_container)
            .add_systems(Update, update_money_text)
            .add_systems(Update, update_power_text)
            .add_systems(Update, toggle_menu_visibility)
            .add_systems(Update, interact_with_menu)
            .add_systems(Update, building_process);
        // .add_systems(Update, interact_with_menu.run_if(in_state(ConState::Idle)))
        // .add_systems(Update, building_process.run_if(in_state(ConState::Building)))
        // .init_state::<ConState>()
        // .insert_resource(GlobalConInfo::new());
        // .insert_resource(BuildingTimer);
    }
}
